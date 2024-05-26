use axum::{
    http::StatusCode, routing::{delete, get, post}, Json, Router,
    extract::Path
};
use my_server_impl::{repos_mem::InMemoryThingRepo, usecases, Thing};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
   // initialize tracing
   tracing_subscriber::registry()
   .with(
       tracing_subscriber::EnvFilter::try_from_default_env()
           .unwrap_or_else(|_| "my_server=debug".into()),
   )
   .with(tracing_subscriber::fmt::layer())
   .init();

   let app = Router::new()
    .route("/", get(root))
    .route("/things", get(things))
    .route("/things", post(add_thing))
    .route("/things/:id", get(get_thing))
    .route("/things/:id", delete(del_thing));

// run our app with hyper, listening globally on port 3000
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    usecases::root()
}

async fn things() -> (StatusCode, Json<Vec<Thing>>) {
    let l = InMemoryThingRepo::new();
    match usecases::things_list(l) {
        Ok(list) => {
            return (StatusCode::OK, Json(list));
        },
        Err(_) => {
            // TODO logging
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()));
        }
    };
}

async fn add_thing(Json(new_thing): Json<Thing>) -> (StatusCode, Json<Thing>) {
    let l = InMemoryThingRepo::new();
    match usecases::add_thing(new_thing, l) {
        Ok(t) => {
            return (StatusCode::CREATED, Json(t));
        },
        Err(_) => {
            // TODO logging
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Thing::default()))
        },
    }
}

async fn get_thing(Path(id): Path<u32>) -> (StatusCode, Json<Thing>) {
    let l = InMemoryThingRepo::new();
    match usecases::get_thing(id, l) {
        Ok(t) => {
            return (StatusCode::OK, Json(t));
        },
        Err(_) => {
            // TODO logging
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Thing::default()))
        },
    }
}

async fn del_thing(Path(id): Path<u32>) -> (StatusCode, Json<Thing>) {
    let l = InMemoryThingRepo::new();
    match usecases::del_thing(id, l) {
        Ok(t) => {
            return (StatusCode::OK, Json(t));
        },
        Err(_) => {
            // TODO logging
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Thing::default()))
        },
    }
}
