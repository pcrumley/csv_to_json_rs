# csv_to_json_rs
A simple REST Post API where you can send csv and get JSON in return.

This is a minimal viable product of a [Warp](https://github.com/seanmonstar/warp)-based 
asynchronous server that takes Post requests at `~/csv_to_json/` with a body containing a CSV, 
and returns a JSON object. 

Right now, the csv_parsing is pretty simple. It assumes the deliminator is `,`, doesn't try to 
infer types, and that the first line of the file is a header. Be warned: violations of these assumptions 
may return a valid JSON object that is contrary to user expectations. This code also doesn't handle 
the case when rows are different lengths than others, however that is guaranteed to return an error 
(if and only if delimiter is `,`)

If the csv_parsing succeeds, the server sends a `200` status with the created JSON. If it fails, 
the server returns an error status with JSON giving a description of the error.

# To run start the server locally:
```bash 
cargo run --release
```

You can test the API by sending it post requests from a different shell, e.g.:
```bash
curl --location --request POST 'localhost:3030/csv_to_json' \
--header 'Content-Type: text/plain' \
--data-raw 'city,country,pop
Boston,United States,4628910
Austin,United States,964254
'
```
should return (*note* JSON is not pretty printed on the cmd line, but is here for legibility) 
```
[
  { "city": "Boston",
    "country": "United States",
    "pop": "4628910"
  },
  { "city": "Austin",
    "country": "United States",
    "pop": "964254"
  }
]
```

As an example of a bad request, if you send 
```bash -i 
curl --location --request POST 'localhost:3030/csv_to_json' \
--header 'Content-Type: text/plain' \
--data-raw 'city,country,pop
Boston,United States,462891,Will Fail
Austin,United States,964254
'
```
the server returns (again with pretty print for legibility)
```
HTTP/1.1 400 Bad Request
content-type: application/json
content-length: 130
date: Fri, 18 Sep 2020 20:49:07 GMT

{ 
  "code": 400,
  "message": "CSV error: record 1 (line: 2, byte: 17): found record with 4 fields, but the previous record has 3 fields"
}
```


## Next Steps:
1) Add some options to the caller when parsing csv, e.g. specify delimiters, no headers, whether 
    or not numbers should be parsed or saved as string.
2) Add database to save the JSON on serverside.
3) Limit body length for extremely large CSV (very easy to do in Warp, just don't know reasonable amount.)

## Important Dependencies:
- [csv](https://docs.rs/csv) -- for csv parsing
- [serde_json](https://docs.serde.rs/serde_json/) -- for JSON conversion and serialization.
- [Warp](https://docs.rs/warp) -- for the HTTP stack
- [tokio](https://tokio.rs/) -- for the async runtime
