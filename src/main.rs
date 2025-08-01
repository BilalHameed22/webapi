extern crate chrono;
extern crate env_logger;
extern crate iron;
extern crate logger;
extern crate router;
extern crate rustc_serialize;
extern crate uuid;

mod models;
mod database;
mod handlers;

use models::*;
use database::Database;
use handlers::*;

fn main() {
    println!("Hello, world!");
}
