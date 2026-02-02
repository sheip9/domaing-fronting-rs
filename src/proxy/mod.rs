use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::http::StatusCode;
use reqwest::{Client, ClientBuilder as ReqwestClientBuilder};
use std::net::SocketAddr;
use std::sync::Arc;
use crate::config::Config;
use crate::utils::build_fronted_url;

pub struct ProxyServer {
    config: Arc<Config>,
    client: Client,
}

impl ProxyServer {
    pub fn new(config: Config) -> Self {
        // 创建reqwest客户端，配置TLS以实现域前置
        let client = ReqwestClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create client");
        
        Self {
            config: Arc::new(config),
            client,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr: SocketAddr = format!("{}:{}", self.config.server.address, self.config.server.port).parse()?;
        let server = HttpServer::new({
            let server = self.clone();
            move || {
                App::new()
                    .app_data(web::Data::new(server.clone()))
                    .default_service(web::route().to(Self::handle_request))
            }
        })
        .bind(addr)?
        .run();
        println!("Proxy server running on {}:{}", self.config.server.address, self.config.server.port);
        server.await?;
        Ok(())
    }

    async fn handle_request(req: HttpRequest, body: web::Bytes, server: web::Data<Self>) -> impl Responder {
        let path = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
        let fronted_url = build_fronted_url(
            &server.config.domain_fronting.front_domain,
            &server.config.domain_fronting.target_domain,
            path
        );
        
        println!("Sending request to: {}", fronted_url);
        println!("Using HOST header: {}", server.config.domain_fronting.target_domain);
        
        // 构建reqwest请求
        let mut request_builder = server.client.request(req.method().clone(), fronted_url);
        
        // 复制所有头部，除了Host
        for (name, value) in req.headers() {
            if name != &actix_web::http::header::HOST {
                request_builder = request_builder.header(name.clone(), value.clone());
            }
        }
        
        // 设置Host头部为目标域名
        request_builder = request_builder.header(actix_web::http::header::HOST, server.config.domain_fronting.target_domain.clone());
        
        // 发送请求
        match request_builder.body(body.to_vec()).send().await {
            Ok(response) => {
                println!("Received response with status: {}", response.status());
                let mut resp = HttpResponse::build(response.status());
                
                // 复制响应头部
                for (name, value) in response.headers() {
                    resp.append_header((name.clone(), value.clone()));
                }
                
                // 读取响应体
                match response.bytes().await {
                    Ok(body) => resp.body(body),
                    Err(e) => {
                        println!("Error reading response body: {}", e);
                        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(format!("Error reading response body: {}", e))
                    },
                }
            }
            Err(e) => {
                println!("Error sending request: {}", e);
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(format!("Error sending request: {}", e))
            }
        }
    }
}

impl Clone for ProxyServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            client: self.client.clone(),
        }
    }
}
