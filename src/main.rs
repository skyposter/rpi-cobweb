#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

use rppal::gpio::Gpio;

use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct MainTemplate {
    status: String,
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "front",
        MainTemplate {
            status: String::from("Hello World"),
            
        },
    )
}

#[post("/btn/<action>")]
fn do_action(action: &str) {
    let gpio = Gpio::new().expect("unable to open GPIO");
    let mut pin = gpio.get(23).expect("unable to get pin 23").into_output();
    
    if action == "blink" {
        pin.set_high();
        thread::sleep(Duration::from_secs(1));
        pin.set_low();
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        do_action,
    ]).mount("/static", FileServer::from(relative!("static"))).attach(Template::fairing())
}
