use serde::{Deserialize, Serialize};
use salvo::prelude::*;

use crate::middleware::{KeyValue, Value, ValueType, Entity, Decimal, RunBusinessProcessResponse};

#[derive(Serialize, Debug, Clone)]
pub struct EntityResponse {
    pub table_name: Option <String>,
    pub id: Option<i32>
}

impl Default for EntityResponse {
    fn default() -> Self {
        EntityResponse {
            table_name: None,
            id: None
        }
    }
}

impl EntityResponse {
    pub fn from_entity(source: Entity) -> Self {
        let mut entity_response = EntityResponse::default();
        entity_response.table_name = Some(source.table_name);
        entity_response.id = Some(source.id);
        entity_response
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ProcessLog {
    pub record_id: Option<i32>,
    pub log: Option<String>
}

#[derive(Serialize, Debug, Clone)]
pub struct ProcessResponse {
    pub id: Option<i32>,
    pub is_error: Option<bool>,
    pub summary: Option<String>,
    pub result_table_name: Option<String>,
    pub is_processing: Option<bool>,
    pub last_run: Option<String>,
    pub logs: Option<Vec<ProcessLog>>
}

impl Default for ProcessResponse {
    fn default() -> Self {
        ProcessResponse {
            id: None,
            is_error: None,
            summary: None,
            result_table_name: None,
            is_processing: None,
            last_run: None,
            logs: None
        }
    }
}

impl ProcessResponse {
    pub fn from_process_response(source: RunBusinessProcessResponse) -> Self {
        let mut process_response = ProcessResponse::default();
        process_response.id = Some(source.id);
        process_response.is_error = Some(source.is_error);
        process_response.summary = Some(source.summary);
        process_response.result_table_name = Some(source.result_table_name);
        process_response.is_processing = Some(source.is_processing);
        process_response.last_run = Some(source.last_run);
        process_response.logs = Some(source.logs.iter().map(|log_value| {
            ProcessLog {
                record_id: Some(log_value.to_owned().record_id),
                log: Some(log_value.to_owned().log)
            }
        }).collect());
        process_response
    }
}

#[derive(Deserialize, Extractible, Debug, Clone)]
pub struct KeyAndValue {
    pub key: String,
    pub int_value: Option<i32>,
    pub boolean_value: Option<bool>,
    pub string_value: Option<String>,
    pub date_value: Option<String>,
    pub decimal_value: Option<f64>,
    pub value_type: Option<String> 
}

impl KeyAndValue {
    pub fn to_grpc_value(self) -> KeyValue {
        let mut value = Value {
            ..Default::default()
        };
        if self.int_value.is_some() {
            value.int_value = self.int_value.unwrap_or_default();
            value.value_type = ValueType::Integer.to_owned().into();
        } else if self.decimal_value.is_some() {
            let numeric_value = self.decimal_value.unwrap();
            let string_value = numeric_value.to_string();
            let decimal_index = string_value.find(".");
            let precision = (string_value.len() - 1) - decimal_index.unwrap_or_default();
            value.decimal_value = Some(Decimal {
                decimal_value: numeric_value.to_string(),
                scale: precision as i32
            });
            value.value_type = ValueType::Decimal.to_owned().into();
        } else if self.boolean_value.is_some() {
            value.boolean_value = self.boolean_value.unwrap_or_default();
            value.value_type = ValueType::Boolean.to_owned().into();
        } else if self.string_value.is_some() {
            value.string_value = self.string_value.unwrap_or_default();
            value.value_type = ValueType::String.to_owned().into();
        } else if self.date_value.is_some() {
            value.date_value = self.date_value.unwrap_or_default();
            value.value_type = ValueType::Date.to_owned().into();
        }
        KeyValue {
            key: self.key, 
            value: Some(value) 
        }
    }
}

#[derive(Deserialize, Extractible, Debug, Clone)]
pub struct EntityNew {
    pub table_name: Option<String>,
    pub attributes: Option<Vec<KeyAndValue>>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct EntityNewDocument {
    pub entity: Option<EntityNew>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
pub struct EntityUpdate {
    pub table_name: Option<String>,
    pub id: Option<i32>,
    pub attributes: Option<Vec<KeyAndValue>>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct EntityUpdateDocument {
    pub entity: Option<EntityUpdate>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
pub struct EntityDelete {
    pub table_name: Option<String>,
    pub id: Option<i32>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct EntityDeleteDocument {
    pub entity: Option<EntityDelete>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
pub struct RunProcess {
    pub table_name: Option<String>,
    pub id: Option<i32>,
    pub process_code: Option<String>,
    pub table_selected_id: Option<i32>,
    pub parameters: Option<Vec<KeyAndValue>>
}

#[derive(Deserialize, Extractible, Debug, Clone)]
#[extract(default_source(from = "body", format = "json"))]
pub struct RunProcessDocument {
    pub process: Option<RunProcess>
}