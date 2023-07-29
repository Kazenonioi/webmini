pub mod Http {
    use std::collections::HashMap;
    use std::error::Error;
    use std::fmt;
    pub struct HttpResponse {
        ver: String,
        code: String,
        msg: String,
        headers: HashMap<String, String>,
        file: String,
    }
    impl fmt::Display for HttpResponse {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} {}\r\n", self.ver, self.code, self.msg)?;
            for (key, value) in &self.headers {
                write!(f, "{}: {}\r\n", key, value)?;
            }
            write!(f, "Server: https://github.com/Kazenonioi \r\n")?;
            write!(f, "\r\n")?;
            Ok(())
        }
    }
    impl HttpResponse {
        pub fn new() -> Self {
            HttpResponse { 
                ver: "HTTP/1.1".to_string(), 
                code: "200".to_string(), 
                msg: "OK".to_string(), 
                headers: HashMap::new(), 
                file : String::new(),
            }
        }
        pub fn ver(&mut self, ver: &str) -> &mut Self {
            self.ver = ver.to_string();
            self
        }
        pub fn code(&mut self, code: &str) -> &mut Self {
            self.code = code.to_string();
            self
        }
        pub fn msg(&mut self, msg: &str) -> &mut Self {
            self.msg = msg.to_string();
            self
        }
        pub fn header(&mut self, key: &str, val: &str) -> &mut Self {
            self.headers.insert(key.to_string(), val.to_string());
            self
        }
        pub fn file(&mut self, file: &str) -> &mut Self {
            self.file = file.to_string();
            self
        }
        pub fn get_file(&self) -> &String {
            &self.file
        }
        pub fn into_bytes(&self) -> Vec<u8> {
            format!("{}", self).into_bytes()
        }
    }


    enum Method {
        GET,
        POST,
        DELETE,
    }
    
    pub struct HttpRequest {
        method: String,
        url: String,
        ver: String,
        headers: HashMap<String, String>,
        payload: String,
    }

    impl HttpRequest {
        pub fn from(request: &String) -> Result<HttpRequest, Box<dyn Error>> {
            let mut method = String::new();
            let mut url = String::new();
            let mut ver = String::new();
            let mut headers: HashMap<String, String> = HashMap::new();
            let mut payload = String::new();
            let mut lines = request.lines();
            if let Some(request_line) = lines.next() {
                let request_line: Vec<&str> = request_line.split_whitespace().collect();
                if let (Some(x), Some(y), Some(z)) = (request_line.get(0), request_line.get(1), request_line.get(2)) {
                    method = x.to_string();
                    url = y.to_string();
                    ver = z.to_string();
                }
                let mut payload_line = lines.clone();
                for line in lines {
                    if line == "\r\n" {
                        break;
                    }
                    let header: Vec<&str> = line.split(": ").collect();
                    if let (Some(k), Some(v)) = (header.get(0), header.get(1)) {
                        headers.insert(k.to_string(), v.to_string());
                    }
                    payload_line.next();
                }
                for line in payload_line {
                    payload += line;
                }
            }
            Ok(HttpRequest{
                method,
                url,
                ver,
                headers,
                payload,
            })
        }
        pub fn get_url(&self) -> &str {
            &self.url.as_str()
        }
    }
    
}
