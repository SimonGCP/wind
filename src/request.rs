use std::vec::Vec;
use std::io::{ BufRead, BufReader, Read };
use std::net::TcpStream;

use crate::Param;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: String,
    pub query: String,
    pub content_type: String,
    pub content_length: usize,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub accept_encoding: String,
    pub accept_language: String,
    pub referer: String,
    pub cookie: String,
    pub connection: String,
    pub priority: String,
    pub upgrade_insecure_requests: String,

    pub sec_fetch_dest: String,
    pub sec_fetch_mode: String,
    pub sec_fetch_site: String,
    pub sec_fetch_user: String,

    pub body: Vec<u8>,
    pub params: Vec<Param>,
}

impl Request {
    pub fn new(stream: &TcpStream) -> Result<Request, std::io::Error> {
        let mut buf_reader = BufReader::new(stream);
        let mut req =  Request { 
            method: String::new(),
            query: String::new(),
            content_type: String::new(),
            content_length: 0,
            host: String::new(),
            user_agent: String::new(),
            accept: String::new(),
            accept_encoding: String::new(),
            accept_language: String::new(),
            referer: String::new(),
            cookie: String::new(), 
            connection: String::new(),
            priority: String::new(),
            upgrade_insecure_requests: String::new(),

            sec_fetch_dest: String::new(),
            sec_fetch_mode: String::new(),
            sec_fetch_site: String::new(),
            sec_fetch_user: String::new(),

            body: Vec::new(),
            params: Vec::new(),
        };

        let timeout = 100_000;
        let mut count = 0;
        loop {
            let mut line = String::new();
            let size = buf_reader.read_line(&mut line)?;
            
            let mut line = line.split(":");
            let key = line.next().unwrap();
            
            let mut val = String::new();
            let mut _val = line.next();
            while _val.is_some() {
                val += _val.unwrap();
                _val = line.next();
            }
            if val.len() != 0 {
                val = String::from(&val[1..].replace(&['\r', '\n'], ""));
            }

            // First line should give method and query string 
            if 
                key.starts_with("GET") ||
                key.starts_with("POST") ||
                key.starts_with("PUT") ||
                key.starts_with("DELETE")
            {
                let mut key_list = key.split(" ");
                req.method = key_list.next().unwrap().to_string();
                req.query = key_list.next().unwrap().to_string();

                continue;
            } 

            match key {
                "Content-Length" => {
                    let content_str = val.split(" ")
                        .last()
                        .unwrap();

                    req.content_length = content_str.parse::<usize>().unwrap();
                }
                "Content-Type" => req.content_type = val, 
                "Host" => req.host = val,
                "User-Agent" => req.user_agent = val,
                "Accept" => req.accept = val,
                "Accept-Encoding" => req.accept_encoding = val,
                "Accept-Language" => req.accept_language = val,
                "Connection" => req.connection = val,
                "Referer" => req.referer = val,
                "Cookie" => req.cookie = val,
                "Priority" => req.priority = val,
                "Upgrade-Insecure-Requests" => req.upgrade_insecure_requests = val,
                "Sec-Fetch-Dest" => req.sec_fetch_dest = val,
                "Sec-Fetch-Mode" => req.sec_fetch_mode = val,
                "Sec-Fetch-Site" => req.sec_fetch_site = val,
                "Sec-Fetch-User" => req.sec_fetch_user = val,
                "\r\n" => break,
                _ => { println!("Unrecognized header: {key}") },
            }
            
            count += 1; 
            if count > timeout {
                break;
            }

            if size == 2 {
                break;
            }
        }


        if req.content_length > 0 {
            // read body if there is one included with request
            req.body = Vec::with_capacity(req.content_length);
            for _ in 0..req.content_length {
                let mut single_char = [0u8];
                buf_reader.read_exact(&mut single_char)?;

                req.body.push(single_char[0]);
            }
        }

        // parse parameters
        let query = req.query;
        let contains_params = query.contains("?");
        
        let mut words = query.split("?");
        let query = words.next().unwrap();
        req.query = query.to_string();

        if !contains_params {
            return Ok(req);
        } 

        let param_split= words
            .next()
            .unwrap()
            .split("&"); 

        for param in param_split {
            let param = Param::new(param);

            if param.is_some() {
                req.params.push(param.unwrap());
            }
        }

        Ok(req)
    }
}

