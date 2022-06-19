use super::{common::Id, Request};
use crate::ws::Fill;
use chrono::{DateTime, Utc};
use http::Method;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetFills {
    pub limit: usize,
    pub market: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_as_timestamp"
    )]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serialize_as_timestamp"
    )]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Id>,
}

impl GetFills {
    pub fn new() -> Self {
        Self {
            limit: 20, // this is equal to that is used by the API if not explicitly provided.
            ..Self::default()
        }
    }

    pub fn for_market(market: &str) -> Self {
        let mut req = Self::new();
        req.market = Some(market.to_owned());
        req
    }
}

impl Request for GetFills {
    const METHOD: Method = Method::GET;
    const PATH: &'static str = "/fills";
    const AUTH: bool = true;

    type Response = Vec<Fill>;
}
