#[macro_use] extern crate rocket;
extern crate easy_http_request;
extern crate json;

// use rocket::response::content;
use rocket::http::{Status, ContentType};
use rocket_dyn_templates::{Template, context};
use rocket::serde::{Deserialize, Serialize, json::Json};
use easy_http_request::DefaultHttpRequest;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
  name: String,
  age: u8,
  alive: bool, 
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct UserId {
  name: String,
  age: u8,
  alive: bool,
  id: u8 
}


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

#[get("/chuck")]
fn chuck() -> Template {
    let response = DefaultHttpRequest::get_from_url_str("https://api.chucknorris.io/jokes/random").unwrap().send().unwrap();

    let answer: String = String::from_utf8(response.body).unwrap(); 
    let resp= json::parse(&answer).unwrap();

    let joke = resp["value"].to_string(); 

    Template::render("chuck", context! {
        title: "Chuck Norris Page",
        joke
    })
}

#[get("/about")]
fn about() -> Template {
    let response = DefaultHttpRequest::get_from_url_str("https://yesno.wtf/api").unwrap().send().unwrap();
    
    // println!("{}", response.status_code);
    // println!("{:?}", response.headers);
    let answer: String = String::from_utf8(response.body).unwrap(); 
    let eureka = json::parse(&answer).unwrap();
    // let eureka = Json(&answer);
    // let image = String::Copy(&eureka["image"]);
    // let resp = eureka.dump();;
    let image: String = eureka["image"].to_string();
    let answer: String = eureka["answer"].to_string();
    println!("{:#?}", eureka["image"]);
    Template::render("about", context! {
        title: "About Page",
        image,
        answer
    })
    // "Hola Mundo"
}

#[get("/user")]
fn user() -> Json<User> {

    let usuario = User {
        name: String::from("Juan Escutia"),
        age: 20,
        alive: false
    };

    Json(usuario)
}

#[get("/user/<id>")]
fn user_by_id(id: u8) -> Json<UserId> {

    let usuario = UserId {
        name: String::from("Tux"),
        age: 31,
        alive: true,
        id
    };

    Json(usuario)
}

#[get("/hello")]
fn jsapi() -> (Status, (ContentType, &'static str)) {
    (Status::Accepted, (ContentType::JSON, "{ \"hi\": \"world\" }"))
}

#[post("/api", data = "<persona>" )]
fn api(persona: Json<Persona<'_>>) -> String {


    let nombre = persona.nombre;
    let edad  = persona.edad;

    if  nombre.len() < 1 && edad.to_string().len() < 1 {
        return format!("Test")
    }
    if  nombre != ""  && edad >= 18 && edad <= 115 {
        return format!("Hola {}, tu edad es {}, así que eres mayor de edad", nombre, edad)
    } if nombre != ""  && edad < 18 {
        return format!("Hola {}, tu edad es {}, así que eres menor de edad", nombre, edad)
    } if nombre != "" && edad > 115 {
        return format!("Hola {}, tu edad no es correcta, deja de jugar! :)", nombre);
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
        .mount("/", routes![index, about, api, user, user_by_id, jsapi, chuck])
        .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
        .attach(Template::fairing())
}











