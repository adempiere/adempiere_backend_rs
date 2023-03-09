# ADempiere Backend Service from Rust
A Rest API developed with rust for ADempiere, this backend use **gRPC** for connect with **adempiere-middleware** and publish all endpoints using Rest API.

This backend was developed over [salvo.rs](https://salvo.rs/) as framework.

You are free of contribute with us for improve it.

## Requirements
- Just install [rust](https://www.rust-lang.org/tools/install) from page
- Protoc compiler [here](https://grpc.io/docs/protoc-installation/)
- [rust](https://www.rust-lang.org/)

```Shell
apt install -y protobuf-compiler
$ protoc --version  # Ensure compiler version is 3+
```

## Getting Started
This project ewas developed over [rust](https://www.rust-lang.org/) language and if you need run it just need run the follow commands

### Clone it
```Shell
git clone https://github.com/erpya/adempiere_backend_rs
```

### Go to folder
```Shell
cd adempiere_backend_rs
```

### Build Project
```Shell
cargo build
```

### Start Service
```Shell
cargo run --bin server
```

### Console Output

```Shell
2023-03-09T20:12:09.714Z INFO  [server] Server Address: "0.0.0.0:7878"
2023-03-09T20:12:09.714Z INFO  [server] └──!NULL!
    ├──entities
    │   ├──[POST] -> server::create_entity
    │   ├──[PATCH] -> server::update_entity
    │   └──[DELETE] -> server::delete_entity
    └──process
        └──[POST] -> server::run_process
```

### Deploy with docker

You can build images using the follow command

```
docker build -t adempiere-backend-rs -f docker/Dockerfile .
```

After build just run it

```
docker run -d -p 7878:7878 --name adempiere-backend -e MIDDLEWARE_HOST=0.0.0.0:50051 adempiere-backend-rs
```

### Setup a new Entity

The follow is a curl calling for create a Device

- Table Name: **M_Product_Class**
- Value: **0d005e89-42e0-4dd0-bbb7-af6b4212da6c**
- Name: **Test from Rest API**
- IsDefault: **false**
- Description: **Test**

```Shell
curl --location '0.0.0.0:7878/entities' \
--header 'Authorization: Bearer <Token>' \
--header 'Content-Type: application/json' \
--data '{
    "entity": {
        "table_name": "M_Product_Class",
        "attributes": [
            {
                "key": "Value",
                "string_value": "0d005e89-42e0-4dd0-bbb7-af6b4212da6c",
                "value_type": "STRING"
            },
            {
                "key": "Name",
                "string_value": "Test from Rest API",
                "value_type": "STRING"
            },
            {
                "key": "IsDefault",
                "boolean_value": false,
                "value_type": "BOOLEAN"
            },
            {
                "key": "Description",
                "string_value": "Test",
                "value_type": "STRING"
            }
        ]
    }
}'
```
See all endpoints [here](https://documenter.getpostman.com/view/18440575/2s93JnW7XM)