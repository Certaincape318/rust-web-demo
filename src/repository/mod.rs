pub mod task;
pub mod account;

use postgres::rows::*;
use postgres::types::ToSql;
use r2d2::{Config, Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, SslMode};
use rustc_serialize::{Encodable, Decodable};
use cache;
use utils::config::Config as MyConfig;
pub trait Row2Model {
    fn convert(row: Row) -> Self;
}

fn get_conn() -> PooledConnection<PostgresConnectionManager> {
    match POOL.get() {
        Ok(conn) => conn,
        Err(err) => panic!("error in get_conn():{}", err),
    }
}
lazy_static! {
    static ref POOL:Pool<PostgresConnectionManager>  = connect_pool();
}
fn connect_pool() -> Pool<PostgresConnectionManager> {
    let config = MyConfig::default();
    let host = config.get_str("database.host");
    let port = config.get_str("database.port");
    let user_name = config.get_str("database.user_name");
    let password = config.get_str("database.password");
    let db_name = config.get_str("database.db_name");

    let connect_str = format!("postgres://{}:{}@{}:{}/{}",
                              user_name,
                              password,
                              host,
                              port,
                              db_name);
    info!("Connecting to postgres:{}", connect_str);
    let manager = PostgresConnectionManager::new(connect_str.as_str(), SslMode::None).unwrap();

    let config = Config::builder().pool_size(10).build();
    match Pool::new(config, manager) {
        Ok(pool) => {
            info!("Connected to postgres with pool: {:?}", pool);
            return pool;
        }
        Err(err) => {
            panic!("error occurs when connect to postgres {}.Error info:{}",
                   connect_str,
                   err);
        }
    };
}

pub fn find_cached_list<T>(query: &str, params: &[&ToSql], cache_key: &str) -> Vec<T>
    where T: Row2Model + Decodable + Encodable + Clone
{
    match cache::get(cache_key) {
        Ok(list) => {
            debug!("get from redis cache");
            return list;
        }
        Err(err) => {
            error!("error while fetching data from cache:{}", err);
            let _ = cache::del(cache_key);
        }
    }
    let list = find_list(query, params);
    cache::set(cache_key, list.clone()).unwrap();
    list
}
pub fn find_one<T>(query: &str, params: &[&ToSql]) -> Option<T>
    where T: Row2Model
{
    let conn = get_conn();
    match conn.query(query, params) {
        Ok(rows) => {
            for row in &rows {
                return Some(T::convert(row));
            }
        }
        Err(err) => {
            panic!("error occur when execute query:{},params:{:?},error:{}",
                   query,
                   params,
                   err)
        }
    }
    None
}

pub fn find_list<T>(query: &str, params: &[&ToSql]) -> Vec<T>
    where T: Row2Model
{
    let mut result: Vec<T> = vec![];
    let conn = get_conn();
    match conn.query(query, params) {
        Ok(rows) => {
            for row in &rows {
                result.push(T::convert(row));
            }
        }
        Err(err) => {
            panic!("error occur when execute query:{},params:{:?},error:{}",
                   query,
                   params,
                   err)
        }
    }
    result
}

// return: the number of rows modified
pub fn execute(query: &str, params: &[&ToSql]) -> u64 {
    let conn = get_conn();
    // conn.execute(query,params).unwrap()
    match conn.execute(query, params) {
        Ok(count) => {
            return count;
        }
        Err(err) => {
            panic!("error occur when execute query:{},params:{:?},error:{}",
                   query,
                   params,
                   err)
        }
    }
}
