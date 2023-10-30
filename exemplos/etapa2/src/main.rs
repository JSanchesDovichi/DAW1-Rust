use rocket::launch;

#[launch]
async fn rocket() -> _ {
    rocket::build()
}
