use rocket::launch;

pub mod classes;
pub mod enums;

#[launch]
async fn rocket() -> _ {
    rocket::build()
}