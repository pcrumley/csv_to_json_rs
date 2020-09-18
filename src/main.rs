use warp::Filter;
use std::collections::HashMap;
use serde_json::json;
use std::error::Error;

/// Converts a csv file which is bytes into a json file
///
/// To Do:
/// 1) Better Error handling -> Done, could be better.
/// 2) try and parse into numbers: Should do...
/// 3) add flags that allow to change delimiter: Reach goal...
///
/// # Examples
/// let csv = bytes::Bytes::from(&b"city,country,pop
/// Boston,United States,4628910
/// Austin,United States,5999999"[..])
/// let json = super::csv_to_json(csv);

fn csv_to_json(csv_file: bytes::Bytes) -> Result<serde_json::value::Value, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(&csv_file[..]);
    let mut ans = vec![];
    for result in rdr.deserialize() {
        // Examine our Result.
        // If there was no problem, print the record.
        // Otherwise, convert our error to a Box<dyn Error> and return it.
        let record: HashMap<String, String> = result?;
        ans.push(record);
    }
    Ok(json!(ans))
}

/// csv_handler is a wrapper method around csv_to_json which allows for better error handling.
/// Right now, it just panics! This will change.

async fn csv_handler(csv_file: bytes::Bytes)
    -> Result<impl warp::Reply, warp::Rejection> {
    let json = csv_to_json(csv_file).unwrap();
    Ok(warp::reply::json(&json))
}

#[tokio::main]
async fn main() {
    // put /hello/warp => 200 OK with body "Hello, warp!"
    let csv_filter = warp::post()
        .and(warp::path("csv_to_json"))
        .and(warp::path::end())
        .and(warp::body::bytes())
        .and_then(|bytes: bytes::Bytes| {
            csv_handler(bytes)
         });

    warp::serve(csv_filter)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_csv_parser() {
        // this test checks that the csv_parser is workd
        let csv = bytes::Bytes::from(&b"city,country,pop
Boston,United States,4628910
Austin,United States,5999999"[..]);
        let json = super::csv_to_json(csv);
        assert_eq!(json, serde_json::from_str::<serde_json::value::Value>(
        "[
            {\"city\":\"Boston\",\"country\":\"United States\",\"pop\":\"4628910\"},
            {\"city\":\"Austin\",\"country\":\"United States\",\"pop\":\"5999999\"}
        ]" ).unwrap());
    }
}
