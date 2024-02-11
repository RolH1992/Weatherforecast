#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn create_rocket<P: Phase>() -> Rocket<P> {
    rocket::build().mount("/", routes![index, files])
}