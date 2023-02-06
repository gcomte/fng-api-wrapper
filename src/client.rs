use crate::errors::*;
use crate::wrapper::FearAndGreedIndex;
use std::cmp::Ordering;

const API_URL: &str = "https://api.alternative.me/fng/";

pub fn fetch_daily_fng_max_records() -> Result<FearAndGreedIndex, FngApiError> {
    fetch_fng(usize::MAX)
}

pub fn fetch_daily_fng(amt_days_in_past: usize) -> Result<FearAndGreedIndex, FngApiError> {
    let fng = fetch_fng(amt_days_in_past)?;

    match fng.data.len().cmp(&amt_days_in_past) {
        Ordering::Less => Err(FngApiError::ReceivedLessRecords),
        Ordering::Greater => Err(FngApiError::ReceivedMoreRecords),
        Ordering::Equal => Ok(fng),
    }
}

fn fetch_fng(amt_days_in_past: usize) -> Result<FearAndGreedIndex, FngApiError> {
    let json = ureq::get(format!("{API_URL}?limit={amt_days_in_past}").as_str())
        .call()
        .map_err(|e| FngApiError::ApiError(Box::new(e)))?
        .into_string()
        .map_err(FngApiError::ParseResultError)?;

    serde_json::from_str(&json).map_err(FngApiError::ParseJsonError)
}
