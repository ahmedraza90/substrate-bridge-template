// 1. In Node.js / Express
// When you write:

app.get("/user/:id", (req, res) => {
  const id = req.params.id;   // Express filled this for you
  res.send("User " + id);
});


// You didn’t create req or res yourself.
// Express built them for you and passed them into your function.
// It looked at the HTTP request and said:
// "req = info about the request (params, body, headers...)"
// "res = helper for sending back a response"
// So your handler just receives those arguments.

// 2. In Rust / Axum
// Same idea. In Axum:

// With req.params in Express, you tell Express “give me this param”.
// With Axum, you use extractors like Path, Json, or State to tell it where to pull data from.

Example:

async fn handler(
    axum::extract::State(state): axum::extract::State<AppState>, // from shared state
    axum::extract::Path(id): axum::extract::Path<u32>,           // from URL
    axum::extract::Json(body): axum::extract::Json<MyBody>,      // from request body
) {
    // Axum gave you everything already filled in
}


// State<AppState> → comes from app’s shared state (.with_state)
// Path<u32> → comes from URL params
// Json<MyBody> → comes from request body
// Axum builds these arguments just like Express builds req and res.

// Example Without State
use axum::{Router, routing::get};
use std::net::SocketAddr;

struct DbClient;

async fn handler(db: &DbClient) {
    println!("Using DB client at: {:p}", db);
}

#[tokio::main]
async fn main() {
    let db = DbClient;

    // ❌ Wrong: trying to give handler a reference to db
    let app = Router::new().route("/", get(|| async {
        handler(&db).await   // db borrowed here
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 🚨 What Happens Here

// The closure for get is 'static — it might live as long as the whole server.
// But db only lives inside main.
// The borrow &db would not compile because the compiler sees a risk:
// the closure could be called after db is gone.
// → Lifetime violation.
// You’ll get an error like:
// borrowed value does not live long enough

// How Axum State Solves This
use axum::{
    Router, 
    routing::get,
    extract::State
};
use std::net::SocketAddr;

#[derive(Clone)]  // ← Must implement Clone
struct DbClient;

// ✅ Fixed: Use State extractor instead of borrowing
async fn handler(State(db): State<DbClient>) {
    println!("Using DB client at: {:p}", &db);
}

#[tokio::main]
async fn main() {
    let db = DbClient;

    // ✅ Move ownership to Axum's state management
    let app = Router::new()
        .route("/", get(handler))      // ← Just pass the function
        .with_state(db);               // ← Give ownership to Axum

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// What Axum Does Behind the Scenes

// 1. When you call .with_state(db):
//    Axum takes OWNERSHIP of your DbClient
//    Stores it safely inside the Router

// 2. When a request comes in:
//    Axum clones the DbClient
//    Wraps it in State<DbClient>
//    Passes it to your handler

// 3. Your handler gets its own copy:
//    No borrowing, no lifetime issues!

// Why This Works

// Ownership Transfer: You give Axum the DbClient - no more borrowing
// Clone on Demand: Axum clones it for each request - each handler gets its own copy
// 'static Lifetime: The cloned data lives as long as needed - no lifetime issues