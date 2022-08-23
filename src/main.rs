#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};


#[get("/")]
fn index() -> Template{
    Template::render("index", context! {
        title: "Rocket Overview"
    })
}

#[post("/api")]
fn api() -> &'static str{
    "Hola Api"
}

#[get("/")]
fn profile() -> &'static str {
    "Profile!"
}


#[post("/")]
fn create_profile() -> &'static str {
    "New profile!"
}

#[put("/")]
fn update_profile() -> &'static str {
    "Updated profile!"
}

#[delete("/")]
fn delete_profile() -> &'static str {
    "Deleted profile!"
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("notFound", context! {
        msg: "Not Found"
    })
}




#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, api])
        .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
        .attach(Template::fairing())
}











