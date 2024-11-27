use serde::{Serialize, Deserialize};
use crate::source::{SplitId, SplitMetaData};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash)]
pub struct AmqpSplit {}

impl SplitMetaData for AmqpSplit {
    fn id(&self) -> crate::source::SplitId {
        todo!()
    }

    fn encode_to_json(&self) -> risingwave_common::types::JsonbVal {
        serde_json::to_value(self.clone()).unwrap().into()
    }

    fn restore_from_json(value: risingwave_common::types::JsonbVal) -> crate::error::ConnectorResult<Self> {
        serde_json::from_value(value.take()).map_err(Into::into)
    }

    fn update_offset(&mut self, last_seen_offset: String) -> crate::error::ConnectorResult<()> {
        Ok(())
    }
}
