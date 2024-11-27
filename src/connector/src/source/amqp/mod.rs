pub mod enumerator;
pub mod source;
pub mod split;

use std::collections::HashMap;

use serde::Deserialize;
use split::AmqpSplit;
use with_options::WithOptions;

use crate::source::amqp::enumerator::AmqpSplitEnumerator;
use crate::source::amqp::source::AmqpSplitReader;
use crate::source::SourceProperties;

pub const AMQP_CONNECTOR: &str = "amqp";

#[derive(Clone, Debug, Deserialize, WithOptions)]
pub struct AmqpProperties {
    #[serde(rename = "amqp.url")]
    pub url: String,

    #[serde(rename = "amqp.split_count")]
    pub split_count: u32,

    pub unknown_fields: HashMap<String, String>,
}

impl SourceProperties for AmqpProperties {
    type Split = AmqpSplit;
    type SplitEnumerator = AmqpSplitEnumerator;
    type SplitReader = AmqpSplitReader;

    const SOURCE_NAME: &'static str = AMQP_CONNECTOR;
}

impl crate::source::UnknownFields for AmqpProperties {
    fn unknown_fields(&self) -> HashMap<String, String> {
        self.unknown_fields.clone()
    }
}
