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
