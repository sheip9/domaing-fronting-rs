use clap::Parser;
use steam_proxy::config::Config;
use steam_proxy::proxy::ProxyServer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

#[actix_web::main]
async fn main() {
    let args = Args::parse();
    let config = Config::load(args.config).expect("Failed to load config");
    let proxy_server = ProxyServer::new(config);
    if let Err(e) = proxy_server.run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

