use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use argon2::{
    self,
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use dtos::user::DatabaseUser;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection, Database,
};
use password::Password;
use std::env;

mod dtos;
mod password;

async fn get_new_client() -> Client {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let options =
        ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare())
            .await
            .expect("Failed to parse options");

    let client = Client::with_options(options).expect("Failed to initialize client.");

    client
}

async fn index() -> &'static str {
    "This server is online!"
}

async fn register(info: web::Json<dtos::user::DtoUser>) -> impl Responder {
    let password = Password::new(&info.password);
    if password.is_secure() == false {
        return HttpResponse::BadRequest().body("Password is not secure");
    }

    let client: Client = get_new_client().await;
    let db: Database = client.database("users");
    let coll: Collection<dtos::user::DatabaseUser> = db.collection("users");

    let database_user = dtos::user::DatabaseUser::new(&info.name, &info.email, password);

    let result = database_user.insert_to_database(coll).await;

    if result.inserted_id.to_string().is_empty() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().body(result.inserted_id.to_string())
}

async fn login(info: web::Json<dtos::user::LoginUser>) -> impl Responder {
    let client = get_new_client().await;
    let db = client.database("users");
    let coll = db.collection("users");

    println!("recieved login request with email {}", info.email);

    let result: Option<DatabaseUser> = coll
        .find_one(doc! {"email": &info.email}, None)
        .await
        .unwrap();

    if result.is_none() {
        println!("Account not found");
        return HttpResponse::NotFound().finish();
    }
    let result: DatabaseUser = result.unwrap();

    let password = &info.password;
    let password = password.as_bytes();

    let parsed_db_hash = PasswordHash::new(&result.password).expect("Failed to parse hash");

    let is_valid = Argon2::default()
        .verify_password(password, &parsed_db_hash)
        .is_ok();

    if is_valid == false {
        println!("Password is not valid");
        return HttpResponse::NotFound().finish();
    }

    let auth_token = SaltString::generate(&mut OsRng).to_string();

    println!("issued auth_token: {}", auth_token);

    let result = coll
        .update_one(
            doc! {"email": &info.email},
            doc! {"$push": {"auth_tokens": &auth_token}},
            None,
        )
        .await;

    if result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().body(auth_token)
}

async fn verify(info: web::Json<dtos::user::VerifyRequest>) -> impl Responder {
    let verify_secret = env::var("VERIFY_SECRET").expect("VERIFY_SECRET must be set");

    if info.verify_secret != verify_secret {
        println!("Invalid verify secret");
        return HttpResponse::NotFound().finish();
    }

    let client = get_new_client().await;
    let db = client.database("users");
    let coll: Collection<DatabaseUser> = db.collection("users");

    let result = coll
        .find_one(
            doc! {"auth_tokens":  &info.auth_token, "email": &info.email},
            Some(mongodb::options::FindOneOptions::builder().build()),
        )
        .await;

    println!("Recieved verify request with email: {}", info.email);
    let is_verified = result.unwrap().is_some();

    HttpResponse::Ok().body(is_verified.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on port 3000...");

    println!("Environment: ");
    println!("MONGO_URI: {}", env::var("MONGO_URI").unwrap());

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/verify", web::post().to(verify))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
