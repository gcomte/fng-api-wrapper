use chrono::prelude::*;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum ValueClassification {
    #[serde(rename(deserialize = "Extreme Fear"))]
    ExtremeFear,
    Fear,
    Neutral,
    Greed,
    #[serde(rename(deserialize = "Extreme Greed"))]
    ExtremeGreed,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct FearAndGreedIndex {
    pub name: String,
    pub data: Vec<Data>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Data {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: u8,
    pub value_classification: ValueClassification,
    #[serde(deserialize_with = "deserialize_datetime_utc_from_seconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub time_until_update: Option<u16>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub error: Option<String>,
}
