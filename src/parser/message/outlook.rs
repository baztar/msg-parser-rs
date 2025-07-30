use std::path::Path;

use super::{Attachment, Header};
use crate::ole;
use crate::parser::storage::{Properties, Storages};

#[derive(Debug)]
pub struct Outlook {
    pub headers: Option<Vec<Header>>,
    pub body: String,
    pub html: Option<String>,
    pub attachments: Vec<Attachment>,
}

impl Outlook {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, crate::Error> {
        let data = std::fs::read(path)?;

        Self::from_slice(&data)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, crate::Error> {
        let reader = ole::Reader::new(slice)?;
        let mut storages = Storages::new(&reader);
        storages.process_streams(&reader);

        println!("{:?}", storages.attachments);

        let message = Self::parse(storages);

        Ok(message)
    }

    fn parse(storages: Storages) -> Self {
        let headers = Self::parse_headers(&storages.root);
        let body = storages.root.get("Body").map(|dt| dt.into()).unwrap_or_default();
        let html = storages.root.get("Body").map(|dt| dt.into());
        let attachments = Self::extract_attachments(&storages.attachments);

        Self {
            headers,
            body,
            html,
            attachments,
        }
    }

    fn parse_headers(root_storage: &Properties) -> Option<Vec<Header>> {
        let raw: String = root_storage.get("TransportMessageHeaders").map(|h| h.into())?;

        let mut headers = Vec::new();
        let mut current: String = String::new();
        for line in raw.lines() {
            let mut chars = line.chars();
            if let Some(first) = chars.next() {
                if first.is_whitespace() {
                    current.push_str(line);
                } else {
                    if let Some((key, value)) = current.split_once(':') {
                        headers.push(Header::new(key.trim(), value.trim()));
                    }
                    current.clear();

                    current.push_str(line);
                }
            } else {
                continue;
            }
        }

        Some(headers)
    }

    fn extract_attachments(attachments: &[Properties]) -> Vec<Attachment> {
        attachments.iter().filter_map(Attachment::from_properties).collect()
    }
}
