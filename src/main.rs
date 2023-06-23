use std::{io::{Write, Read}, process::{Stdio, Child}, path::Path};
use serde_json::{json, Value};
use lsp_types::*;

mod utils;

pub struct Server {
    server: Child
}
impl Server {
    pub fn new() -> Self {
        Self {
            server: std::process::Command::new("rust-analyzer")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn().unwrap()
        }
    }
    pub fn write(&mut self, v: Value) {
        let parsed = serde_json::to_string(&v).unwrap();
        println!("sending: {parsed}");
        self.server.stdin.as_mut().unwrap().write_all(
            format!("Content-Length: {}\r\n\r\n{parsed}", parsed.len()).as_bytes()
        ).unwrap();
    }
    pub fn read(&mut self) {
        loop {
            let mut buf = String::new();
            self.server.stdout.as_mut().unwrap().read_to_string(&mut buf).unwrap();
            if buf.len() > 0 {
                println!("received: {buf}")
            }
        }
    }
    pub fn init(&mut self, root: &Path) {
        self.write(json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": json!(InitializeParams {
                process_id: Some(1),
                client_info: Some(ClientInfo {
                    name: "doro".to_string(),
                    version: Some("3.17.0".to_string())
                }),
                root_uri: Some(Url::from_directory_path(root).unwrap()),
                trace: Some(TraceValue::Verbose),
                ..Default::default()
            })
        }));
        self.write(json!({
            "method": "initialized"
        }));
    }
    pub fn didopen(&mut self, path: &Path) {
        let text = std::fs::read_to_string(path).unwrap();
        self.write(json!({
            "method": "textDocument/didOpen",
            "parms": DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: Url::from_file_path(path).unwrap(),
                    language_id: utils::get_file_language_id(path),
                    text,
                    version: 1
                }
            }
        }));
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.server.wait().unwrap();
    }
}

fn main() {
    let mut server = Server::new();
    server.init(Path::new("C://Users/User09/Documents/GitHub/doro"));
    server.didopen(Path::new("C://Users/User09/Documents/GitHub/doro/src/main.rs"));
    server.read()
}