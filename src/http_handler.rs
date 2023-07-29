use std::io::{BufReader, BufRead};
use std::{fs, fs::File, net::TcpStream, error::Error};
use crate::http::Http::HttpRequest;
use crate::http::Http::HttpResponse;
use std::io::{Read, Write};
use crate::Config;
use std::sync::Arc;

trait HttpConn {
    fn read_header(&mut self) -> Result<HttpRequest, Box<dyn Error>>;
    fn read_body(&mut self) -> Result<(), Box<dyn Error>>;
    fn write_header(&mut self, response: &HttpResponse) -> Result<(), Box<dyn Error>>;
    fn write_body_from_file(&mut self, path: &str) -> Result<(), Box<dyn Error>>;
}

impl HttpConn for TcpStream {
    fn read_header(&mut self) -> Result<HttpRequest, Box<dyn Error>> {
        let mut reader = BufReader::new(self);
        let mut request = String::new();
        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf)?;
            if buf == "\r\n" {
                break;
            }
            request = request + &buf[..];
        }
        HttpRequest::from(&request)
    }
    fn read_body(&mut self) -> Result<(), Box<dyn Error>> {
        // todo
        Ok(())
    }
    fn write_header(&mut self, response: &HttpResponse) -> Result<(), Box<dyn Error>> {
        self.write(&response.into_bytes())?;
        Ok(())
    }
    fn write_body_from_file(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        // Old implementation
        // let mut f = File::open(path)?;
        // let mut buf = [0; 1024];
        // loop {
        //     let size = f.read(&mut buf)?;
        //     self.write(&buf[0..size])?;
        //     if size < buf.len() {
        //         break;
        //     }
        // }
        self.write(&fs::read(path)?)?;
        Ok(())
    }
}

pub fn response_from_request(cfg: Arc<Config>, req: &HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let mut response = HttpResponse::new();
    let mut path = req.get_url().to_string();
    if path == "/" {
        path = path + cfg.default.as_str();
    }
    let file = String::from(&cfg.root) + path.as_str();
    if let Err(e) = File::open(&file) {
        response.code("404").msg("Not Found.").file((String::from(&cfg.root) + "/" + &cfg.error).as_str());
    } else {
        response.file(file.as_str());
    }
    Ok(response)
}

pub fn handle_request(cfg: Arc<Config>, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {

    // Read request header
    let req = stream.read_header()?;

    // Generate and send response header
    let response = response_from_request(Arc::clone(&cfg), &req)?;
    stream.write_header(&response)?;

    // Send response body
    stream.write_body_from_file(&response.get_file())?;

    Ok(())
}
