use serde::{Deserialize, Serialize};

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
    pub value: String,
    pub value_classification: ValueClassification,
    pub timestamp: String,
    pub time_until_update: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub error: Option<String>,
}
