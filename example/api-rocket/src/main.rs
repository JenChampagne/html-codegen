use html_codegen::dylibload::dylibload;
use rocket::{get, http::Status, launch, routes};

//#[dylibload(html, "/home/iferc/.cargo/target")]
#[dylibload(example_html, "../dist")]
pub fn never_error() -> String {}

#[get("/")]
fn index() -> Result<String, Status> {
    never_error().map_err(|error| {
        eprintln!("Failed to load html crate with error: {error}");
        Status::InternalServerError
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
