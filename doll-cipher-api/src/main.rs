/*
DOLL-CIPHER API by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "var"
/// function to read
/// environment variables.
use std::env::var;

/// Importing the "GET"
/// response handler.
use actix_web::get;

/// Importing the "web"
/// handler to parse slugs.
use actix_web::web;

/// Importing the "App"
/// structure to make a new
/// Actix Web app.
use actix_web::App;

/// Importing the "Serialize"
/// derive-macro so we can make
/// a JSON response.
use serde::Serialize;

/// Importing the Actix Web
/// result.
use actix_web::Result;

/// Importing the method
/// from my cryptography
/// crate to generate a key.
use doll_cipher::gen_key;

/// Importing the "encrypt"
/// function from my 
/// cryptography library.
use doll_cipher::encrypt;

/// Importing the "decrypt"
/// function from my 
/// cryptography library.
use doll_cipher::decrypt;

/// Importing the a response
/// hanlder from Actix Web.
use actix_web::Responder;

/// Importing Actix Web's
/// HTTP server.
use actix_web::HttpServer;

/// Importing the "KeyLength"
/// enum to give users the
/// option to generate keys
/// of different strengths.
use doll_cipher::KeyLength;

/// Importing the "HashMap"
/// structure because making
/// a struct for key generation
/// is overkill.
use std::collections::HashMap;

/// A structure to hold
/// the API's response.
#[derive(Serialize)]
pub struct CipherResponse{
    pub key: String,
    pub msg: String
}

/// Implements methods for
/// the "CipherResponse"
/// structure.
impl CipherResponse {

    /// A method to create
    /// a new "instance"
    /// of the "CipherResponse"
    /// structure.
    pub fn new(
        msg: &String,
        key: &String
    ) -> CipherResponse{
        CipherResponse{
            key: key.to_owned(),
            msg: msg.to_owned()
        }
    }

}

/// Sanitizes "splitter" to "joiner" in a string.
pub fn sanitize_string(subject: &String, splitter: &str, joiner: &str) -> String {
    let mut result: Vec<String> = Vec::new();
    let underscore_iterator = subject.split(splitter);
    for word in underscore_iterator {
        result.push(word.to_string());
    }
    result.join(joiner)
}

/// The path that accepts a
/// "key" and "msg" slug
/// to encrypt "msg".
#[get("/e/{key}/{msg}")]
async fn encrypt_msg(
    path: web::Path<(String, String)>
) -> Result<impl Responder> {
    let (key,msg) = path.into_inner();
    let key_var: &String = &key;
    let mut msg_var: String = String::from("N/A");
    match encrypt(&sanitize_string(&msg, "_", " "), &key){
        Ok(res) => {
            msg_var = sanitize_string(&res, "\n", "|");
        },
        Err(_e) => {}
    };
    let obj = CipherResponse::new(&msg_var, key_var);
    Ok(web::Json(obj))
}

/// The path that accepts a
/// "key" and "msg" slug
/// to decrypt "msg".
#[get("/d/{key}/{msg}")]
async fn decrypt_msg(
    path: web::Path<(String, String)>,
) -> Result<impl Responder> {
    let (key,msg) = path.into_inner();
    let key_var: &String = &key;
    let mut msg_var: String = String::from("N/A");
    match decrypt(&sanitize_string(&msg, "|", "\n"), &key){
        Ok(res) => {
            msg_var = res;
        },
        Err(_e) => {}
    };
    let obj = CipherResponse::new(&msg_var, key_var);
    Ok(web::Json(obj))
}

/// Generates a key of the given
/// strength.
#[get("/genkey/{strength}")]
async fn generate_key(
    strength: web::Path<String>,
) -> Result<impl Responder> {
    let mut obj: HashMap<String, String> = HashMap::new();
    let mut key_strength: KeyLength = KeyLength::SuperSecure;
    let mut key: String = String::from("");
    if strength.to_string() == String::from("iron"){
        key_strength = KeyLength::Iron;
    }
    else if strength.to_string() == String::from("msec"){
        key_strength = KeyLength::Secure;
    }
    else if strength.to_string() == String::from("ssec"){
        key_strength = KeyLength::SuperSecure;
    }
    else {}
    match gen_key(&key_strength){
        Ok(gen) => {key = gen;}
        Err(_e) => {}
    };
    obj.insert(String::from("key"), key);
    Ok(web::Json(obj))
}

/// The main point of
/// entry for the Rust compiler
/// and Actix Web.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut host: String = String::from("127.0.0.1");
    let mut port: u16 = 8080;
    match var("ACTIX_WEB_ADDR"){
        Ok(addr) => {host = addr;}
        Err(_e) => {}
    };
    match var("PORT"){
        Ok(set_port) => {
            match set_port.parse::<u16>(){
                Ok(parsed) => {port = parsed;},
                Err(_e) => {}
            };
        }
        Err(_e) => {}
    };
    println!("Starting server on \"{}:{}\"", &host, &port);
    HttpServer::new(|| {
        App::new()
            .service(encrypt_msg)
            .service(decrypt_msg)
            .service(generate_key)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}