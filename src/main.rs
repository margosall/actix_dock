use actix_web::{web, App, HttpServer};

use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<u32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard

    let r=format!("Täiesti suvaline!\nKülastaja number: {counter}"); // <- response with count
    *counter += 1; // <- access counter inside MutexGuard
    r
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    
    println!("Server started!");
    HttpServer::new(move || {
        App::new()
        .app_data(counter.clone()) // <- register the created data
        .route("/", web::get().to(index))
})
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
