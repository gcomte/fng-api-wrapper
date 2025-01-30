use fng_api_wrapper::client;

#[test]
fn fetch_10_last_days() {
    // For debugging reasons: If the test fails, the API response is printed.
    println!(
        "{}",
        ureq::get("https://api.alternative.me/fng/?limit=10")
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
            .unwrap()
    );

    let fng = client::fetch_daily_fng(10).unwrap();
    assert_eq!(fng.data.len(), 10);
}

#[test]
fn fetch_maximum_days() {
    let fng = client::fetch_daily_fng_max_records().unwrap();

    // Delivers 5 years of data (a bit more than 365 * 5 because of leap years)
    assert!(fng.data.len() > 365 * 5);
    assert!(fng.data.len() < 365 * 6);
}
