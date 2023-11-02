#[macro_use] extern crate rocket;

use rocket::response::content;
use sha2::{Sha256, Digest};

#[get("/<username>/<pass>", format = "json")]
fn reg_me(username: &str, pass: &str) -> content::RawJson<String> {

    let mut hasher = Sha256::new();
    hasher.update(username);
    hasher.update(pass);

    let output = hasher.finalize();
    let hash_hex = output.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    let json_response = format!(r#"{{ "sha1pass": "{}" }}"#, hash_hex);
    content::RawJson(json_response)

}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/get_me", routes![reg_me])
}
