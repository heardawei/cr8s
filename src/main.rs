#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket_sync_db_pools::database("postgres_app_db")]
pub struct DbConn(diesel::PgConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::view_rustacean,
                rocket_routes::rustaceans::create_rustacean,
                rocket_routes::rustaceans::delete_rustacean,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::crates::get_crates,
                rocket_routes::crates::view_crate,
                rocket_routes::crates::create_crate,
                rocket_routes::crates::delete_crate,
                rocket_routes::crates::update_crate,
            ],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
