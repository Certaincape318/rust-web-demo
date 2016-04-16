extern crate demo as app;
extern crate env_logger;

fn main(){
    env_logger::init().unwrap();
    app::run();
}
