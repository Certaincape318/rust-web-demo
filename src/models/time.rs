use chrono::*;
use std::io::prelude::*;
use rustc_serialize::*;
use postgres;
use postgres::types::{Type, FromSql, ToSql, IsNull, SessionInfo};
use redis::RedisResult;
use redis::ToRedisArgs;
use redis::FromRedisValue;
use redis::RedisError;
use redis::Value;
use std::io::Error;
use std::io::ErrorKind;
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};


#[derive(Debug,Copy,Clone, PartialEq,Eq)]
pub struct Time{
    value:DateTime<UTC>,
}
impl Encodable for Time {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.value.format("%Y-%m-%d %H:%M:%S").to_string().encode(s)
    }
}
impl Decodable for Time{
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>{
        match String::decode(d){
            Ok(value)=>{
                match UTC.datetime_from_str(&value, "%Y-%m-%d %H:%M:%S"){
                    Ok(value)=>Ok(Time{value:value}),
                    Err(_)=>panic!("error decode datetime")
                }
            },
            Err(_)=>panic!("error decode datetime")
        }
    }
}

impl Time{
    pub fn new()->Self{
        Time{value:UTC::now()}
    }
}

impl FromSql for Time {
    fn from_sql<R: Read>(ty: &Type, raw: &mut R, ctx: &SessionInfo) -> postgres::Result<Time> {
        DateTime::from_sql(ty,raw,ctx).map(|value| Time{value:value})
    }
    fn accepts(_: &Type) -> bool{
        true
    }
}

impl ToSql for Time {
    fn to_sql<W: Write + ?Sized>(&self, ty: &Type, mut w: &mut W, ctx: &SessionInfo) -> postgres::Result<IsNull> {
        self.value.to_sql(ty,w,ctx)
    }
    fn accepts(_: &Type) -> bool {
        true
    }
    fn to_sql_checked(&self, ty: &Type, out: &mut Write, ctx: &SessionInfo) -> postgres::Result<IsNull>{
        self.value.to_sql_checked(ty,out,ctx)
    }
}
impl ToRedisArgs for Time {
    fn to_redis_args(&self) -> Vec<Vec<u8>> {
        vec![encode(&self.value, SizeLimit::Infinite).unwrap()]
    }
}

impl FromRedisValue for Time {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        if let Value::Data(ref items)=*v{
            let decoded: DateTime<UTC> = decode(&items[..]).unwrap();
            return Ok(Time{value:decoded});
        }
        Err(RedisError::from(Error::new(ErrorKind::Other, "oh no!")))
    }
}
