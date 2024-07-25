use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};//Port, multipart::FormData

// use futures::StreamExt;
// use datafusion::prelude::*;
// use arrow::record_batch::RecordBatch;
// use arrow::csv::ReaderBuilder;

// use std::sync::Arc;
// use std::io::Cursor;

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
            Response::builder().body(format!("The sum of {} and {} is {}.", num1, num2, sum ))
        });


     // Endpoint for processing file
    //  let process_file = warp::post()
    //     .and(warp::path("api"))
    //     .and(warp::path("ProcessFile"))
    //     .and(warp::multipart::form().max_length(10_000_000)) // Limit form size to 10MB
    //     .and_then(process_file);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1.or(add_numbers)).run((Ipv4Addr::LOCALHOST, port)).await;
}

// async fn process_file(form: FormData) -> Result<impl warp::Reply, warp::Rejection> {
//     let mut buf = Vec::new();
    
//     // Collect all parts of the form
//     let parts: Vec<_> = form.collect().await;

//     for part in parts {
//         match part {
//             Ok(mut p) if p.name() == "file" => {
//                 while let Some(chunk) = p.data().await {
//                     let chunk = chunk.map_err(|_| warp::reject::not_found())?; // Use a more appropriate error
//                     buf.extend_from_slice(chunk.chunk());
//                 }
//             },
//             Err(_) => return Err(warp::reject::not_found()), // Return a rejection for errors
//             _ => continue,
//         }
//     }

//     let data = String::from_utf8(buf).unwrap();

//     // Process the CSV data
//     let mut rdr = ReaderBuilder::new().from_reader(data.as_bytes());
//     let mut wtr = WriterBuilder::new().from_writer(vec![]);
    
//     for result in rdr.records() {
//         let record = result.unwrap();
//         let mut new_record = StringRecord::new();
        
//         for (i, field) in record.iter().enumerate() {
//             if i == 0 {
//                 let formatted_date = if let Ok(naive_date) = NaiveDate::parse_from_str(field, "%Y-%m-%d") {
//                     naive_date.format("%Y%m%d").to_string()
//                 } else {
//                     field.to_string()
//                 };
//                 new_record.push_field(&formatted_date);
//             } else {
//                 new_record.push_field(field);
//             }
//         }
        
//         wtr.write_record(&new_record).unwrap();
//     }

//     let output = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
//     Ok(Response::builder().body(output))
// }
