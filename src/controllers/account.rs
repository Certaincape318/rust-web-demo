use super::prelude::*;
use services::account as service;
use std::sync::{Once, ONCE_INIT};
use models::*;
static START: Once = ONCE_INIT;
pub fn init_router(router: &mut Router) {
    START.call_once(|| {
        router.get("/account/login/",
                   |_: &mut Request| response::template("account/login", ()));

        router.post("/account/login/", |req: &mut Request| {
            let name = req.param::<String>("name");
            let password = req.param::<String>("password");
            if let Some(account) = service::get(name, password) {
                req.set_session("account", &account).unwrap();
                return response::redirect(req,"/");
            }
            let mut data = BTreeMap::new();
            data.insert("error".to_string(), true);
            response::template("account/login", data)
        });

        router.get("/account/logout/", |req: &mut Request| {
            req.clear_session().unwrap();
            let ref url = req.url;
            let url = Url::parse(format!("{}://{}:{}/account/login/",
                                         url.scheme(),
                                         url.host(),
                                         url.port())
                    .as_str())
                .unwrap();
            Ok(Response::with((status::Found, Redirect(url.clone()))))
        });
    });
}
pub fn check_login(req: &mut Request) -> bool {
    let account = req.get_session::<Account>("account");
    account.is_ok()
}
