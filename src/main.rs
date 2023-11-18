#[macro_use]
extern crate rocket;

mod db;
mod recipes;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(db::stage()).attach(recipes::stage())
}
