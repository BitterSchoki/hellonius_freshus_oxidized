use rocket::fairing::AdHoc;
use rocket::{fairing, Build, Rocket};
use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("sqlite_recipes")]
pub struct Recipes(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Recipes::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database", |rocket| async {
        rocket
            .attach(Recipes::init())
            .attach(AdHoc::try_on_ignite("Database migrations", run_migrations))
    })
}
