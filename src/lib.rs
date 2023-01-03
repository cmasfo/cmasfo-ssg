
#![allow(dead_code)]
#![allow(unused_variables)]

struct SiteConfig {

  port: u16,

}

impl SiteConfig {

  fn default() -> Self {

    Self {
      port: 7878,
    }

  }

  fn init(
    config_file: &str,
  ) -> Self {

    println!("Reading config is not implemented yet. Using default config.");

    Self::default()

  }

}

pub struct SiteBuilder {

  config: SiteConfig,

}

impl SiteBuilder {
  
  pub fn run(
    site_base: &str,
  ) {

    let config = SiteConfig::init("config.toml");

    let builder = Self::init(config);

    builder.serve();

  }

  fn init(config: SiteConfig) -> Self {

    Self {
      config,
    }

  }

}

impl SiteBuilder {

  fn serve(&self) {

    use std::{
      io::{prelude::*, BufReader},
      net::{TcpListener},
    };

    let url = format!("127.0.0.1:{}", self.config.port);

    println!("Serving the website: {url}");

    let listener = TcpListener::bind(url).unwrap();

    for stream in listener.incoming() {
      let mut stream = stream.unwrap();

      let buf_reader = BufReader::new(&mut stream);

      let _http_request: Vec<_>
      = buf_reader
      .lines()
      .map(|result| result.unwrap())
      .take_while(|line| !line.is_empty())
      .collect();
      
      let status_line = "HTTP/1.1 200 OK";
      let contents = "Welcome to CMASFO SSG";
      let length = contents.len();

      let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
      );

      stream.write_all(response.as_bytes()).unwrap();

    }

  }  

}
