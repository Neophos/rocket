#![allow(non_snake_case)]

#[macro_use] extern crate rocket;

mod card;
mod deck;

#[get("/")]
fn game() -> String {
    format!(":<!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![game])
}


#[cfg(test)]
mod test {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn shouldSuccessStatus() {
        let client = Client::tracked(rocket()).expect("rocketo uwu");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn shouldHappyFace() {
        let client = Client::tracked(rocket()).expect("rocketo uwu");
        let response = client.get("/").dispatch();

        assert_eq!(response.into_string().unwrap(), ":<!");
    }
}