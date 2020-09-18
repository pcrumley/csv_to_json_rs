use std::collections::HashMap;
use std::convert::Infallible;
use serde::{Serialize};
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};
use serde_json::json;
use std::error::Error;

/// Converts a csv file which is bytes into a json file
///
/// # Examples
/// ```
/// let csv = bytes::Bytes::from(&b"city,country,pop
/// Boston,United States,4628910
/// Austin,United States,964254"[..])
/// let json = super::csv_to_json(csv).unwrap();
/// ```
fn csv_to_json(csv_file: bytes::Bytes) -> Result<serde_json::value::Value, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(&csv_file[..]);
    let mut ans = vec![];
    for result in rdr.deserialize() {
        // convert the result to a HashMap using try operator (?)
        // if this returns Box<dyn Error> it should be handled.
        let record: HashMap<String, String> = result?;
        ans.push(record);
    }
    Ok(json!(ans))
}

/// csv_handler is a wrapper method around csv_to_json which allows for better error handling.
///
/// If csv_to_json returns a Result, it sends it to the request.
///
/// If csv_to_json return an error in one of the lines due to parsing,
/// We use our custom Error wrapper (see struct CsvError)
async fn csv_handler(csv_file: bytes::Bytes)
    -> Result<impl warp::Reply, warp::Rejection> {
    let json = csv_to_json(csv_file);
    match json {
        Err(e) => Err(reject::custom(CsvError{
            message: format!("{}", e)})),
        Ok(v) => Ok(warp::reply::json(&v))
    }

}

#[tokio::main]
async fn main() {
    // put ~/csv_to_json/ => 200 OK with body "Hello, warp!"
    let csv_in_body = warp::path("csv_to_json")
        .and(warp::post())
        .and(warp::path::end())
        .and(warp::body::bytes())
        .and_then(|bytes: bytes::Bytes| {
            csv_handler(bytes)
         });
    // the nice thing about warp is it handles errors at the filter level and you can try to
    // recover from them with a particular method. We are most interested in letting the caller
    // know that the csv parsing failed because the csv was not formatted correctly.
    //
    // This is done in the handle_rejection method
    let route = csv_in_body.recover(handle_rejection);
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// A struct where we will save our CsvError message, so we can return it to the caller.
#[derive(Debug)]
struct CsvError {
    message: String
}

// have to implement the Reject trait on the the struct so it can be used as a custom error type.
impl reject::Reject for CsvError {}

/// An API error serializable to JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}


// This function receives a `Rejection` and tries to return a custom
// value, otherwise simply passes the rejection along.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(csv_error) = err.find::<CsvError>() {
        // The most important use case, if the csv_parser failed, we
        // need the caller to know why. We will return with a 400 and the
        // message
        code = StatusCode::BAD_REQUEST;
        message = &csv_error.message;
    } else {
        // Who knows what happend, Log the error and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });
    // return the status and message to the caller
    Ok(warp::reply::with_status(json, code))
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_csv_parser() {
        // this test checks that the csv_parser is workd
        let csv = bytes::Bytes::from(&b"city,country,pop
Boston,United States,4628910
Austin,United States,964254"[..]);
        let json = super::csv_to_json(csv).unwrap();
        assert_eq!(json, serde_json::from_str::<serde_json::value::Value>(
        "[
            {\"city\":\"Boston\",\"country\":\"United States\",\"pop\":\"4628910\"},
            {\"city\":\"Austin\",\"country\":\"United States\",\"pop\":\"964254\"}
        ]" ).unwrap());
    }
}
