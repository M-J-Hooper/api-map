use crate::{client::Client};
use serde_json::Value;

pub struct Context<'a> {
    pub client: &'a Client,
    pub value: &'a Value,
}