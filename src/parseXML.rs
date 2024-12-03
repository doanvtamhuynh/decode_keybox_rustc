use quick_xml::{Reader, events::Event};
use std::io::Cursor;

pub fn parse_xml(keybox: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut reader = Reader::from_str(keybox);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag_name = e.name();
                if tag_name == b"PrivateKey" || tag_name == b"Certificate" {
                    let mut text_buf = Vec::new();  
                    if let Ok(text) = reader.read_text(tag_name, &mut text_buf) {
                        let cleaned_text = clean_key_or_certificate(&text);
                        result.push(cleaned_text); 
                    }
                }
            }
            Ok(Event::Eof) => break, 
            Err(_) => break,
            _ => {}
        }
        buf.clear(); 
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
