protoc --rust_out=./src --grpc_out=./src --plugin=protoc-gen-grpc=/home/yangjq/.cargo/bin/protoc-gen-rust-grpc  yadfs.proto
echo "Generating rpc-code for server..."
cp yadfs.proto ../rs-client
echo "Syncing client protocal..."
echo "Finish, have a nice day~"