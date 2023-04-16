# Crypto Fear and Greed API wrapper
`fng-api-wrapper` is a Rust API client for Crypto Fear &amp; Greed Index.

The wrapper is incomplete and does not cover the entire API.  
So far it only covers the endpoint https://api.alternative.me/fng/

## Sample code
```rust
use chrono::Datelike;
use fng_api_wrapper::client;

fn main() {
    let fng = client::fetch_daily_fng(10).unwrap();
    fng.data.iter().for_each(|daily_record| {

        let ts = &daily_record.timestamp;
        let (year, month, day) = (ts.year(), ts.month(), ts.day());

        println!(
            "{year}-{month:0>2}-{day:0>2}: {:>2} -> {:?}",
            daily_record.value, daily_record.value_classification
        );
    });
}
```
