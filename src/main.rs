#[macro_use]
extern crate rocket;

#[post("/", data = "<body>")]
fn report(body: String) {
    println!("{}", body);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![report])
}
