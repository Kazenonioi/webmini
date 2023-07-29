mod http;
mod thread_pool;
mod config;
mod http_handler;
use config::Config;
use std::net::{TcpListener};
pub use thread_pool::ThreadPool;
use http_handler::handle_request;
use std::error::Error;
use std::env;
use std::sync::Arc;

fn help() {
    println!("Usage: ./webmini [config path].");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = 
    match args.get(1) {
        Some(p) => {
            match Config::build(p) {
                Ok(r) => Arc::new(r),
                _ => {
                    println!("Failed parsing config file");
                    return ()
                }
            }
        },
        _ => {
            help();
            return ()
        }
    };
    if let Err(e) = run(cfg) {
        println!("{:?}", e);
    }
}

fn run(cfg: Arc<Config>) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(String::from(&cfg.ip) + ":" + cfg.listen.to_string().as_str())?;
    let mut pool = ThreadPool::new(cfg.worker_connections);
    for stream in listener.incoming() {
        let tcfg = Arc::clone(&cfg);
        let thread_run = move || {
            match stream {
                Ok(stream) => {
                    if let Err(e) = handle_request(tcfg, stream) {
                        
                    }
                }
                Err(e) => {
        
                }
            }
        };
        if let Err(e) = pool.execute(thread_run) {
            println!("{:?}", e);
        }
    }
    Ok(())
}
