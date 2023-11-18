#[macro_use]
extern crate rocket;

mod db;
mod logic;
mod model;
mod persistence;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(db::stage()).attach(routes::stage())
}
