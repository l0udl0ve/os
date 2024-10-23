use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
use rsa::{RsaPrivateKey, RsaPublicKey};
pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write, private_key: &RsaPrivateKey,client_public_key: &RsaPublicKey) -> () {
        let web_service_handler = WebServiceHandler::new(private_key.clone(),client_public_key.clone());
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            println!("api\n");
                            let resp: HttpResponse =  web_service_handler.handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&StaticPageHandler,&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&PageNotFoundHandler, &req);
                let _ = resp.send_response(stream);
            }
        }
    }
}

