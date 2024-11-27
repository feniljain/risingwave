use async_trait::async_trait;

use super::split::AmqpSplit;
use super::AmqpProperties;
use crate::error::ConnectorResult;
use crate::source::{SourceEnumeratorContextRef, SplitEnumerator};

pub struct AmqpSplitEnumerator {}

#[async_trait]
impl SplitEnumerator for AmqpSplitEnumerator {
    type Properties = AmqpProperties;
    type Split = AmqpSplit;

    async fn new(
        properties: Self::Properties,
        context: SourceEnumeratorContextRef,
    ) -> ConnectorResult<Self> {
        todo!()
    }

    async fn list_splits(&mut self) -> ConnectorResult<Vec<Self::Split>> {
        todo!()
    }
}
