#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Persona<'r> {
    nombre: &'r str,
    edad: u8
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Rocket Overview"
    })
    // "Hola Mundo"
}

#[post("/api", data = "<persona>" )]
fn api(persona: Json<Persona<'_>>) -> String {


    let nombre = persona.nombre;
    let edad  = persona.edad;

    if  nombre.len() < 1 && edad.to_string().len() < 1 {
        return format!("Test")
    }
    if  nombre != ""  && edad > 18 {
        return format!("Hola {}, tu edad es {}, así que eres mayor de edad", nombre, edad)
    } if nombre != ""  && edad < 18 {
        return format!("Hola {}, tu edad es {}, así que eres menor de edad", nombre, edad)
    } else {
        return format!("Hola Api")
    }
    // format!("{} y {}", persona.edad, persona.nombre)
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
    // "Not Found"
}




#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, api])
        .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
        .attach(Template::fairing())
}











