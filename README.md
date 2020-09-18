# csv_to_json_rs
A simple REST Post API where you can send csv and get JSON in return.

This is a minimal viable product of a [Warp](https://github.com/seanmonstar/warp)-based 
asynchronous server that takes Post requests at `~/csv_to_json/` with a body containing a CSV, 
and returns a JSON object. 

# To run start the server locally:
```bash 
cargo run --release
```

You can test the API by sending it post requests, e.g.:
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

## ToDo:
Right now: the API just panics if it cannot parse the csv, 
e.g. some rows may have fewer columns etc. 
The Warp server handles this gracefully, but it may be a good idea to either log the error on the server side,
or return and error to the caller.

## Next Steps:
1) Add some options to the caller when parsing csv, e.g. specify delimiters, no headers, whether or not numbers should be parsed or saved as string
2) Add database to save the JSON on serverside.
3) Limit body length for extremely large CSV.
4) Logging
5) ???

## Dependencies:
- [csv](https://docs.rs/csv) -- for csv parsing
- [serde_json](https://docs.serde.rs/serde_json/) -- for JSON conversion and serialization.
- [Warp](https://docs.rs/warp) -- for the HTTP stack
- [tokio](https://tokio.rs/) -- for the async runtime
