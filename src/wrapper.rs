use chrono::prelude::*;
use serde::Deserialize;
use serde_aux::prelude::*;

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum ValueClassification {
    #[default]
    #[serde(rename(deserialize = "Extreme Fear"))]
    ExtremeFear,
    Fear,
    Neutral,
    Greed,
    #[serde(rename(deserialize = "Extreme Greed"))]
    ExtremeGreed,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct FearAndGreedIndex {
    pub name: String,
    pub data: Vec<Data>,
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Data {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: u8,
    pub value_classification: ValueClassification,
    #[serde(deserialize_with = "deserialize_datetime_utc_from_seconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
    pub time_until_update: Option<u32>,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct Metadata {
    pub error: Option<String>,
}

#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]
// if one of the variables of a type is not Send or Sync, that autotrait will not be derived for that type
fn ensure_autotraits_are_implemented_for_types() {
    is_normal::<Metadata>();
    is_normal::<Data>();
    is_normal::<ValueClassification>();
    is_normal::<FearAndGreedIndex>();
}
