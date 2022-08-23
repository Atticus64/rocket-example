#[macro_use] extern crate rocket;



#[get("/")]
fn index() -> &'static str{
    "Hola Mundo"
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}











