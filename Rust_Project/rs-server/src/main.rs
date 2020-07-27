extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate lib;
extern crate protobuf;
mod yadfs;
mod yadfs_grpc;
use grpc::Server;
use lib::*;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use yadfs::*;
use yadfs_grpc::*;

struct Yadfs;

impl YadfsService for Yadfs {
    fn hello(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::HelloRequest,
    ) -> ::grpc::SingleResponse<HelloResponse> {
        let mut resp = yadfs::HelloResponse::new();
        let req = p.get_name();
        let res = format!("response to req:{}", req);
        resp.set_name(res);
        grpc::SingleResponse::completed(resp)
    }
    fn getdir(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::GetDirRequest,
    ) -> grpc::SingleResponse<GetDirResponse> {
        let mut resp = yadfs::GetDirResponse::new();
        let current = std::env::current_dir();
        if let Ok(path) = current {
            println!("The current directory is {}", path.display());
            resp.set_current_dir(format!("The current directory is {}", path.display()));
        } else {
            println!("ERROR: GETDIR");
            resp.set_current_dir(format!("ERROR: GETDIR"))
        }
        grpc::SingleResponse::completed(resp)
    }
    fn file_count(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::FileCountRequest,
    ) -> grpc::SingleResponse<FileCountResponse> {
        let mut resp = FileCountResponse::new();
        grpc::SingleResponse::completed(resp)
    }
    fn change_dir(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::ChangeDirRequest,
    ) -> grpc::SingleResponse<ChangeDirResponse> {
        let mut resp = ChangeDirResponse::new();
        let req = p.get_dir_path();
        let cd = std::env::set_current_dir(req);
        if let Ok(_) = cd {
            println!("The current directory is {}", req);
            resp.set_is_success(1);
        } else {
            println!("ERROR: CHANGEDIR");
            resp.set_is_success(0);
        }
        grpc::SingleResponse::completed(resp)
    }
    fn list(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::ListRequest,
    ) -> grpc::SingleResponse<ListResponse> {
        use std::fs::{self, DirEntry};
        use std::path::Path;
        let mut resp = ListResponse::new();
        let mut entries = std::fs::read_dir(".")
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();
        println!("{:?}", entries);
        entries.sort();
        let mut dirs = String::new();
        for e in entries.iter() {
            let e_str = e.to_str();
            if let Some(s) = e_str {
                dirs.push_str(s);
                dirs.push_str("\n");
            }
        }
        let dirs = String::from(dirs.trim_end());
        resp.set_list_dir(dirs);
        grpc::SingleResponse::completed(resp)
    }
    fn open_file_to_read(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::OpenFiletoReadRequest,
    ) -> grpc::SingleResponse<OpenFiletoReadResponse> {
        let mut resp = OpenFiletoReadResponse::new();
        let file_name = p.get_file_path();
        if let Ok(_) = File::open(file_name) {
            resp.set_is_success(1);
        } else {
            resp.set_is_success(0);
        }
        grpc::SingleResponse::completed(resp)
    }
    fn next_read(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::NextReadRequest,
    ) -> grpc::SingleResponse<NextReadResponse> {
        let mut resp = NextReadResponse::new();
        if let Ok(mut f) = File::open(p.file_name) {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            resp.set_is_end(0);
            resp.set_block(buf);
        } else {
            resp.set_is_end(1);
            resp.set_block("".to_string());
        }
        grpc::SingleResponse::completed(resp)
    }
    fn close_file(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::CloseFileRequest,
    ) -> grpc::SingleResponse<CloseFileResponse> {
        let mut resp = CloseFileResponse::new();
        grpc::SingleResponse::completed(resp)
    }
    fn open_file_to_write(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::OpenFiletoWriteRequest,
    ) -> grpc::SingleResponse<OpenFiletoWriteResopnse> {
        let mut resp = OpenFiletoWriteResopnse::new();
        if let Ok(f) = File::create(p.get_file_path()) {
            println!("OPEN FILE {} TO WRITE", p.get_file_path());
            resp.set_is_success(1);
        } else {
            resp.set_is_success(0);
        }
        grpc::SingleResponse::completed(resp)
    }
    fn next_write(
        &self,
        o: ::grpc::RequestOptions,
        p: yadfs::NextWriteRequest,
    ) -> grpc::SingleResponse<NextWriteResponse> {
        let mut resp = NextWriteResponse::new();
        let content = p.get_block();
        if let Ok(mut f) = File::create(p.get_file_name()) {
            if let Ok(size) = f.write(content.as_bytes()) {
                println!("WRITE {} BYTES TO FILE", size);
            } else {
                println!("ERROR: CAN NOT WITRE TO FILE {}", p.get_file_name());
            }
        }
        grpc::SingleResponse::completed(resp)
    }
}

fn main() {
    println!("Rust gRPC server.");
    println!("Welcome to Yadfs-Server.");
    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_addr("0.0.0.0:30303").unwrap();
    server.http.set_cpu_pool_threads(4);
    server.add_service(YadfsServiceServer::new_service_def(Yadfs));
    let _server: Server = server.build().expect("server");

    loop {
        thread::park();
    }
}
