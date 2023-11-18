#[macro_use]
extern crate rocket;

mod db;

use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;

#[get("/<id>")]
async fn read(mut db: Connection<db::Recipes>, id: i64) -> Option<String> {
    sqlx::query!(
        "SELECT title, serves, descr
        FROM recipes WHERE id = ?",
        id
    )
    .fetch_one(&mut **db)
    .await
    .and_then(|r| {
        let show = format!("This {} serves {} people.\n{}", r.title, r.serves, r.descr);
        Ok(show)
    })
    .ok()
}

#[launch]
fn rocket() -> _ {
    // Run DB migrations, if there are any pending
    rocket::build()
        .attach(db::stage())
        .mount("/recipes", routes![read])
}
