#![allow(dead_code)]

use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use webkit::prelude::*;

pub fn base_config() -> WebViewConfiguration {
    let config = WebViewConfiguration::new();
    config.use_nonpersistent_data_store();
    config
}

pub fn load_html(view: &WebView, body: &str, base_url: &str) -> Result<(), Box<dyn Error>> {
    let html = format!(
        r#"<!doctype html><html><head><meta charset=\"utf-8\"><title>webkit-rs</title></head><body>{body}</body></html>"#
    );
    view.load_html(&html, Some(base_url))?;
    Ok(())
}

pub fn artifact_dir(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(name);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn wait_for(timeout: Duration, mut predicate: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if predicate() {
            return true;
        }
        webkit::pump_run_loop(0.05);
        thread::sleep(Duration::from_millis(10));
    }
    predicate()
}

pub fn start_attachment_server(
    body: Vec<u8>,
    file_name: &str,
    content_type: &str,
) -> Result<(String, JoinHandle<()>), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let url = format!("http://{address}/download");
    let file_name = file_name.to_owned();
    let content_type = content_type.to_owned();
    let handle = thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut request = [0_u8; 1024];
            let _ = stream.read(&mut request);
            let headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nContent-Disposition: attachment; filename=\"{file_name}\"\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(headers.as_bytes());
            let _ = stream.write_all(&body);
            let _ = stream.flush();
        }
    });
    Ok((url, handle))
}
