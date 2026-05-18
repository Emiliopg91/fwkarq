use crate::rest_client::{HttpMethod, RestClient, error::RestClientError};

#[test]
pub fn test_01_get() {
    println!("");
    RestClient::new("https://jsonplaceholder.typicode.com/todos/1")
        .invoke()
        .unwrap();
}

#[test]
pub fn test_02_post() {
    println!("");
    RestClient::new("https://jsonplaceholder.typicode.com/posts")
        .method(HttpMethod::POST)
        .header("Content-Type", "application/json")
        .unwrap()
        .body(
            r#"{
    "title": "hola",
    "body": "esto es una prueba",
    "userId": 1
}"#
            .to_string(),
        )
        .invoke()
        .unwrap();
}

#[test]
pub fn test_03_bad_header() {
    println!("");

    if let Some(e) = RestClient::new("https://jsonplaceholder.typicode.com/posts")
        .header("Content Type", "application/json")
        .err()
    {
        match e {
            RestClientError::InvalidRequestHeader(_, _, _) => {
                //OK!!
            }
            _ => {
                assert!(false, "Expected InvalidRequestHeader error, {} found", e)
            }
        }
    } else {
        panic!("Expected InvalidRequestHeader error")
    }
}
