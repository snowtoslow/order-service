protoc --proto_path=/home/snowtoslow/MagicStuff/univer/PAD/order-service/rpc/proto/order --go_out=plugins=grpc:/home/snowtoslow/go/src/car-service/ car.proto

useful link: https://github.com/hyperium/tonic/issues/851

cd src && cargo build
