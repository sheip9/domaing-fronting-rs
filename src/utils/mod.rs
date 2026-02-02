use actix_web::http::header::HeaderMap;
use actix_web::http::header;

pub fn extract_host(headers: &HeaderMap) -> Option<&str> {
    headers.get(header::HOST)?.to_str().ok()
}

pub fn build_fronted_url(front_domain: &str, _target_domain: &str, path: &str) -> String {
    // 使用HTTPS协议，确保域前置技术生效
    format!("https://{}{}", front_domain, path)
}

pub fn replace_host_header(headers: &mut HeaderMap, new_host: &str) {
    headers.insert(header::HOST, new_host.parse().unwrap());
}
