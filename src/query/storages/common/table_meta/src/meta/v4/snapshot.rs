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

use std::io::Cursor;
use std::io::Read;

use chrono::DateTime;
use chrono::Utc;
use databend_common_base::base::uuid::Uuid;
use databend_common_exception::Result;
use databend_common_expression::TableSchema;
use databend_common_io::prelude::BinaryRead;
use serde::Deserialize;
use serde::Serialize;

use crate::meta::format::compress;
use crate::meta::format::encode;
use crate::meta::format::read_and_deserialize;
use crate::meta::format::MetaCompression;
use crate::meta::monotonically_increased_timestamp;
use crate::meta::trim_timestamp_to_micro_second;
use crate::meta::v2;
use crate::meta::v3;
use crate::meta::ClusterKey;
use crate::meta::FormatVersion;
use crate::meta::Location;
use crate::meta::MetaEncoding;
use crate::meta::SnapshotId;
use crate::meta::Statistics;
use crate::meta::Versioned;

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for TableSnapshot {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where __D: _serde::Deserializer<'de> {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;

            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        9u64 => _serde::__private::Ok(__Field::__field9),
                        10u64 => _serde::__private::Ok(__Field::__field10),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "format_version" => _serde::__private::Ok(__Field::__field0),
                        "snapshot_id" => _serde::__private::Ok(__Field::__field1),
                        "timestamp" => _serde::__private::Ok(__Field::__field2),
                        "prev_table_seq" => _serde::__private::Ok(__Field::__field3),
                        "prev_snapshot_id" => _serde::__private::Ok(__Field::__field4),
                        "schema" => _serde::__private::Ok(__Field::__field5),
                        "summary" => _serde::__private::Ok(__Field::__field6),
                        "segments" => _serde::__private::Ok(__Field::__field7),
                        "cluster_key_meta" => _serde::__private::Ok(__Field::__field8),
                        "table_statistics_location" => _serde::__private::Ok(__Field::__field9),
                        "least_visible_timestamp" => _serde::__private::Ok(__Field::__field10),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"format_version" => _serde::__private::Ok(__Field::__field0),
                        b"snapshot_id" => _serde::__private::Ok(__Field::__field1),
                        b"timestamp" => _serde::__private::Ok(__Field::__field2),
                        b"prev_table_seq" => _serde::__private::Ok(__Field::__field3),
                        b"prev_snapshot_id" => _serde::__private::Ok(__Field::__field4),
                        b"schema" => _serde::__private::Ok(__Field::__field5),
                        b"summary" => _serde::__private::Ok(__Field::__field6),
                        b"segments" => _serde::__private::Ok(__Field::__field7),
                        b"cluster_key_meta" => _serde::__private::Ok(__Field::__field8),
                        b"table_statistics_location" => _serde::__private::Ok(__Field::__field9),
                        b"least_visible_timestamp" => _serde::__private::Ok(__Field::__field10),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where __D: _serde::Deserializer<'de> {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TableSnapshot>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TableSnapshot;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TableSnapshot")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match _serde::de::SeqAccess::next_element::<FormatVersion>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct TableSnapshot with 11 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match _serde::de::SeqAccess::next_element::<SnapshotId>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct TableSnapshot with 11 elements",
                                ));
                            }
                        };
                    let __field2 = match _serde::de::SeqAccess::next_element::<Option<DateTime<Utc>>>(
                        &mut __seq,
                    )? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct TableSnapshot with 11 elements",
                            ));
                        }
                    };
                    let __field3 =
                        match _serde::de::SeqAccess::next_element::<Option<u64>>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct TableSnapshot with 11 elements",
                                ));
                            }
                        };
                    let __field4 = match _serde::de::SeqAccess::next_element::<
                        Option<(SnapshotId, FormatVersion)>,
                    >(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct TableSnapshot with 11 elements",
                            ));
                        }
                    };
                    let __field5 =
                        match _serde::de::SeqAccess::next_element::<TableSchema>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct TableSnapshot with 11 elements",
                                ));
                            }
                        };
                    let __field6 =
                        match _serde::de::SeqAccess::next_element::<Statistics>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct TableSnapshot with 11 elements",
                                ));
                            }
                        };
                    let __field7 =
                        match _serde::de::SeqAccess::next_element::<Vec<Location>>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct TableSnapshot with 11 elements",
                                ));
                            }
                        };
                    let __field8 = match _serde::de::SeqAccess::next_element::<Option<ClusterKey>>(
                        &mut __seq,
                    )? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                8usize,
                                &"struct TableSnapshot with 11 elements",
                            ));
                        }
                    };
                    let __field9 =
                        match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    9usize,
                                    &"struct TableSnapshot with 11 elements",
                                ));
                            }
                        };
                    let __field10 = match _serde::de::SeqAccess::next_element::<
                        Option<DateTime<Utc>>,
                    >(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                10usize,
                                &"struct TableSnapshot with 11 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(TableSnapshot {
                        format_version: __field0,
                        snapshot_id: __field1,
                        timestamp: __field2,
                        prev_table_seq: __field3,
                        prev_snapshot_id: __field4,
                        schema: __field5,
                        summary: __field6,
                        segments: __field7,
                        cluster_key_meta: __field8,
                        table_statistics_location: __field9,
                        least_visible_timestamp: __field10,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<FormatVersion> =
                        _serde::__private::None;
                    let mut __field1: _serde::__private::Option<SnapshotId> =
                        _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Option<DateTime<Utc>>> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<Option<u64>> =
                        _serde::__private::None;
                    let mut __field4: _serde::__private::Option<
                        Option<(SnapshotId, FormatVersion)>,
                    > = _serde::__private::None;
                    let mut __field5: _serde::__private::Option<TableSchema> =
                        _serde::__private::None;
                    let mut __field6: _serde::__private::Option<Statistics> =
                        _serde::__private::None;
                    let mut __field7: _serde::__private::Option<Vec<Location>> =
                        _serde::__private::None;
                    let mut __field8: _serde::__private::Option<Option<ClusterKey>> =
                        _serde::__private::None;
                    let mut __field9: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field10: _serde::__private::Option<Option<DateTime<Utc>>> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                let value =
                                    _serde::de::MapAccess::next_value::<FormatVersion>(&mut __map);
                                eprintln!("format_version: {:?}", value);
                            }
                            __Field::__field1 => {
                                let value =
                                    _serde::de::MapAccess::next_value::<SnapshotId>(&mut __map);
                                eprintln!("snapshot_id: {:?}", value);
                            }
                            __Field::__field2 => {
                                let value = _serde::de::MapAccess::next_value::<
                                    Option<DateTime<Utc>>,
                                >(&mut __map);
                                eprintln!("timestamp: {:?}", value);
                            }
                            __Field::__field3 => {
                                let value =
                                    _serde::de::MapAccess::next_value::<Option<u64>>(&mut __map);
                                eprintln!("prev_table_seq: {:?}", value);
                            }
                            __Field::__field4 => {
                                let value = _serde::de::MapAccess::next_value::<
                                    Option<(SnapshotId, FormatVersion)>,
                                >(&mut __map);
                                eprintln!("prev_snapshot_id: {:?}", value);
                            }
                            __Field::__field5 => {
                                let value =
                                    _serde::de::MapAccess::next_value::<TableSchema>(&mut __map);
                                eprintln!("schema: {:?}", value);
                            }
                            __Field::__field6 => {
                                let value =
                                    _serde::de::MapAccess::next_value::<Statistics>(&mut __map);
                                eprintln!("summary: {:?}", value);
                            }
                            __Field::__field7 => {
                                let value =
                                    _serde::de::MapAccess::next_value::<Vec<Location>>(&mut __map);
                                eprintln!("segments: {:?}", value);
                            }
                            __Field::__field8 => {
                                let value = _serde::de::MapAccess::next_value::<Option<ClusterKey>>(
                                    &mut __map,
                                );
                                eprintln!("cluster_key_meta: {:?}", value);
                            }
                            __Field::__field9 => {
                                let value =
                                    _serde::de::MapAccess::next_value::<Option<String>>(&mut __map);
                                eprintln!("table_statistics_location: {:?}", value);
                            }
                            __Field::__field10 => {
                                let value = _serde::de::MapAccess::next_value::<
                                    Option<DateTime<Utc>>,
                                >(&mut __map);
                                eprintln!("least_visible_timestamp: {:?}", value);
                            }
                            _ => {
                                let value = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                                eprintln!("ignored: {:?}", value);
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => Default::default(),
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => Default::default(),
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => Default::default(),
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => Default::default(),
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => Default::default(),
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => Default::default(),
                    };
                    let __field6 = match __field6 {
                        _serde::__private::Some(__field6) => __field6,
                        _serde::__private::None => Default::default(),
                    };
                    let __field7 = match __field7 {
                        _serde::__private::Some(__field7) => __field7,
                        _serde::__private::None => Default::default(),
                    };
                    let __field8 = match __field8 {
                        _serde::__private::Some(__field8) => __field8,
                        _serde::__private::None => Default::default(),
                    };
                    let __field9 = match __field9 {
                        _serde::__private::Some(__field9) => __field9,
                        _serde::__private::None => Default::default(),
                    };
                    let __field10 = match __field10 {
                        _serde::__private::Some(__field10) => __field10,
                        _serde::__private::None => Default::default(),
                    };
                    _serde::__private::Ok(TableSnapshot {
                        format_version: __field0,
                        snapshot_id: __field1,
                        timestamp: __field2,
                        prev_table_seq: __field3,
                        prev_snapshot_id: __field4,
                        schema: __field5,
                        summary: __field6,
                        segments: __field7,
                        cluster_key_meta: __field8,
                        table_statistics_location: __field9,
                        least_visible_timestamp: __field10,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "format_version",
                "snapshot_id",
                "timestamp",
                "prev_table_seq",
                "prev_snapshot_id",
                "schema",
                "summary",
                "segments",
                "cluster_key_meta",
                "table_statistics_location",
                "least_visible_timestamp",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "TableSnapshot",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TableSnapshot>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

/// The structure of the TableSnapshot is the same as that of v2, but the serialization and deserialization methods are different
#[derive(Serialize, Clone, Debug)]
pub struct TableSnapshot {
    /// format version of TableSnapshot meta data
    ///
    /// Note that:
    ///
    /// - A instance of v3::TableSnapshot may have a value of v2/v1::TableSnapshot::VERSION for this field.
    ///
    ///   That indicates this instance is converted from a v2/v1::TableSnapshot.
    ///
    /// - The meta writers are responsible for only writing down the latest version of TableSnapshot, and
    ///   the format_version being written is of the latest version.
    ///
    ///   e.g. if the current version of TableSnapshot is v3::TableSnapshot, then the format_version
    ///   that will be written down to object storage as part of TableSnapshot table meta data,
    ///   should always be v3::TableSnapshot::VERSION (which is 3)
    pub format_version: FormatVersion,

    /// id of snapshot
    pub snapshot_id: SnapshotId,

    /// timestamp of this snapshot
    //  for backward compatibility, `Option` is used
    pub timestamp: Option<DateTime<Utc>>,

    // The table seq before snapshot commit.
    pub prev_table_seq: Option<u64>,

    /// previous snapshot
    pub prev_snapshot_id: Option<(SnapshotId, FormatVersion)>,

    /// For each snapshot, we keep a schema for it (in case of schema evolution)
    pub schema: TableSchema,

    /// Summary Statistics
    pub summary: Statistics,

    /// Pointers to SegmentInfos (may be of different format)
    ///
    /// We rely on background merge tasks to keep merging segments, so that
    /// this the size of this vector could be kept reasonable
    pub segments: Vec<Location>,

    /// The metadata of the cluster keys.
    pub cluster_key_meta: Option<ClusterKey>,
    pub table_statistics_location: Option<String>,

    pub least_visible_timestamp: Option<DateTime<Utc>>,
}

impl TableSnapshot {
    pub fn new(
        snapshot_id: SnapshotId,
        prev_table_seq: Option<u64>,
        prev_timestamp: &Option<DateTime<Utc>>,
        prev_snapshot_id: Option<(SnapshotId, FormatVersion)>,
        schema: TableSchema,
        summary: Statistics,
        segments: Vec<Location>,
        cluster_key_meta: Option<ClusterKey>,
        table_statistics_location: Option<String>,
    ) -> Self {
        let now = Utc::now();
        // make snapshot timestamp monotonically increased
        let adjusted_timestamp = monotonically_increased_timestamp(now, prev_timestamp);

        // trim timestamp to micro seconds
        let trimmed_timestamp = trim_timestamp_to_micro_second(adjusted_timestamp);
        let timestamp = Some(trimmed_timestamp);

        Self {
            format_version: TableSnapshot::VERSION,
            snapshot_id,
            timestamp,
            prev_table_seq,
            prev_snapshot_id,
            schema,
            summary,
            segments,
            cluster_key_meta,
            table_statistics_location,
            least_visible_timestamp: None,
        }
    }

    pub fn new_empty_snapshot(schema: TableSchema, prev_table_seq: Option<u64>) -> Self {
        Self::new(
            Uuid::new_v4(),
            prev_table_seq,
            &None,
            None,
            schema,
            Statistics::default(),
            vec![],
            None,
            None,
        )
    }

    pub fn from_previous(previous: &TableSnapshot, prev_table_seq: Option<u64>) -> Self {
        let id = Uuid::new_v4();
        let clone = previous.clone();
        // the timestamp of the new snapshot will be adjusted by the `new` method
        Self::new(
            id,
            prev_table_seq,
            &clone.timestamp,
            Some((clone.snapshot_id, clone.format_version)),
            clone.schema,
            clone.summary,
            clone.segments,
            clone.cluster_key_meta,
            clone.table_statistics_location,
        )
    }

    /// Serializes the struct to a byte vector.
    ///
    /// The byte vector contains the format version, encoding, compression, and compressed data. The encoding
    /// and compression are set to default values. The data is encoded and compressed.
    ///
    /// # Returns
    ///
    /// A Result containing the serialized data as a byte vector. If any errors occur during
    /// encoding, compression, or writing to the byte vector, an error will be returned.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let encoding = MetaEncoding::MessagePack;
        let compression = MetaCompression::default();

        let data = encode(&encoding, &self)?;
        let data_compress = compress(&compression, data)?;

        let data_size = self.format_version.to_le_bytes().len()
            + 2
            + data_compress.len().to_le_bytes().len()
            + data_compress.len();
        let mut buf = Vec::with_capacity(data_size);

        buf.extend_from_slice(&self.format_version.to_le_bytes());
        buf.push(encoding as u8);
        buf.push(compression as u8);
        buf.extend_from_slice(&data_compress.len().to_le_bytes());

        buf.extend(data_compress);

        Ok(buf)
    }

    /// Reads a snapshot from Vec<u8> and returns a `TableSnapshot` object.
    ///
    /// This function reads the following fields from the stream and constructs a `TableSnapshot` object:
    ///
    /// * `version` (u64): The version number of the snapshot.
    /// * `encoding` (u8): The encoding format used to serialize the snapshot's data.
    /// * `compression` (u8): The compression format used to compress the snapshot's data.
    /// * `snapshot_size` (u64): The size (in bytes) of the compressed snapshot data.
    ///
    /// The function then reads the compressed snapshot data from the stream, decompresses it using
    /// the specified compression format, and deserializes it using the specified encoding format.
    /// Finally, it constructs a `TableSnapshot` object using the deserialized data and returns it.
    pub fn from_slice(buffer: &[u8]) -> Result<TableSnapshot> {
        Self::from_read(Cursor::new(buffer))
    }

    pub fn from_read(mut r: impl Read) -> Result<TableSnapshot> {
        let version = r.read_scalar::<u64>()?;
        assert_eq!(version, TableSnapshot::VERSION);
        let encoding = MetaEncoding::try_from(r.read_scalar::<u8>()?)?;
        let compression = MetaCompression::try_from(r.read_scalar::<u8>()?)?;
        let snapshot_size: u64 = r.read_scalar::<u64>()?;

        read_and_deserialize(&mut r, snapshot_size, &encoding, &compression)
    }

    #[inline]
    pub fn encoding() -> MetaEncoding {
        MetaEncoding::MessagePack
    }
}

// use the chain of converters, for versions before v3
impl From<v2::TableSnapshot> for TableSnapshot {
    fn from(s: v2::TableSnapshot) -> Self {
        Self {
            // NOTE: it is important to let the format_version return from here
            // carries the format_version of snapshot being converted.
            format_version: s.format_version,
            snapshot_id: s.snapshot_id,
            timestamp: s.timestamp,
            prev_table_seq: None,
            prev_snapshot_id: s.prev_snapshot_id,
            schema: s.schema,
            summary: s.summary,
            segments: s.segments,
            cluster_key_meta: s.cluster_key_meta,
            table_statistics_location: s.table_statistics_location,
            least_visible_timestamp: None,
        }
    }
}

impl<T> From<T> for TableSnapshot
where T: Into<v3::TableSnapshot>
{
    fn from(s: T) -> Self {
        let s: v3::TableSnapshot = s.into();
        Self {
            // NOTE: it is important to let the format_version return from here
            // carries the format_version of snapshot being converted.
            format_version: s.format_version,
            snapshot_id: s.snapshot_id,
            timestamp: s.timestamp,
            prev_table_seq: None,
            prev_snapshot_id: s.prev_snapshot_id,
            schema: s.schema.into(),
            summary: s.summary.into(),
            segments: s.segments,
            cluster_key_meta: s.cluster_key_meta,
            table_statistics_location: s.table_statistics_location,
            least_visible_timestamp: None,
        }
    }
}

// A memory light version of TableSnapshot(Without segments)
// This *ONLY* used for some optimize operation, like PURGE/FUSE_SNAPSHOT function to avoid OOM.
#[derive(Clone, Debug)]
pub struct TableSnapshotLite {
    pub format_version: FormatVersion,
    pub snapshot_id: SnapshotId,
    pub timestamp: Option<DateTime<Utc>>,
    pub prev_snapshot_id: Option<(SnapshotId, FormatVersion)>,
    pub row_count: u64,
    pub block_count: u64,
    pub index_size: u64,
    pub uncompressed_byte_size: u64,
    pub compressed_byte_size: u64,
    pub segment_count: u64,
}

impl From<(&TableSnapshot, FormatVersion)> for TableSnapshotLite {
    fn from((value, ver): (&TableSnapshot, FormatVersion)) -> Self {
        TableSnapshotLite {
            format_version: ver,
            snapshot_id: value.snapshot_id,
            timestamp: value.timestamp,
            prev_snapshot_id: value.prev_snapshot_id,
            row_count: value.summary.row_count,
            block_count: value.summary.block_count,
            index_size: value.summary.index_size,
            uncompressed_byte_size: value.summary.uncompressed_byte_size,
            segment_count: value.segments.len() as u64,
            compressed_byte_size: value.summary.compressed_byte_size,
        }
    }
}
