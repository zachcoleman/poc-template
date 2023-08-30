# poc-template

## Prerequisites
```sh
brew install make
brew install protobuf
brew install bufbuild/buf/buf
```

## Gateway Setup
```sh
cd gateway
go get \
    github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway \
    github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2 \
    google.golang.org/protobuf/cmd/protoc-gen-go \
    google.golang.org/grpc/cmd/protoc-gen-go-grpc
go install \
    github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-grpc-gateway \
    github.com/grpc-ecosystem/grpc-gateway/v2/protoc-gen-openapiv2 \
    google.golang.org/protobuf/cmd/protoc-gen-go \
    google.golang.org/grpc/cmd/protoc-gen-go-grpc
export GO_PATH=~/go
export PATH=$PATH:$GO_PATH/bin
```

## Run gRPC-Gateway
```sh
cd gateway && go run .
```

## Generate Protobuf Definitions and Code
For the Gateway and other `buf` tools:
```sh
cd proto && buf generate --path *.proto
```

For the Tonic client and server:
```sh
cd backend && cargo build
```

## Run Server:
```sh
cargo run --bin server
```

## Run Integration Tests (depends on running server)
```sh
cargo test
```

## Verify SQLite is Installed:
```sh
sqlite3 --version
```
Install if necessary:
```sh
brew install sqlite
```

## Install `sqlx` cli
```sh
cargo install sqlx-cli
```

## Create and Migrate the database
If `DATABASE_URL` is set:
```sh
sqlx database create
sqlx migrate run
```
