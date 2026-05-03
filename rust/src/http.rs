//! Thin HTTP helpers over extism-pdk's `http::request`.

use extism_pdk::{http, HttpRequest};

pub fn get(url: &str) -> Result<Vec<u8>, extism_pdk::Error> {
    let req = HttpRequest::new(url).with_method("GET");
    let resp = http::request::<()>(&req, None)?;
    Ok(resp.body())
}

pub fn post_json(url: &str, body: &[u8]) -> Result<Vec<u8>, extism_pdk::Error> {
    let req = HttpRequest::new(url)
        .with_method("POST")
        .with_header("Content-Type", "application/json");
    let resp = http::request(&req, Some(body))?;
    Ok(resp.body())
}
