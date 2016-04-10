use models::*;
use repository::task as repos;
use cache;
pub fn list() -> Vec<Task> {
    let cached_list=cache::get("task_list");
    if let Ok(list)=cached_list {
        println!("get from redis cache");
        return list;
    }
    let list= repos::list();
    let _=cache::set("task_list",list);
    cache::get("task_list").unwrap()
}

pub fn get(id:i32) -> Option<Task> {
    repos::get(id)
}

pub fn delete(id:i32){
    repos::delete(id);
}
pub fn save(task:&Task){
    repos::save(task);
}
