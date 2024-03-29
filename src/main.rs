#![feature(plugin, const_fn, decl_macro, custom_attribute, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use] extern crate diesel;

extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] 
extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use routes::*;

mod schema;
mod models;
mod db;
mod static_files;
mod routes;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    
    let pool = db::init_pool(database_url);
    rocket::ignite()
    .manage(pool)
    .mount(
        "/api/v1",
        routes![index, new, show, delete, author, update],
    )
    .mount("/", routes![static_files::all, static_files::index])
    .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}

