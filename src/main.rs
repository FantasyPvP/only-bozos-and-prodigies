#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"sup bozo"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
