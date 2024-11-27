use async_trait::async_trait;

use super::super::{AmqpProperties, AmqpSplit};
use crate::error::ConnectorResult;
use crate::parser::ParserConfig;
use crate::source::{BoxChunkSourceStream, Column, SourceContextRef, SplitReader};

pub struct AmqpSplitReader {}

#[async_trait]
impl SplitReader for AmqpSplitReader {
    type Properties = AmqpProperties;
    type Split = AmqpSplit;

    async fn new(
        properties: Self::Properties,
        state: Vec<Self::Split>,
        parser_config: ParserConfig,
        source_ctx: SourceContextRef,
        columns: Option<Vec<Column>>,
    ) -> ConnectorResult<Self> {
        todo!()
    }

    fn into_stream(self) -> BoxChunkSourceStream {
        todo!()
    }
}
