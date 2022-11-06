use std::io::Write;

use crate::handler::Hanler;

use super::handler::{PageNotFoundHandler, StaticPageHandler, 
WebServiceHanlder};
use http::{
    http_request,
    http_request::HttpRequest,
    httpreponse::HttpReponse
};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            http_request::Method::Get => match &req.resource {
                http_request::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpReponse = WebServiceHanlder::handle(&req);
                            let _ = resp.send_reponse(stream);
                        }
                        _ => {
                            let resp: HttpReponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_reponse(stream);
                        }
                    }
                }
            }
            _ => {
                let resp: HttpReponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_reponse(stream);
            }
            
        }
    }
}