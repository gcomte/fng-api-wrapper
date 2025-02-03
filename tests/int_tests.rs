use chrono::{DateTime, Duration, TimeZone, Utc};
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
    let feb1st2018 = Utc.with_ymd_and_hms(2018, 2, 1, 0, 0, 0).unwrap();

    assert_eq!(fng.data.last().unwrap().timestamp, feb1st2018);

    let days_since_feb_1st_2018 = (0..)
        .map(|i| Utc::now().date_naive() - Duration::days(i))
        .take_while(|&d| d >= feb1st2018.date_naive())
        .count();

    // For unknown reasons, some days are not provided by the API:
    let missing_dates: Vec<DateTime<Utc>> = vec![
        Utc.with_ymd_and_hms(2018, 4, 14, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2018, 4, 15, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2018, 4, 16, 0, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2024, 10, 26, 0, 0, 0).unwrap(),
    ];

    for date in &missing_dates {
        assert!(!fng.data.iter().any(|d| &d.timestamp == date));
    }

    assert_eq!(
        days_since_feb_1st_2018 - missing_dates.len(),
        fng.data.len()
    );
}
