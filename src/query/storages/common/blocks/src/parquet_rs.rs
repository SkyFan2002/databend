// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use databend_common_exception::Result;
use databend_common_expression::converts::arrow::table_schema_to_arrow_schema_ignore_inside_nullable;
use databend_common_expression::DataBlock;
use databend_common_expression::TableSchema;
use databend_storages_common_table_meta::meta::StatisticsOfColumns;
use databend_storages_common_table_meta::table::TableCompression;
use parquet_rs::arrow::arrow_to_parquet_schema;
use parquet_rs::arrow::ArrowWriter;
use parquet_rs::basic::Encoding;
use parquet_rs::file::properties::EnabledStatistics;
use parquet_rs::file::properties::WriterProperties;
use parquet_rs::format::FileMetaData;

use crate::codec::choose_codec;

/// Serialize data blocks to parquet format.
pub fn blocks_to_parquet(
    table_schema: &TableSchema,
    blocks: Vec<DataBlock>,
    write_buffer: &mut Vec<u8>,
    compression: TableCompression,
    stat: &StatisticsOfColumns,
) -> Result<FileMetaData> {
    assert!(!blocks.is_empty());
    let mut props_builder = WriterProperties::builder()
        .set_compression(compression.into())
        // use `usize::MAX` to effectively limit the number of row groups to 1
        .set_max_row_group_size(usize::MAX)
        // this is a global setting, will be covered by the column level setting(if set)
        .set_encoding(Encoding::PLAIN)
        // ditto
        .set_dictionary_enabled(false)
        .set_statistics_enabled(EnabledStatistics::None)
        .set_bloom_filter_enabled(false);
    let arrow_schema = Arc::new(table_schema_to_arrow_schema_ignore_inside_nullable(
        table_schema,
    ));
    let parquet_schema = arrow_to_parquet_schema(&arrow_schema)?;
    if blocks.len() == 1 {
        // doesn't not cover the case of multiple blocks for now. to simplify the implementation
        props_builder = choose_codec(
            props_builder,
            &blocks[0],
            parquet_schema.root_schema().get_fields(),
            table_schema,
            stat,
        )?;
    }
    let props = props_builder.build();
    let batches = blocks
        .into_iter()
        .map(|block| block.to_record_batch(table_schema))
        .collect::<Result<Vec<_>>>()?;

    let mut writer = ArrowWriter::try_new(write_buffer, arrow_schema, Some(props))?;
    for batch in batches {
        writer.write(&batch)?;
    }
    let file_meta = writer.close()?;
    Ok(file_meta)
}
