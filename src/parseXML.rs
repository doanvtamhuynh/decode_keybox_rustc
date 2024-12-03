use xml::reader::{EventReader, XmlEvent};
use std::io::Cursor;

pub fn parse_xml(keybox: &str) -> Vec<String> {
    let mut result = Vec::new();
    let parser = EventReader::from_str(keybox);

    let mut inside_private_key_or_certificate = false;

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let tag_name = name.local_name.as_str();
                if tag_name == "PrivateKey" || tag_name == "Certificate" {
                    inside_private_key_or_certificate = true;
                }
            }
            Ok(XmlEvent::Characters(text)) if inside_private_key_or_certificate => {
                let cleaned_text = clean_key_or_certificate(&text);
                result.push(cleaned_text);
                inside_private_key_or_certificate = false;
            }
            Ok(XmlEvent::EndElement { name }) => {
                let tag_name = name.local_name.as_str();
                if tag_name == "PrivateKey" || tag_name == "Certificate" {
                    inside_private_key_or_certificate = false;
                }
            }
            Ok(XmlEvent::EndDocument) | Err(_) => break,
            _ => {}
        }
    }

    result
}

fn clean_key_or_certificate(text: &str) -> String {
    let mut cleaned = text.to_string();

    cleaned = cleaned.replace("-----BEGIN EC PRIVATE KEY-----", "")
                     .replace("-----END EC PRIVATE KEY-----", "")
                     .replace("-----BEGIN CERTIFICATE-----", "")
                     .replace("-----END CERTIFICATE-----", "")
                     .replace("-----BEGIN RSA PRIVATE KEY-----", "")
                     .replace("-----END RSA PRIVATE KEY-----", "");
    cleaned = cleaned.replace("\n", "").replace("\r", "");
    cleaned = cleaned.trim().to_string();
    cleaned
}
