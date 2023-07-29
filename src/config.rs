use std::collections::HashMap;
use std::fs::File;
use std::{fs, error::Error};
use std::io::{Read};

#[derive(Debug)]
pub struct Config {
    pub error_log: String,
    pub access_log: String, 
    pub worker_connections: i32,
    pub ip: String,
    pub listen: u16,
    pub server_name: String,
    pub root: String,
    pub default: String,
    pub error: String,
}

impl Config {
    pub fn build(path: &str) -> Result<Config, Box<dyn Error>> {
        let f = fs::read_to_string(path)?;
        let mut conf = Config {
            error_log: String::new(),
            access_log: String::new(),
            worker_connections: 0,
            ip: String::new(),
            listen: 0,
            server_name: String::new(),
            root: String::new(),
            default: String::new(),
            error: String::new()
        };
        for line in f.lines() {
            let config_line: Vec<&str> = line.split_whitespace().collect();
            if let (Some(k), Some(v)) = (config_line.get(0), config_line.get(1)) {
                let v = &&v[0..v.len()-1];
                match *k {
                    "error_log" => conf.error_log = v.to_string(),
                    "access_log" => conf.access_log = v.to_string(),
                    "worker_connections" => conf.worker_connections = v.trim().parse()?,
                    "ip" => conf.ip = v.to_string(),
                    "listen" => conf.listen = v.trim().parse()?,
                    "server_name" => conf.server_name = v.to_string(),
                    "root" => conf.root = v.to_string(),
                    "default" => conf.default = v.to_string(),
                    "error" => conf.error = v.to_string(),
                    _ => continue,
                }
            }
        }
        Ok(conf)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_config() {
        println!("{:?}", Config::build("/home/jingcheng/.webserver"));
    }
}
