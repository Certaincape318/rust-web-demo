use std::sync::{Once, ONCE_INIT};
use std::str::FromStr;
use std::collections::BTreeMap;
use rustc_serialize::json::{self,Json, ToJson};
use persistent_time::Time;
use iron::prelude::*;
use iron::status;
use params::*;
use router::Router;
use utils::crypto;
use services;
use models::*;

impl ToJson for Task {
    fn to_json(&self) -> Json {
        return Json::from_str(&json::encode(&self).unwrap()).unwrap();
    }
}
fn get_path_param(req: &mut Request, name: &str) -> Option<String> {
    let ref value = req.extensions.get::<Router>().unwrap().find(name).unwrap_or("");
    if value.len() > 0 {
        return Some(String::from(*value));
    }
    None
}

static START: Once = ONCE_INIT;
pub fn init_router(router: &mut Router) {
    START.call_once(|| {
        router.get("/task/",|_: &mut Request|{
            let tasks=services::task::list();
            let mut data = BTreeMap::new();
            data.insert("tasks".to_string(), tasks.to_json());
            super::template("task/list",&data)
        });

        router.get("/task/json/", |_:&mut Request|{
            super::ok_json(&format!("{}",services::task::list().to_json()))
        });

        router.get("/task/json/aes/",|_:&mut Request|{
            let data=crypto::aes_encrypt_string(&format!("{}",services::task::list().to_json()));
            let data=crypto::base64_encode_bytes(&data.ok().unwrap());
            let data=data.expect("");
            super::ok_text(&data)
        });

        router.get("/task/json/base64/",|_:&mut Request|{
            let data=crypto::base64_encode_string(&format!("{}",services::task::list().to_json())).expect("");
            super::ok_text(&data)
        });

        router.get("/task/new",|_:&mut Request|super::template("task/new",&()));

        router.get("/task/:id",|req: &mut Request|{
            let id=get_path_param(req,"id").unwrap_or("0".to_owned());
            let id=i32::from_str(&*id).unwrap_or(0);
            let mut response = Response::new();
            response.set_mut(status::Ok);
            if id>0{
                let task=services::task::get(id);
                if let Some(task)=task {
                    let mut data = BTreeMap::new();
                    data.insert("task".to_string(), task.to_json());
                    response.set_mut(super::render("task/edit", &data).unwrap());
                }
            }
            Ok(response)
        });

        router.get("/task/delete/:id",|req: &mut Request| {
            let id=get_path_param(req,"id").unwrap_or("0".to_owned());
            let id=i32::from_str(&*id).unwrap_or(0);
            if id>0{
                services::task::delete(id);
            }
            super::redirect(req,"/task/")
        });

        router.post("/task/",|req: &mut Request|{
            let name=req.param::<String>("name");
            let content=req.param::<String>("content");
            let status=req.param::<i32>("status").unwrap_or(0);
            let time=Time::new();
            let id=req.param::<i32>("id").unwrap_or(0);
            let task=Task{
                id:             id,
                name:           name,
                content:        content,
                create_time:    Some(time),
                update_time:    Some(time),
                status:         status,
            };
            debug!("saving task:{:?}",&task);
            services::task::save(&task);
            super::redirect(req,"/task/")
        });

        //curl --data-urlencode "data=NTDlhYMzMDDmnaEs5aSW6ZO+5Luj5Y+RLOmUmuaWh+acrA==" "http://localhost:8080/api"
        router.post("/task/json-post",|req: &mut Request| {
            if let Some(s)=req.param::<String>("data"){
                if let Some(data)=crypto::base64_decode_to_string(&s) {
                    if let Ok(data)=Json::from_str(&data) {
                        if let Some(obj)=data.as_object() {
                            //let id=get_json_i64(&obj,"id");
                            let manufactor=get_json_string(&obj,"manufactor");
                            let id=get_json_i64(&obj,"id");
                            debug!("id:{}",id);
                            debug!("manufactor:{:?}",manufactor);
                        }
                    }
                }
            }
            Ok(Response::with(status::Ok))
        });
    });
}

fn get_json_string(obj: &BTreeMap<String, Json>, key: &str) -> Option<String> {
    obj.get(key).map(|json| json.as_string()).unwrap_or_else(|| None).map(|str| str.to_owned())
}

fn get_json_i64(obj: &BTreeMap<String, Json>, key: &str) -> i64 {
    obj.get(key).map(|json| json.as_i64()).unwrap_or_default().unwrap_or_default()
}
