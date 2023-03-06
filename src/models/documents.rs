use serde::{Deserialize, Serialize};
use salvo::prelude::*;

use crate::middleware::{KeyValue, Value, ValueType, Entity};

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

#[derive(Deserialize, Extractible, Debug, Clone)]
pub struct KeyAndValue {
    pub key: String,
    pub int_value: Option<i32>,
    pub boolean_value: Option<bool>,
    pub string_value: Option<String>,
    pub decimal_value: Option<f64>,
    pub value_type: Option<String> 
}

impl KeyAndValue {
    pub fn to_grpc_value(self) -> KeyValue {
        let mut value = Value {
            ..Default::default()
        };
        match self.value_type {
            Some(value_type) => {
                if value_type == "INTEGER" {
                    value.int_value = self.int_value.unwrap_or_default();
                    value.value_type = ValueType::Integer.to_owned().into();
                } else if value_type == "DECIMAL" {

                } else if value_type == "BOOLEAN" {
                    value.boolean_value = self.boolean_value.unwrap_or_default();
                    value.value_type = ValueType::Boolean.to_owned().into();
                } else if value_type == "STRING" {
                    value.string_value = self.string_value.unwrap_or_default();
                    value.value_type = ValueType::String.to_owned().into();
                } else if value_type == "DATE" {
                    value.string_value = self.string_value.unwrap_or_default();
                    value.value_type = ValueType::Date.to_owned().into();
                }
            },
            None => {}
        };
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