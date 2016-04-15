use chrono::*;
#[derive(Default,Debug)]
pub struct Account {
    pub id          : i32,
    pub name        : Option<String>,
    pub password    : Option<String>,
}

#[derive(Default,Debug,RustcEncodable, RustcDecodable,Clone, PartialEq,Eq)]
pub struct Task {
    pub id              : i32,
    pub name            : Option<String>,
    pub content         : Option<String>,
    pub create_time     : Option<DateTime<UTC>>,
    pub update_time     : Option<DateTime<UTC>>,
    pub status          : i32,//0:new,1:ongoing,2:finished,3:canceld
}

