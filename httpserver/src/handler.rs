use crypto::rsa_crypto;
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
// use service::message::Message;
//用于序列化和反序列化 JSON 数据
use std::collections::HashMap;
use std::env;
use std::fs;
mod call;
use rsa::{RsaPrivateKey, RsaPublicKey};
pub trait Handler {
    //定义了一个抽象方法，用于处理 HTTP 请求并返回 HttpResponse。
    fn handle(&self,req: &HttpRequest) -> HttpResponse;
    
    fn load_file(&self,file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        //读取文件内容，并返回 Option<String>。
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;

// pub struct rpcServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}


pub struct WebServiceHandler {
    private_key:  RsaPrivateKey,
    client_public_key:  RsaPublicKey,
}
impl Handler for PageNotFoundHandler {
    fn handle(&self,_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file(&self,"404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(&self,req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "index" => HttpResponse::new("200", None, Self::load_file(&self,"index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file(&self,"health.html")),
            
            path => match &Self::load_file(&self,path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents.to_string()))
                }
                None => HttpResponse::new("404", None, Self::load_file(&self,"404.html")),
            },
        }
    }
}

impl WebServiceHandler {
    pub fn new(private_key:  RsaPrivateKey,client_public_key:  RsaPublicKey) -> Self {
        WebServiceHandler { private_key , client_public_key}
    }
    // 加载特定文件夹下的JSON文件
    // fn load_json() -> Vec<OrderStatus> {
    //     let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    //     let data_path = env::var("DATA_PATH").unwrap_or(default_path);
    //     let full_path = format!("{}/{}", data_path, "orders.json");
    //     let json_contents = fs::read_to_string(full_path);
    //     let orders: Vec<OrderStatus> =
    //         serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
    //     orders
    // }
}

impl Handler for WebServiceHandler {
    
    fn handle(&self, req: &HttpRequest) -> HttpResponse {
        //获取资源路径 服务类型
        // let http::httprequest::Resource::Path(s) = &req.resource;
        // 获取请求体 数据
        println!("handle");
        let req_data =&req.msg_body;
        println!("{:?}", req_data);
        let res=call::handle_client(req_data,&self.private_key,&self.client_public_key);
        // 将路径分割成数组
        
        if res.contains("Error") {
            HttpResponse::new("404", None, Self::load_file(&self,"404.html"))
        }
        else{
            let res=rsa_crypto::encrypt_str(&self.client_public_key,&res);
            let body = Some(serde_json::to_string(&json!({
                "result": res,
            })).unwrap());
            let mut headers: HashMap<&str, &str> = HashMap::new();
            headers.insert("Content-Type", "application/json");
            HttpResponse::new("200", Some(headers), body)         
        }
        
    }
}

