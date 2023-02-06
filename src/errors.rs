use thiserror::Error;

#[derive(Error, Debug)]
pub enum FngApiError {
    #[error("Calling the API failed.")]
    ApiError(#[from] Box<ureq::Error>),
    #[error("Parsing the API response failed.")]
    ParseResultError(#[from] std::io::Error),
    #[error("Parsing the JSON response failed.")]
    ParseJsonError(#[from] serde_json::Error),
    #[error("Received records for fewer days than requested. This is likely because you a wider time frame than the API provides. Consider calling fetch_daily_fng_max_records instead of fetch_daily_fng(amt_days_in_past).")]
    ReceivedLessRecords,
    #[error("Received records for more days than requested. This is unexpected behavior. Please report this bug.")]
    ReceivedMoreRecords,
}
