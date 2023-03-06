use std::env;
use adempiere_backend_rs::middleware::{KeyValue, CreateEntityRequest, UpdateEntityRequest, DeleteEntityRequest, RunBusinessProcessRequest};
use adempiere_backend_rs::middleware::middleware_service_client::MiddlewareServiceClient;
use adempiere_backend_rs::models::documents::{EntityNewDocument, EntityResponse, EntityUpdateDocument, EntityDeleteDocument, RunProcessDocument, ProcessResponse};
use dotenv::dotenv;
use local_ip_address::local_ip;
use salvo::prelude::*;
extern crate serde_json;
use simple_logger::SimpleLogger;
use tonic::Request;
use tonic::transport::Channel;
use tonic::metadata::MetadataValue;

#[tokio::main]
async fn main() {
    dotenv().ok();
    SimpleLogger::new().env().init().unwrap();
    let host =  match env::var("HOST") {
        Ok(value) => value,
        Err(_) => {
            log::info!("Variable `HOST` Not found from enviroment, loaded from local IP");
            match local_ip() {
                Ok(value) => {
                    let mut address = value.to_string();
                    address.push_str(":7878");
                    address
                },
                Err(_) => "127.0.0.1:7878".to_owned()
            }
        }.to_owned(),
    };
    let middleware_host = env::var("MIDDLEWARE_HOST");
    if middleware_host.is_err() {
        log::info!("Middleware Host not found");
        return;  
    }
    log::info!("Server Address: {:?}", host.clone());
    let router = Router::new()
        .push(
            Router::with_path("entities")
                .post(create_entity)
                .patch(update_entity)
                .delete(delete_entity)
        )
        .push(
            Router::with_path("process")
                .post(run_process)
        )
        ;
    log::info!("{:#?}", router);
    let acceptor = TcpListener::bind(&host);
    Server::new(acceptor).serve(router).await
}

#[handler]
async fn create_entity<'a>(_req: &mut salvo::Request, _document: EntityNewDocument, _res: &mut Response) {
    let _entity = _document.entity;
    if _entity.is_none() {
       return _res.render("Entity Is Mandatory");
    }
    let _entity = _entity.unwrap();
    if _entity.table_name.is_none() {
        return _res.render("Table Is Mandatory");
    }
    if _entity.attributes.is_none() {
        return _res.render("Attributes are Mandatory");
    }
    let token_value = _req.header::<String>("authorization");
    if token_value.is_none() {
        return _res.set_status_code(StatusCode::FORBIDDEN);
    }
    let token_value = token_value.unwrap();
    let attributes: Vec<KeyValue> = _entity.attributes.unwrap().iter().map(|value| value.to_owned().to_grpc_value()).collect();
    let middleware_host = env::var("MIDDLEWARE_HOST").unwrap().clone();();
    let channel = Channel::from_shared(middleware_host).unwrap().connect().await.unwrap();
    let token: MetadataValue<_> =  token_value.parse().unwrap();
    let mut client = MiddlewareServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    match client.create_entity(tonic::Request::new(CreateEntityRequest {
        table_name: _entity.table_name.unwrap(),
        attributes: attributes
    })).await {
        Ok(response) => {
            let entity = response.get_ref();
            log::info!("{}", entity.to_owned().id);
            _res.render(Json(EntityResponse::from_entity(entity.to_owned())));
        }, 
        Err(error) => {
            log::warn!("{}", error);
            _res.set_status_error(StatusError::internal_server_error());
            _res.render(Json(format!("{}", error.message())));
        }
    }
}

#[handler]
async fn update_entity<'a>(_req: &mut salvo::Request, _document: EntityUpdateDocument, _res: &mut Response) {
    let _entity = _document.entity;
    if _entity.is_none() {
       return _res.render("Entity Is Mandatory");
    }
    let _entity = _entity.unwrap();
    if _entity.table_name.is_none() {
        return _res.render("Table Is Mandatory");
    }
    if _entity.attributes.is_none() {
        return _res.render("Attributes are Mandatory");
    }
    if _entity.id.is_none() {
        return _res.render("ID is Mandatory");
    }
    let token_value = _req.header::<String>("authorization");
    if token_value.is_none() {
        return _res.set_status_code(StatusCode::FORBIDDEN);
    }
    let token_value = token_value.unwrap();
    let attributes: Vec<KeyValue> = _entity.attributes.unwrap().iter().map(|value| value.to_owned().to_grpc_value()).collect();
    let middleware_host = env::var("MIDDLEWARE_HOST").unwrap().clone();();
    let channel = Channel::from_shared(middleware_host).unwrap().connect().await.unwrap();
    let token: MetadataValue<_> =  token_value.parse().unwrap();
    let mut client = MiddlewareServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    match client.update_entity(tonic::Request::new(UpdateEntityRequest {
        table_name: _entity.table_name.unwrap(),
        id: _entity.id.unwrap(),
        attributes: attributes
    })).await {
        Ok(response) => {
            let entity = response.get_ref();
            log::info!("{}", entity.to_owned().id);
            _res.render(Json(EntityResponse::from_entity(entity.to_owned())));
        }, 
        Err(error) => {
            log::warn!("{}", error);
            _res.set_status_error(StatusError::internal_server_error());
            _res.render(Json(format!("{}", error.message())));
        }
    }
}

#[handler]
async fn delete_entity<'a>(_req: &mut salvo::Request, _document: EntityDeleteDocument, _res: &mut Response) {
    let _entity = _document.entity;
    if _entity.is_none() {
       return _res.render("Entity Is Mandatory");
    }
    let _entity = _entity.unwrap();
    if _entity.table_name.is_none() {
        return _res.render("Table Is Mandatory");
    }
    if _entity.id.is_none() {
        return _res.render("ID is Mandatory");
    }
    let token_value = _req.header::<String>("authorization");
    if token_value.is_none() {
        return _res.set_status_code(StatusCode::FORBIDDEN);
    }
    let token_value = token_value.unwrap();
    let middleware_host = env::var("MIDDLEWARE_HOST").unwrap().clone();();
    let channel = Channel::from_shared(middleware_host).unwrap().connect().await.unwrap();
    let token: MetadataValue<_> =  token_value.parse().unwrap();
    let mut client = MiddlewareServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    match client.delete_entity(tonic::Request::new(DeleteEntityRequest {
        table_name: _entity.table_name.unwrap(),
        id: _entity.id.unwrap()
    })).await {
        Ok(_) => {
            log::info!("{}", _entity.id.unwrap());
            _res.render(Json("Ok"));
        }, 
        Err(error) => {
            log::warn!("{}", error);
            _res.set_status_error(StatusError::internal_server_error());
            _res.render(Json(format!("{}", error.message())));
        }
    }
}
#[handler]
async fn run_process<'a>(_req: &mut salvo::Request, _document: RunProcessDocument, _res: &mut Response) {
    let _process = _document.process;
    if _process.is_none() {
       return _res.render("Process Is Mandatory");
    }
    let _process = _process.unwrap();
    if _process.process_code.is_none() {
        return _res.render("Process Code is Mandatory");
    }
    let token_value = _req.header::<String>("authorization");
    if token_value.is_none() {
        return _res.set_status_code(StatusCode::FORBIDDEN);
    }
    let token_value = token_value.unwrap();
    let parameters: Vec<KeyValue> = _process.parameters.unwrap_or_default().iter().map(|value| value.to_owned().to_grpc_value()).collect();
    let middleware_host = env::var("MIDDLEWARE_HOST").unwrap().clone();();
    let channel = Channel::from_shared(middleware_host).unwrap().connect().await.unwrap();
    let token: MetadataValue<_> =  token_value.parse().unwrap();
    let mut client = MiddlewareServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });
    match client.run_business_process(tonic::Request::new(RunBusinessProcessRequest {
        process_code: _process.process_code.unwrap(),
        table_name: _process.table_name.unwrap_or_default(),
        id: _process.id.unwrap_or_default(),
        table_selected_id: _process.table_selected_id.unwrap_or_default(),
        parameters: parameters,
        ..Default::default()
    })).await {
        Ok(response) => {
            let process_respose = response.get_ref();
            log::info!("{:?}", process_respose.to_owned());
            _res.render(Json(ProcessResponse::from_process_response(process_respose.to_owned())));
        }, 
        Err(error) => {
            log::warn!("{}", error);
            _res.set_status_error(StatusError::internal_server_error());
            _res.render(Json(format!("{}", error.message())));
        }
    }
}