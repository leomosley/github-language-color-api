use std::fs;
use axum::{Router, routing::get, response::Json};
use serde_json::Value;
use axum::http::{StatusCode, Response};
use axum::extract::Path;
use shuttle_runtime::main;

#[derive(Debug)]
struct AppState {
    // Define your state fields here
}

async fn index() -> &'static str {
    "To retrieve the color for a programming language, make a GET request to /<language>. Example: GET /rust"
}

fn get_json() -> Option<Value> {
    let json_string = match fs::read_to_string("src/colors.json") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("{}", err);
            return None;
        }
    };

    match serde_json::from_str(&json_string) {
        Ok(value) => Some(value),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

async fn get_language(Path(language): Path<String>) -> Result<Json<String>, Response<String>> {
    let language_lower = language.to_lowercase();
    let data = get_json().ok_or_else(|| {
        let msg = format!("Language '{}' not found", language_lower);
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(msg)
            .unwrap()
    })?;

    if let Some(language_data) = data.get(&language_lower) {
        let color = language_data["color"].as_str().ok_or_else(|| {
            let msg = "Color not found".to_string();
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(msg)
                .unwrap()
        })?;
        Ok(Json(color.to_string()))
    } else {
        let msg = format!("Language '{}' not found", language);
        Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(msg)
            .unwrap())
    }
}

#[main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index))
        .route("/:language", get(get_language));

    Ok(router.into())
}
