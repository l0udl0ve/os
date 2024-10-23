use std::net::TcpListener;
mod call;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    //unwrap() 方法用于处理可能发生的错误：如果绑定成功，则返回一个 TcpListener 实例；如果失败，则会触发 panic，程序终止。
    println!("Running on port 3000...");
    // let result = listener.accept().unwrap(); // 只接收一次请求
    for stream in listener.incoming() {
        let  stream = stream.unwrap();
        println!("Connection established!");
        //处理客户端请求
        call::handle_client(stream);
    }
}
//优化：为了提高性能，可以考虑使用多线程或多任务异步处理来同时处理多个连接。