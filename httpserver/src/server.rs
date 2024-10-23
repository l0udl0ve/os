use super::router::Router;
use crypto::rsa_crypto;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;
use bincode;
use rsa::RsaPublicKey;
pub struct Server<'a> {
    socket_addr: &'a str,//服务器套接字地址
}

impl<'a> Server<'a> {
    //构造函数
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        //生成密钥
        let (private_key, public_key) = rsa_crypto::generate_keys();
        //接受客户端公钥
        let mut decoded_client_public_key;
        let mut client= connection_listener.incoming();
        if let Some(Ok(mut first_stream)) = client.next() {
            let mut buf = [0; 2048];
            let n = first_stream.read(&mut buf).unwrap();
            let  client_public_key = &buf[..n];
            //反序列化客户端公钥
            decoded_client_public_key = match bincode::deserialize::<RsaPublicKey>(&client_public_key) {
                Ok(decoded) => decoded,
                Err(e) => {
                    eprintln!("Failed to deserialize public key: {}", e);
                    return ;
                }
            };
            println!("Received client public key: {:?}", decoded_client_public_key);
            //发送服务端公钥
            let encoded_public_key =match bincode::serialize(&public_key){
                Ok(encoded) => encoded,
                Err(e) => {
                    eprintln!("Failed to serialize public key: {}", e);
                    Vec::new()
                }
            };  
            println!("Sending public key: {:?}", encoded_public_key);
            first_stream.write(&encoded_public_key).unwrap();
        } else {
            eprintln!("Failed to accept the first client");
            return;
        }

        
        // let mut buf = [0; 2048];
        // let n = stream.read(&mut buf).unwrap();
        // let  client_public_key = &buf[..n];
        // //发送服务端公钥
        
        // stream.write(public_key.as_bytes()).unwrap();
        

        for stream in connection_listener.incoming() {
            println!("12300");
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 2058];
            stream.read(&mut read_buffer).unwrap();
            
             // 打印原始的HTTP请求
            let request_str = match str::from_utf8(&read_buffer) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            println!("Received HTTP request:\n{}\n", request_str.trim_end_matches(char::from(0)));

            //将数据转化成http包格式
            let req= HttpRequest::from(request_str.to_string());
            println!("{:?}", req);
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            
            // println!("{:?}", req);
            println!("开始处理\n");
            Router::route(req, &mut stream, &private_key ,&decoded_client_public_key);
        }
    }
}

