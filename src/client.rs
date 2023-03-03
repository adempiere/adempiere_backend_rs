use tonic::Request;
use tonic::transport::Channel;
use tonic::metadata::MetadataValue;
use hmac::{Hmac, Mac};
use sha2::{Sha256};
use std::collections::BTreeMap;
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use uuid::Uuid;

use crate::middleware::{CreateEntityRequest, KeyValue, Value, ValueType, Entity, DeleteEntityRequest};
use crate::middleware::middleware_service_client::MiddlewareServiceClient;

pub mod middleware {
    tonic::include_proto!("service");
}

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = HmacSha256::new_from_slice(b"ba54a050aaf4a9cfc619a31afbb03d212b5024a9957fa8b069a8c1b742de8c878846244f9c4b6834")?;
    let mut claims = BTreeMap::new();
    claims.insert("sub", "MiddlewareClient");

    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };
    let token_str = Token::new(header, claims).sign_with_key(&key)?;
    let channel = Channel::from_static("http://[::1]:50059").connect().await?;

    let mut token_value = String::from("Bearer ");
    token_value.push_str(&token_str.as_str());
    println!("{}", token_value);
    let token: MetadataValue<_> =  token_value.parse()?;
    let mut client = MiddlewareServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    let mut value: Vec<i32> = [].to_vec();
    for index in 1..1500 {
        let attributes = [
            KeyValue {
                key: "Value".to_string(),
                value: Some(Value {
                    string_value: format!("{:?}", Uuid::new_v4()).to_owned(),
                    value_type: ValueType::String.into(),
                    ..Default::default()
                })
            },
            KeyValue {
                key: "Name".to_string(),
                value: Some(Value {
                    string_value: format!("Test for gRPC (From Rust) {:?}", index).to_owned(),
                    value_type: ValueType::String.into(),
                    ..Default::default()
                })
            },
            KeyValue {
                key: "Description".to_string(),
                value: Some(Value {
                    string_value: format!("This is a test based on gRPC {:?}", index).to_owned(),
                    value_type: ValueType::String.into(),
                    ..Default::default()
                })
            },
            KeyValue {
                key: "IsDefault".to_string(),
                value: Some(Value {
                    boolean_value: false,
                    value_type: ValueType::Boolean.into(),
                    ..Default::default()
                })
            }
        ];
        let request = tonic::Request::new(CreateEntityRequest {
            table_name: "M_Product_Class".into(),
            attributes: attributes.to_vec()
        });

        let response: tonic::Response<Entity> = client.create_entity(request).await?;
        value.push(response.get_ref().id);
        println!("Entity Created {:?}", response);
    }

    for index in 1..value.len() {
        let request = tonic::Request::new(DeleteEntityRequest {
            table_name: "M_Product_Class".into(),
            id: index as i32
        });
        client.delete_entity(request).await?;
        println!("Entity Deleted {}", index);
    }

    Ok(())
}