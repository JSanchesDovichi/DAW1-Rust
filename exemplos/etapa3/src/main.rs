use rocket::launch;

mod classes;
mod enums;

#[launch]
async fn rocket() -> _ {
    rocket::build()
}