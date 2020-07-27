extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate lib;
extern crate protobuf;
mod yadfs;
mod yadfs_grpc;
use grpc::*;
use lib::*;
use std::fs::File;
use std::io::{Read, Write};
use yadfs::*;
use yadfs_grpc::*;
macro_rules! l {
    () => {
        println!("LINE:{}", std::line!());
    };
}
const STATE_INIT: i32 = 0; // try to connect server
                           // const STATE_IDLE: i32 =  1; // accept command
const STATE_EXEC_CMD: i32 = 2; // excute command
                               // const STATE_RECV_RSP: i32 = 3; // receive response and handle it
const STATE_EXIT: i32 = 4; // exit process

const CMD_IDLE: i32 = 5;
const CMD_GETDIR: i32 = 6;
const CMD_FILE_COUNT: i32 = 7;
const CMD_CHANGEDIR: i32 = 8;
const CMD_LIST: i32 = 9;
const CMD_GET: i32 = 10;
const CMD_PUT: i32 = 11;
const CMD_QUIT: i32 = 12;
const CMD_HELP: i32 = 13;

fn main() {
    println!("Rust gRPC client.");
    println!("Welcome to Yadfs-Client.");
    let mut state: i32 = STATE_EXEC_CMD;
    let mut client = YadfsServiceClient::new_plain("0.0.0.0", 30303, Default::default()).unwrap();
    let mut req = HelloRequest::new();
    req.set_name("client request.".to_string());
    let resp = client.hello(grpc::RequestOptions::new(), req);
    if let Ok(data) = resp.wait() {
        // println!("{:?}", data.0);
        // println!("{:?}", data.1);
        // println!("{:?}", data.2);
        println!("Default connection established.");
    }

    while state != STATE_EXIT {
        if state == STATE_INIT {
            print!("Please input IPv4 address:\n>> ");
            let mut ipv4_addr: String = String::new();
            std::io::stdin().read_line(&mut ipv4_addr).unwrap();
            println!("Are you sure: {:?}", ipv4_addr.trim());
            print!("Please input port number:\n>> ");
            let mut port: String = String::new();
            std::io::stdin().read_line(&mut port).unwrap();
            println!("Are you sure: {:?}", port.trim());
            let port: u16 = port.trim().parse::<u16>().unwrap();
            let mut c = YadfsServiceClient::new_plain(&ipv4_addr.trim(), port, Default::default());
            match c {
                Ok(s) => {
                    println!("Connection established successfully.");
                    client = s;
                    state = STATE_EXEC_CMD
                }
                Err(_) => (),
            };
        }
        if state == STATE_EXEC_CMD {
            let mut cmd: i32 = CMD_IDLE;
            let mut cmd_string: String = String::new();
            while cmd != CMD_QUIT {
                if cmd == CMD_IDLE {
                    cmd_string = String::new();
                    print!(">> ");
                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut cmd_string).unwrap();
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() < 1 {
                        cmd = CMD_IDLE
                    } else {
                        match argv[0] {
                            "help" | "h" => cmd = CMD_HELP,
                            "quit" | "q" => cmd = CMD_QUIT,
                            "get" => cmd = CMD_GET,
                            "put" => cmd = CMD_PUT,
                            "getdir" => cmd = CMD_GETDIR,
                            "cd" => cmd = CMD_CHANGEDIR,
                            "ls" => cmd = CMD_LIST,
                            _ => {}
                        }
                    }
                }
                if cmd == CMD_HELP {
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() == 1 {
                        println!("Help info as below:...");
                    }
                    cmd = CMD_IDLE;
                }
                if cmd == CMD_QUIT {
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() == 1 {
                        println!("Good bye! Have a nice day~");
                        std::process::exit(0);
                    }
                    cmd = CMD_IDLE;
                }
                if cmd == CMD_GETDIR {
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() == 1 {
                        let mut req = GetDirRequest::new();
                        req.set_file_path("client request.".to_string());
                        let resp = client.getdir(grpc::RequestOptions::new(), req);
                        if let Ok(data) = resp.wait() {
                            // println!("{:?}", data.0);
                            println!("{}", data.1.current_dir);
                        // println!("{:?}", data.2);
                        } else {
                            println!("ERROR: GETDIR");
                        }
                    }
                    cmd = CMD_IDLE;
                }
                if cmd == CMD_GET {
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() >= 2 {
                        let files = &argv[1..argv.len()];
                        for f in files {
                            let mut req1 = OpenFiletoReadRequest::new();
                            req1.set_file_path(f.to_string());
                            let resp1 = client.open_file_to_read(grpc::RequestOptions::new(), req1);
                            if let Ok(data) = resp1.wait() {
                                if data.1.is_success == 1 {
                                    let mut req2 = NextReadRequest::new();

                                    req2.set_file_name(f.to_string());
                                    let resp2 = client.next_read(grpc::RequestOptions::new(), req2);
                                    if let Ok(data1) = resp2.wait() {
                                        if data1.1.is_end == 0 {
                                            let mut file_local = File::create(f).unwrap();
                                            let size = file_local.write(data1.1.block.as_bytes()).unwrap();
                                            println!("READ {} BYTES TO FILE",size);
                                        };
                                    } else {
                                        println!("ERROR: CAN NOT READ REMOTE FILE {}", f);
                                    }
                                } else {
                                    println!("ERROR: CAN NOT OPEN REMOTE FILE {} TO READ", f);
                                }
                            } else {
                                println!("ERROR: CAN NOT OPEN REMOTE FILE {} TO READ", f);
                            }
                        }
                    }
                    cmd = CMD_IDLE;
                }
                if cmd == CMD_PUT {
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() >= 2 {
                        let files = &argv[1..argv.len()];
                        for f in files {
                            if let Ok(mut local_f) = File::open(f) {
                                let mut buf = String::new();
                                let _f_size = local_f.read_to_string(&mut buf);
                                let mut req = OpenFiletoWriteRequest::new();
                                req.set_file_path(f.to_string());
                                let resp =
                                    client.open_file_to_write(grpc::RequestOptions::new(), req);
                                if let Ok(data) = resp.wait() {
                                    if data.1.is_success == 0 {
                                        println!("ERROR: CAN NOT OPEN REMOTE FILE TO WRITE");
                                    } else if data.1.is_success == 1 {
                                        let mut req1 = NextWriteRequest::new();
                                        req1.set_file_name(f.to_string());
                                        req1.set_block(buf);
                                        req1.set_is_end(1);
                                        let resp1 =
                                            client.next_write(grpc::RequestOptions::new(), req1);
                                        if let Ok(_) = resp1.wait() {
                                            println!("Write FILE {} SUCCESS", f);
                                        }
                                    }
                                } else {
                                    println!("ERROR: NO RESPONSE FOR OPEN_FILE_TO_WRITE ")
                                }
                            } else {
                                println!("ERROR: NO LOCAL FILE {}", f);
                            }
                        }
                    }
                    cmd = CMD_IDLE;
                }
                if cmd == CMD_CHANGEDIR {
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() == 2 {
                        let path = String::from(argv[1]);
                        let mut req = ChangeDirRequest::new();
                        req.set_dir_path(path);
                        let resp = client.change_dir(grpc::RequestOptions::new(), req);
                        if let Ok(data) = resp.wait() {
                            // println!("{:?}", data);
                            // println!("{:?}", data.0);
                            // println!("{:?}", data.1);
                            if data.1.is_success == 1 {
                                println!("CHANGEDIR SUCCESS!")
                            }
                            if data.1.is_success == 0 {
                                println!("ERROR: CHANGEDIR");
                            }
                        } else {
                            println!("ERROR: CHANGEDIR");
                        }
                    }
                    cmd = CMD_IDLE;
                }
                if cmd == CMD_LIST {
                    let argv: Vec<&str> = cmd_string.trim().split(" ").collect();
                    if argv.len() == 1 {
                        let mut req = ListRequest::new();
                        let resp = client.list(grpc::RequestOptions::new(), req);
                        if let Ok(data) = resp.wait() {
                            println!("{}", data.1.list_dir);
                        } else {
                            println!("ERROR: LIST");
                        }
                    }
                    cmd = CMD_IDLE;
                }
            }
            state = STATE_EXIT
        }
    }
}
