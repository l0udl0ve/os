use core::str;
use std::io::{Read, Write};
use std::net::TcpStream;
use service::stub;
use service::message::Message;

pub fn handle_client(mut stream: TcpStream) 
{
    

    let mut buffer = [0; 1024];
    //从客户端读取数据并存储到 buffer 中。unwrap() 方法用于处理可能的错误，如果读取成功，返回读取的字节数。
    let bytes_read = stream.read(&mut buffer).unwrap();
    let received_data = &buffer[..bytes_read];  // 截断缓冲区到实际读取的字节数    
    println!(
        "Request from client: {:?}",
        str::from_utf8(received_data).unwrap()
    );
    let mes=serde_json::from_slice::<Message>(received_data);
    //获取服务类型
    match mes {
        Ok(mes) => {
            
            //服务匹配
            let res= match_service(mes );
            stream.write(res.as_bytes()).unwrap();
            res
        },
        Err(e) => {
            let error_message = format!("Error parsing message: {}", e);
            stream.write(error_message.as_bytes()).unwrap();
            error_message
        }
    };
    

}

pub fn match_service(mes: Message) -> String {
    let mes_sname = mes.service_name.trim();
    let res=match mes_sname {
        "add" => format!("{}", stub::add(1, 2)),
        "subtract" => format!("{}", stub::subtract(5, 3)),
        _ => "Unknown request".to_string(),
    };
    return res;
}


