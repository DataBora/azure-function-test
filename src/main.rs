use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    //running example
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("HttpTest"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });

    // End point for adding 2 numbers
    let add_numbers = warp::get()
        .and(warp::path("api"))
        .and(warp::path("AddNumbers"))
        .and(warp::query::<HashMap<String,String>>())
        .map(|p: HashMap<String, String>|{
            let num1 = p.get("num1").and_then(|n|n.parse::<i32>().ok()).unwrap_or(0);
            let num2 = p.get("num2").and_then(|n| n.parse::<i32>().ok()).unwrap_or(0);
            let sum = num1 + num2;
            Response::builder().body(format!("The sum of {} and {} is {}:", num1, num2, sum ))
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1.or(add_numbers)).run((Ipv4Addr::LOCALHOST, port)).await;
}

