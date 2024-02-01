/*
 * Parseable Server (C) 2022 - 2024 Parseable, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use crate::handlers::http::otel::proto::common::v1::InstrumentationScope;
use crate::handlers::http::otel::proto::common::v1::KeyValue;
use crate::handlers::http::otel::proto::common::v1::Value;
use crate::handlers::http::otel::proto::resource::v1::Resource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// LogsData represents the logs data that can be stored in a persistent storage,
/// OR can be embedded by other protocols that transfer OTLP logs data but do not
/// implement the OTLP protocol.
///
/// The main difference between this message and collector protocol is that
/// in this message there will not be any "control" or "metadata" specific to
/// OTLP protocol.
///
/// When new fields are added into this message, the OTLP request MUST be updated
/// as well.
pub struct LogsData {
    /// An array of ResourceLogs.
    /// For data coming from a single resource this array will typically contain
    /// one element. Intermediary nodes that receive data from multiple origins
    /// typically batch the data before forwarding further and in that case this
    /// array will contain multiple elements.
    #[serde(rename = "resourceLogs")]
    pub resource_logs: Option<Vec<ResourceLogs>>,
}

#[derive(Serialize, Deserialize, Debug)]
/// A collection of ScopeLogs from a Resource.
pub struct ResourceLogs {
    /// The resource for the logs in this message.
    /// If this field is not set then resource info is unknown.
    pub resource: Option<Resource>,
    /// A list of ScopeLogs that originate from a resource.
    #[serde(rename = "scopeLogs")]
    pub scope_logs: Option<Vec<ScopeLogs>>,
    /// This schema_url applies to the data in the "resource" field. It does not apply
    /// to the data in the "scope_logs" field which have their own schema_url field.
    #[serde(rename = "schemaUrl")]
    pub schema_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// A collection of Logs produced by a Scope.
pub struct ScopeLogs {
    /// The instrumentation scope information for the logs in this message.
    /// Semantically when InstrumentationScope isn't set, it is equivalent with
    /// an empty instrumentation scope name (unknown).
    pub scope: Option<InstrumentationScope>,
    /// A list of log records.
    #[serde(rename = "logRecords")]
    pub log_records: Vec<LogRecord>,
    /// This schema_url applies to all logs in the "logs" field.
    #[serde(rename = "schemaUrl")]
    pub schema_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
/// A log record according to OpenTelemetry Log Data Model:
/// <https://github.com/open-telemetry/oteps/blob/main/text/logs/0097-log-data-model.md>
pub struct LogRecord {
    /// time_unix_nano is the time when the event occurred.
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    /// Value of 0 indicates unknown or missing timestamp.
    #[serde(rename = "timeUnixNano")]
    pub time_unix_nano: Option<String>,
    /// Time when the event was observed by the collection system.
    /// For events that originate in OpenTelemetry (e.g. using OpenTelemetry Logging SDK)
    /// this timestamp is typically set at the generation time and is equal to Timestamp.
    /// For events originating externally and collected by OpenTelemetry (e.g. using
    /// Collector) this is the time when OpenTelemetry's code observed the event measured
    /// by the clock of the OpenTelemetry code. This field MUST be set once the event is
    /// observed by OpenTelemetry.
    ///
    /// For converting OpenTelemetry log data to formats that support only one timestamp or
    /// when receiving OpenTelemetry log data by recipients that support only one timestamp
    /// internally the following logic is recommended:
    ///    - Use time_unix_nano if it is present, otherwise use observed_time_unix_nano.
    ///
    /// Value is UNIX Epoch time in nanoseconds since 00:00:00 UTC on 1 January 1970.
    /// Value of 0 indicates unknown or missing timestamp.
    #[serde(rename = "observedTimeUnixNano")]
    pub observed_time_unix_nano: Option<String>,
    /// Numerical value of the severity, normalized to values described in Log Data Model.
    /// \[Optional\].
    #[serde(rename = "severityNumber")]
    pub severity_number: Option<i32>,
    /// The severity text (also known as log level). The original string representation as
    /// it is known at the source. \[Optional\].
    #[serde(rename = "severityText")]
    pub severity_text: Option<String>,
    pub name: Option<String>,
    /// A value containing the body of the log record. Can be for example a human-readable
    /// string message (including multi-line) describing the event in a free form or it can
    /// be a structured data composed of arrays and maps of other values. \[Optional\].
    pub body: Option<Value>,
    /// Additional attributes that describe the specific event occurrence. \[Optional\].
    /// Attribute keys MUST be unique (it is not allowed to have more than one
    /// attribute with the same key).
    pub attributes: Option<Vec<KeyValue>>,
    #[serde(rename = "droppedAttributesCount")]
    pub dropped_attributes_count: Option<u32>,
    /// Flags, a bit field. 8 least significant bits are the trace flags as
    /// defined in W3C Trace Context specification. 24 most significant bits are reserved
    /// and must be set to 0. Readers must not assume that 24 most significant bits
    /// will be zero and must correctly mask the bits when reading 8-bit trace flag (use
    /// flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK). \[Optional\].
    pub flags: Option<u32>,
    /// A unique identifier for a trace. All logs from the same trace share
    /// the same `trace_id`. The ID is a 16-byte array. An ID with all zeroes OR
    /// of length other than 16 bytes is considered invalid (empty string in OTLP/JSON
    /// is zero-length and thus is also invalid).
    ///
    /// This field is optional.
    ///
    /// The receivers SHOULD assume that the log record is not associated with a
    /// trace if any of the following is true:
    ///    - the field is not present,
    ///    - the field contains an invalid value.
    #[serde(rename = "traceId")]
    pub trace_id: Option<String>,
    /// A unique identifier for a span within a trace, assigned when the span
    /// is created. The ID is an 8-byte array. An ID with all zeroes OR of length
    /// other than 8 bytes is considered invalid (empty string in OTLP/JSON
    /// is zero-length and thus is also invalid).
    ///
    /// This field is optional. If the sender specifies a valid span_id then it SHOULD also
    /// specify a valid trace_id.
    ///
    /// The receivers SHOULD assume that the log record is not associated with a
    /// span if any of the following is true:
    ///    - the field is not present,
    ///    - the field contains an invalid value.
    #[serde(rename = "spanId")]
    pub span_id: Option<String>,
}
/// Possible values for LogRecord.SeverityNumber.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SeverityNumber {
    /// UNSPECIFIED is the default SeverityNumber, it MUST NOT be used.
    Unspecified = 0,
    Trace = 1,
    Trace2 = 2,
    Trace3 = 3,
    Trace4 = 4,
    Debug = 5,
    Debug2 = 6,
    Debug3 = 7,
    Debug4 = 8,
    Info = 9,
    Info2 = 10,
    Info3 = 11,
    Info4 = 12,
    Warn = 13,
    Warn2 = 14,
    Warn3 = 15,
    Warn4 = 16,
    Error = 17,
    Error2 = 18,
    Error3 = 19,
    Error4 = 20,
    Fatal = 21,
    Fatal2 = 22,
    Fatal3 = 23,
    Fatal4 = 24,
}
impl SeverityNumber {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(severity_number: i32) -> &'static str {
        match severity_number {
            0 => "SEVERITY_NUMBER_UNSPECIFIED",
            1 => "SEVERITY_NUMBER_TRACE",
            2 => "SEVERITY_NUMBER_TRACE2",
            3 => "SEVERITY_NUMBER_TRACE3",
            4 => "SEVERITY_NUMBER_TRACE4",
            5 => "SEVERITY_NUMBER_DEBUG",
            6 => "SEVERITY_NUMBER_DEBUG2",
            7 => "SEVERITY_NUMBER_DEBUG3",
            8 => "SEVERITY_NUMBER_DEBUG4",
            9 => "SEVERITY_NUMBER_INFO",
            10 => "SEVERITY_NUMBER_INFO2",
            11 => "SEVERITY_NUMBER_INFO3",
            12 => "SEVERITY_NUMBER_INFO4",
            13 => "SEVERITY_NUMBER_WARN",
            14 => "SEVERITY_NUMBER_WARN2",
            15 => "SEVERITY_NUMBER_WARN3",
            16 => "SEVERITY_NUMBER_WARN4",
            17 => "SEVERITY_NUMBER_ERROR",
            18 => "SEVERITY_NUMBER_ERROR2",
            19 => "SEVERITY_NUMBER_ERROR3",
            20 => "SEVERITY_NUMBER_ERROR4",
            21 => "SEVERITY_NUMBER_FATAL",
            22 => "SEVERITY_NUMBER_FATAL2",
            23 => "SEVERITY_NUMBER_FATAL3",
            24 => "SEVERITY_NUMBER_FATAL4",
            _ => "Invalid severity number",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SEVERITY_NUMBER_UNSPECIFIED" => Some(Self::Unspecified),
            "SEVERITY_NUMBER_TRACE" => Some(Self::Trace),
            "SEVERITY_NUMBER_TRACE2" => Some(Self::Trace2),
            "SEVERITY_NUMBER_TRACE3" => Some(Self::Trace3),
            "SEVERITY_NUMBER_TRACE4" => Some(Self::Trace4),
            "SEVERITY_NUMBER_DEBUG" => Some(Self::Debug),
            "SEVERITY_NUMBER_DEBUG2" => Some(Self::Debug2),
            "SEVERITY_NUMBER_DEBUG3" => Some(Self::Debug3),
            "SEVERITY_NUMBER_DEBUG4" => Some(Self::Debug4),
            "SEVERITY_NUMBER_INFO" => Some(Self::Info),
            "SEVERITY_NUMBER_INFO2" => Some(Self::Info2),
            "SEVERITY_NUMBER_INFO3" => Some(Self::Info3),
            "SEVERITY_NUMBER_INFO4" => Some(Self::Info4),
            "SEVERITY_NUMBER_WARN" => Some(Self::Warn),
            "SEVERITY_NUMBER_WARN2" => Some(Self::Warn2),
            "SEVERITY_NUMBER_WARN3" => Some(Self::Warn3),
            "SEVERITY_NUMBER_WARN4" => Some(Self::Warn4),
            "SEVERITY_NUMBER_ERROR" => Some(Self::Error),
            "SEVERITY_NUMBER_ERROR2" => Some(Self::Error2),
            "SEVERITY_NUMBER_ERROR3" => Some(Self::Error3),
            "SEVERITY_NUMBER_ERROR4" => Some(Self::Error4),
            "SEVERITY_NUMBER_FATAL" => Some(Self::Fatal),
            "SEVERITY_NUMBER_FATAL2" => Some(Self::Fatal2),
            "SEVERITY_NUMBER_FATAL3" => Some(Self::Fatal3),
            "SEVERITY_NUMBER_FATAL4" => Some(Self::Fatal4),
            _ => None,
        }
    }
}
/// LogRecordFlags is defined as a protobuf 'uint32' type and is to be used as
/// bit-fields. Each non-zero value defined in this enum is a bit-mask.
/// To extract the bit-field, for example, use an expression like:
///
///    (logRecord.flags & LOG_RECORD_FLAGS_TRACE_FLAGS_MASK)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogRecordFlags {
    /// The zero value for the enum. Should not be used for comparisons.
    /// Instead use bitwise "and" with the appropriate mask as shown above.
    DoNotUse = 0,
    /// Bits 0-7 are used for trace flags.
    TraceFlagsMask = 255,
}
impl LogRecordFlags {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(flag: u32) -> &'static str {
        match flag {
            0 => "LOG_RECORD_FLAGS_DO_NOT_USE",
            255 => "LOG_RECORD_FLAGS_TRACE_FLAGS_MASK",
            _ => "Invalid flag",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOG_RECORD_FLAGS_DO_NOT_USE" => Some(Self::DoNotUse),
            "LOG_RECORD_FLAGS_TRACE_FLAGS_MASK" => Some(Self::TraceFlagsMask),
            _ => None,
        }
    }
}
