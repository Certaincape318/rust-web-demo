use models::*;
use repository::task as repos;
use cache;
const CACHE_KEY: &'static str="task_list";

pub fn list() -> Vec<Task> {
    let cached_list=cache::get(CACHE_KEY);
    if let Ok(list)=cached_list {
        println!("get from redis cache");
        return list;
    }else{
        let _=cache::del(CACHE_KEY);
    }
    cache::set("task_list",repos::list()).unwrap()
}

pub fn get(id:i32) -> Option<Task> {
    repos::get(id)
}

pub fn delete(id:i32){
    repos::delete(id);
    let _=cache::del(CACHE_KEY);
}
pub fn save(task:&Task){
    repos::save(task);
    let _=cache::del(CACHE_KEY);
}
