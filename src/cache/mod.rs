use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};
use redis::Commands;
use redis::RedisResult;
use redis::ToRedisArgs;
use redis::FromRedisValue;
use redis::RedisError;
use redis::Value;
use std::io::Error;
use std::io::ErrorKind;
use rustc_serialize::{Encodable, Decodable};
use r2d2_redis::RedisConnectionManager;
use std::default::Default;
use r2d2::{Pool, PooledConnection};

pub fn set<T>(key:&str,value: T)->RedisResult<()> where T:Encodable{
    let conn=get_conn();
    let c:MyType<T>=MyType(value);
    let _ : () = try!(conn.set(key, c));
    Ok(())
}

pub fn get<T>(key:&str)->RedisResult<T> where T:Decodable{
    let conn=get_conn();
    let t:MyType<T>=try!(conn.get(key));
    Ok(t.0)
}

#[derive(Debug,RustcEncodable, RustcDecodable, PartialEq)]
    struct Task {
    id: i32,
    name: Option<String>,
}


struct MyType<T>(T);
impl<T> ToRedisArgs for MyType<T> where T:Encodable {
    fn to_redis_args(&self) -> Vec<Vec<u8>> {
        vec![encode(&self.0, SizeLimit::Infinite).unwrap()]
    }
}

impl <T> FromRedisValue for MyType<T> where T: Decodable{
    fn from_redis_value(v: &Value) -> RedisResult<MyType<T>> {
        if let Value::Data(ref items)=*v{
            let decoded: T = decode(&items[..]).unwrap();
            return Ok(MyType(decoded));
        }
        Err(RedisError::from(Error::new(ErrorKind::Other, "oh no!")))
    }
}


lazy_static! {
    static ref POOL:Pool<RedisConnectionManager>  = connect_pool();
}
fn connect_pool()->Pool<RedisConnectionManager>{
    let config = Default::default();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Pool::new(config, manager).unwrap();
    pool
}

fn get_conn()->PooledConnection<RedisConnectionManager>{
    let conn = POOL.get().unwrap();
    conn
}
fn test1(){
    let t=Task{id:100,name:Some("aaa".to_owned())};
    set("mykey1",t);
    let t:RedisResult<Task>=get("mykey1");
    println!("{:?}",t);

    let vec=mock_data();
    set("mylist11",vec);
    let v:RedisResult<Vec<Task>>=get("mylist11");

    println!("{:?}",v);
}

fn mock_data()->Vec<Task>{
    let mut result: Vec<Task> = vec![];
    result.push(Task{id:1,name:Some("do work".to_owned())});
    result.push(Task{id:3,name:Some("learn".to_owned())});
    result.push(Task{id:3,name:Some("learn".to_owned())});
    result.push(Task{id:3,name:Some("如何把百度设为您的上网主页？".to_owned())});
    result.push(Task{id:3,name:Some("learn".to_owned())});
    result.push(Task{id:3,name:Some("learn".to_owned())});
    result
}
