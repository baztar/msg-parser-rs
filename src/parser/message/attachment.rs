use crate::parser::storage::Properties;

#[derive(Debug)]
pub struct Attachment {
    pub display_name: String,             // "DisplayName"
    pub payload: String,                  // "AttachDataObject"
    pub extension: Option<String>,        // "AttachExtension"
    pub mime_tag: Option<String>,         // "AttachMimeTag"
    pub file_name: Option<String>,        // "AttachFilename"
    pub content_id: Option<String>,       // "AttachContentId"
    pub content_location: Option<String>, // "AttachContentLocation"
}

impl Attachment {
    pub fn from_properties(properties: &Properties) -> Option<Self> {
        Some(Self {
            display_name: Self::property_value(properties, "DisplayName")?,
            payload: Self::property_value(properties, "AttachDataObject")?,
            extension: Self::property_value(properties, "AttachExtension"),
            mime_tag: Self::property_value(properties, "AttachMimeTag"),
            file_name: Self::property_value(properties, "AttachFilename"),
            content_id: Self::property_value(properties, "AttachContentId"),
            content_location: Self::property_value(properties, "AttachContentLocation"),
        })
    }

    fn property_value(properties: &Properties, name: &str) -> Option<String> {
        properties.get(name).map(|dt| dt.into())
    }
}
