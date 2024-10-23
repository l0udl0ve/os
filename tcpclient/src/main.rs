use core::str;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::io;
use http::httpresponse::HttpResponse;
use serde_json::{json, Value};
use crypto::rsa_crypto;
use bincode;
// use http::httpresponse::HttpResponse;
use rsa::RsaPublicKey;
use serde_json;

fn main() {
    // 连接服务端
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    //生成密钥对
    let (private_key, public_key) = rsa_crypto::generate_keys();
    //序列化客户端公钥
    let encoded_public_key =match bincode::serialize(&public_key){
        Ok(encoded) => encoded,
        Err(e) => {
            eprintln!("Failed to serialize public key: {}", e);
            Vec::new()
        }
    };     
    //发送客户端公钥
    println!("Sending public key to server...\n{:?}",encoded_public_key);
    stream.write(&encoded_public_key).unwrap();
    //接受服务端公钥
    let mut buf = [0; 2048];
    let n = stream.read(&mut buf).unwrap();
    let  server_public_key = &buf[..n];
    let decoded_server_public_key = match bincode::deserialize::<RsaPublicKey>(&server_public_key) {
        Ok(decoded) => decoded,
        Err(e) => {
            eprintln!("Failed to deserialize public key: {}", e);
            return ;
        }
    };
    println!("Received server public key:\n{:?}",decoded_server_public_key);
    //发送服务请求
    println!("请输入服务名称：");
    let mut service_name = String::new();
    io::stdin()
        .read_line(&mut service_name)
        .expect("未能读取行");
    // 去除末尾的换行符
    service_name = service_name.trim().to_string();
    
    println!("请输入数据：");
    let mut service_data = String::new();
    io::stdin()
        .read_line(&mut service_data)
        .expect("未能读取行");

    // 去除末尾的换行符
    service_data = service_data.trim().to_string();
    
    let len_service_data= service_data.len();

    
    let request: String = serde_json::to_string(&json!({
        "service_name": service_name,
        "data_len": len_service_data,
        "data": service_data,
        
    })).expect("JSON serialization failed");
    
    let encrypted_request = rsa_crypto::encrypt_u8(&decoded_server_public_key, request.as_bytes());
    
    
    //采用http协议 
    let request: String = format!("GET /api/{} HTTP/1.1\nHost: localhost:3000\nContent-Type: application/json\nContent-Length: {}\n\n{:?}", service_name, len_service_data, encrypted_request);
    //传输数据给服务端
    stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write(request.as_bytes()).unwrap();
    println!("Sent request:\n{}",request);
    //接受服务端返回的数据
    let mut buffer = [0;2058];
    // stream.read(&mut buffer).unwrap();
    
    let n=match stream.read(&mut buffer) {
        Ok(size) => {
            // 打印接收到的数据
            let received_data = String::from_utf8_lossy(&buffer[..size]);
            println!("Received from server: {}", received_data.trim());
            size
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
            0
        }
    };
    let  buf = &buffer[..n];

    
    let response_str= match str::from_utf8(&buf){
        Ok(s) => s,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
   
    let resp=HttpResponse::from_str(response_str);
    // let response_str: HttpResponse= response_str.into();
    
    let json_res = resp.body;
    let res = json_res.unwrap_or_default();
    // println!("1  {:?}",res);
    let res = &res[1..res.len() - 1];
    // println!("2  {:?}",res);
    let mut res_items = res.splitn(2, ":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = res_items.next() {
        key = k.to_string();
        // println!("{:?}",key);
    }
    if let Some(v) = res_items.next() {
        value = v.to_string();
        // println!("{:?}",value);
    }
    let value=match parse_to_vec_u8(&value) {
                Ok(vec) => vec, // 输出: [15, 69, 40]
                Err(e) => {
                    println!("解析错误: {}", e);
                    return ;
                }
            };
    let res=rsa_crypto::decrypt_u8(&private_key, &value);
    
    match String::from_utf8(res) {
        Ok(s) => println!("result: {}", s),
        Err(e) => println!("Conversion error: {}", e),
    }


    
}
fn parse_to_vec_u8(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    // 去掉方括号
    let trimmed = input.trim_matches(|c| c == '[' || c == ']');
    
    // 分割字符串，去掉空格和逗号
    let parts: Vec<&str> = trimmed.split(|c: char| c.is_whitespace() || c == ',').filter(|s| !s.is_empty()).collect();
    
    // 将字符串部分转换为 u8
    let result: Result<Vec<u8>, _> = parts.iter().map(|s| s.parse::<u8>()).collect();
    
    result
//     match parse_to_vec_u8(input) {
//         Ok(vec) => println!("{:?}", vec), // 输出: [15, 69, 40]
//         Err(e) => println!("解析错误: {}", e),
//     }
}


