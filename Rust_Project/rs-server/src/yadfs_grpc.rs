// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait YadfsService {
    fn hello(&self, o: ::grpc::RequestOptions, p: super::yadfs::HelloRequest) -> ::grpc::SingleResponse<super::yadfs::HelloResponse>;

    fn getdir(&self, o: ::grpc::RequestOptions, p: super::yadfs::GetDirRequest) -> ::grpc::SingleResponse<super::yadfs::GetDirResponse>;

    fn change_dir(&self, o: ::grpc::RequestOptions, p: super::yadfs::ChangeDirRequest) -> ::grpc::SingleResponse<super::yadfs::ChangeDirResponse>;

    fn file_count(&self, o: ::grpc::RequestOptions, p: super::yadfs::FileCountRequest) -> ::grpc::SingleResponse<super::yadfs::FileCountResponse>;

    fn list(&self, o: ::grpc::RequestOptions, p: super::yadfs::ListRequest) -> ::grpc::SingleResponse<super::yadfs::ListResponse>;

    fn open_file_to_read(&self, o: ::grpc::RequestOptions, p: super::yadfs::OpenFiletoReadRequest) -> ::grpc::SingleResponse<super::yadfs::OpenFiletoReadResponse>;

    fn next_read(&self, o: ::grpc::RequestOptions, p: super::yadfs::NextReadRequest) -> ::grpc::SingleResponse<super::yadfs::NextReadResponse>;

    fn close_file(&self, o: ::grpc::RequestOptions, p: super::yadfs::CloseFileRequest) -> ::grpc::SingleResponse<super::yadfs::CloseFileResponse>;

    fn open_file_to_write(&self, o: ::grpc::RequestOptions, p: super::yadfs::OpenFiletoWriteRequest) -> ::grpc::SingleResponse<super::yadfs::OpenFiletoWriteResopnse>;

    fn next_write(&self, o: ::grpc::RequestOptions, p: super::yadfs::NextWriteRequest) -> ::grpc::SingleResponse<super::yadfs::NextWriteResponse>;
}

// client

pub struct YadfsServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Hello: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::HelloRequest, super::yadfs::HelloResponse>>,
    method_Getdir: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::GetDirRequest, super::yadfs::GetDirResponse>>,
    method_ChangeDir: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::ChangeDirRequest, super::yadfs::ChangeDirResponse>>,
    method_FileCount: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::FileCountRequest, super::yadfs::FileCountResponse>>,
    method_List: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::ListRequest, super::yadfs::ListResponse>>,
    method_OpenFileToRead: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::OpenFiletoReadRequest, super::yadfs::OpenFiletoReadResponse>>,
    method_NextRead: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::NextReadRequest, super::yadfs::NextReadResponse>>,
    method_CloseFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::CloseFileRequest, super::yadfs::CloseFileResponse>>,
    method_OpenFileToWrite: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::OpenFiletoWriteRequest, super::yadfs::OpenFiletoWriteResopnse>>,
    method_NextWrite: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::yadfs::NextWriteRequest, super::yadfs::NextWriteResponse>>,
}

impl ::grpc::ClientStub for YadfsServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        YadfsServiceClient {
            grpc_client: grpc_client,
            method_Hello: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/Hello".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Getdir: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/Getdir".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ChangeDir: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/ChangeDir".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_FileCount: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/FileCount".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_List: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/List".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_OpenFileToRead: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/OpenFileToRead".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_NextRead: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/NextRead".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CloseFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/CloseFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_OpenFileToWrite: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/OpenFileToWrite".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_NextWrite: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/YadfsService/NextWrite".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl YadfsService for YadfsServiceClient {
    fn hello(&self, o: ::grpc::RequestOptions, p: super::yadfs::HelloRequest) -> ::grpc::SingleResponse<super::yadfs::HelloResponse> {
        self.grpc_client.call_unary(o, p, self.method_Hello.clone())
    }

    fn getdir(&self, o: ::grpc::RequestOptions, p: super::yadfs::GetDirRequest) -> ::grpc::SingleResponse<super::yadfs::GetDirResponse> {
        self.grpc_client.call_unary(o, p, self.method_Getdir.clone())
    }

    fn change_dir(&self, o: ::grpc::RequestOptions, p: super::yadfs::ChangeDirRequest) -> ::grpc::SingleResponse<super::yadfs::ChangeDirResponse> {
        self.grpc_client.call_unary(o, p, self.method_ChangeDir.clone())
    }

    fn file_count(&self, o: ::grpc::RequestOptions, p: super::yadfs::FileCountRequest) -> ::grpc::SingleResponse<super::yadfs::FileCountResponse> {
        self.grpc_client.call_unary(o, p, self.method_FileCount.clone())
    }

    fn list(&self, o: ::grpc::RequestOptions, p: super::yadfs::ListRequest) -> ::grpc::SingleResponse<super::yadfs::ListResponse> {
        self.grpc_client.call_unary(o, p, self.method_List.clone())
    }

    fn open_file_to_read(&self, o: ::grpc::RequestOptions, p: super::yadfs::OpenFiletoReadRequest) -> ::grpc::SingleResponse<super::yadfs::OpenFiletoReadResponse> {
        self.grpc_client.call_unary(o, p, self.method_OpenFileToRead.clone())
    }

    fn next_read(&self, o: ::grpc::RequestOptions, p: super::yadfs::NextReadRequest) -> ::grpc::SingleResponse<super::yadfs::NextReadResponse> {
        self.grpc_client.call_unary(o, p, self.method_NextRead.clone())
    }

    fn close_file(&self, o: ::grpc::RequestOptions, p: super::yadfs::CloseFileRequest) -> ::grpc::SingleResponse<super::yadfs::CloseFileResponse> {
        self.grpc_client.call_unary(o, p, self.method_CloseFile.clone())
    }

    fn open_file_to_write(&self, o: ::grpc::RequestOptions, p: super::yadfs::OpenFiletoWriteRequest) -> ::grpc::SingleResponse<super::yadfs::OpenFiletoWriteResopnse> {
        self.grpc_client.call_unary(o, p, self.method_OpenFileToWrite.clone())
    }

    fn next_write(&self, o: ::grpc::RequestOptions, p: super::yadfs::NextWriteRequest) -> ::grpc::SingleResponse<super::yadfs::NextWriteResponse> {
        self.grpc_client.call_unary(o, p, self.method_NextWrite.clone())
    }
}

// server

pub struct YadfsServiceServer;


impl YadfsServiceServer {
    pub fn new_service_def<H : YadfsService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/YadfsService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/Hello".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.hello(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/Getdir".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.getdir(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/ChangeDir".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.change_dir(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/FileCount".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.file_count(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/List".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/OpenFileToRead".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.open_file_to_read(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/NextRead".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.next_read(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/CloseFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.close_file(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/OpenFileToWrite".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.open_file_to_write(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/YadfsService/NextWrite".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.next_write(o, p))
                    },
                ),
            ],
        )
    }
}
