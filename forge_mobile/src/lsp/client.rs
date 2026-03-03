use serde_json::{json, Value};
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct LspClient {
    stream: TcpStream,
}

impl LspClient {
    pub fn new(address: &str) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(address)?;
        Ok(LspClient { stream })
    }

    pub fn initialize(&mut self) -> Result<Value, Box<dyn Error>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": { 
                "processId": None,
                "rootUri": None,
                "capabilities": {},
            },
        });
        self.send_request(&request)
    }

    pub fn completion(&mut self, text_document: &str, position: (i32, i32)) -> Result<Value, Box<dyn Error>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/completion",
            "params": {
                "textDocument": { "uri": text_document },
                "position": {
                    "line": position.0,
                    "character": position.1,
                },
            },
        });
        self.send_request(&request)
    }

    pub fn did_open(&mut self, text_document: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let notification = json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": text_document,
                    "languageId": "rust",
                    "version": 1,
                    "text": content,
                },
            },
        });
        self.send_notification(&notification)
    }

    pub fn did_change(&mut self, text_document: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let notification = json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": { "uri": text_document, "version": 2 },
                "contentChanges": [{ "text": content }],
            },
        });
        self.send_notification(&notification)
    }

    pub fn hover(&mut self, text_document: &str, position: (i32, i32)) -> Result<Value, Box<dyn Error>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "textDocument/hover",
            "params": {
                "textDocument": { "uri": text_document },
                "position": {
                    "line": position.0,
                    "character": position.1,
                },
            },
        });
        self.send_request(&request)
    }

    pub fn goto_definition(&mut self, text_document: &str, position: (i32, i32)) -> Result<Value, Box<dyn Error>> {
        let request = json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "textDocument/definition",
            "params": {
                "textDocument": { "uri": text_document },
                "position": {
                    "line": position.0,
                    "character": position.1,
                },
            },
        });
        self.send_request(&request)
    }

    fn send_request(&mut self, request: &Value) -> Result<Value, Box<dyn Error>> {
        let request_str = request.to_string();
        let content_length = request_str.len();
        let header = format!("Content-Length: {}\r\n\r\n", content_length);

        self.stream.write_all(header.as_bytes())?;
        self.stream.write_all(request_str.as_bytes())?;
        self.read_response()
    }

    fn send_notification(&mut self, notification: &Value) -> Result<(), Box<dyn Error>> {
        let notification_str = notification.to_string();
        let content_length = notification_str.len();
        let header = format!("Content-Length: {}\r\n\r\n", content_length);

        self.stream.write_all(header.as_bytes())?;
        self.stream.write_all(notification_str.as_bytes())?;
        Ok(())
    }

    fn read_response(&mut self) -> Result<Value, Box<dyn Error>> {
        let mut buffer = vec![0; 1024];
        let length = self.stream.read_exact(&mut buffer)?;
        let response: Value = serde_json::from_slice(&buffer[..length])?;
        Ok(response)
    }
}
