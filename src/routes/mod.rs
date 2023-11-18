use rocket::fairing::AdHoc;

mod recipe;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Recipe routes", |rocket| async {
        rocket.mount("/recipes", routes![recipe::get_recipe])
    })
}
