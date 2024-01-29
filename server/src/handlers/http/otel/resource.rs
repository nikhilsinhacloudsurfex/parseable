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

use crate::handlers::http::otel::proto::common::v1::KeyValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Resource information.
pub struct Resource {
    /// Set of attributes that describe the resource.
    /// Attribute keys MUST be unique (it is not allowed to have more than one
    /// attribute with the same key).
    #[serde(rename = "attributes")]
    pub attributes: Option<Vec<KeyValue>>,
    /// dropped_attributes_count is the number of dropped attributes. If the value is 0, then
    /// no attributes were dropped.

    #[serde(rename = "droppedAttributesCount")]
    pub dropped_attributes_count: Option<u32>,
}
