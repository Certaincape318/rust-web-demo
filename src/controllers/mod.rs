pub mod task;
pub mod account;

use std::sync::Arc;
use std::path::Path;
use iron::{AfterMiddleware, AroundMiddleware, Handler};
use iron::prelude::*;
use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::ContentType;
use iron::{Url, status};
use iron::modifiers::Redirect;
use staticfile::Static;
use template::TemplateEngine;
use router::{Router, NoRoute};
use rustc_serialize::json::ToJson;
use mount::Mount;
use params::*;
use session::*;


pub fn get_chain() -> Chain {
    let mut router = Router::new();
    router.get("/", |_: &mut Request| template("index", &()));
    self::task::init_router(&mut router);
    self::account::init_router(&mut router);

    let mut mount = Mount::new();
    mount.mount("/", router).mount("/static", Static::new(Path::new("./web-root/static/")));

    let mut chain = Chain::new(mount);
    chain.link_before(Params {});
    chain.link_after(ErrorHandler);
    chain.link_around(LoginChecker);
    chain.link_around(Session::new("key-123456", 3600, "redis://localhost"));
    chain
}

lazy_static! {
    static ref ENGINE:Arc<TemplateEngine>  = TemplateEngine::new("./web-root/templates/",".hbs");
}

pub fn render<T: ToJson + Sized>(name: &str, data: &T) -> Option<String> {
    ENGINE.render(name, data).ok()
}

pub fn redirect(req: &Request, path: &str) -> IronResult<Response> {
    let ref url = req.url;
    let url = Url::parse(format!("{}://{}:{}{}", url.scheme(), url.host(), url.port(), path).as_str()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))
}

pub fn template<T: ToJson>(name: &str, value: &T) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(render(name, value).unwrap());
    response.set_mut(status::Ok);
    response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![(Attr::Charset, Value::Utf8)])));
    Ok(response)
}

pub fn ok_json(data: &str) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok).set_mut(data);
    response.headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
    Ok(response)
}

pub fn ok_text(data: &str) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok).set_mut(data);
    response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![(Attr::Charset, Value::Utf8)])));
    Ok(response)
}


struct LoginChecker;

impl AroundMiddleware for LoginChecker {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        struct LoggerHandler<H: Handler> {
            handler: H,
        }
        impl<H: Handler> Handler for LoggerHandler<H> {
            fn handle(&self, req: &mut Request) -> IronResult<Response> {
                if self::account::check_login(req) || req.url.path().join("/").contains("account") {
                    let res = self.handler.handle(req);
                    return res;
                }
                let url = format!("{}://{}:{}/account/login/",
                                  req.url.scheme(),
                                  req.url.host(),
                                  req.url.port());
                let url = Url::parse(url.as_str()).unwrap();
                Ok(Response::with((status::Found, Redirect(url.clone()))))
            }
        }
        Box::new(LoggerHandler { handler: handler }) as Box<Handler>
    }
}

struct ErrorHandler;

impl AfterMiddleware for ErrorHandler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            error!("error handler catch some errors:{:?}", err);
            Err(err)
        }
    }
}
