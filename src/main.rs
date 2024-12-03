use std::fs::File;
use std::io::Read;
use std::path::Path;
use base64::{decode, encode};
mod parseXML;
mod convert_key;

static mut EC_PRIVATE_KEY: Vec<u8> = vec![];
static mut CERTIFICATE_1: Vec<u8> = vec![];
static mut CERTIFICATE_2: Vec<u8> = vec![];
static mut CERTIFICATE_3: Vec<u8> = vec![];

const ROOT_DIR: &str = env!("CARGO_MANIFEST_DIR");
const KEYBOX_PATH: &str = r"\res\keybox.xml";

fn read_keybox_file() -> String {
    let path_str = format!("{}{}", ROOT_DIR, KEYBOX_PATH);
    let path = Path::new(&path_str);
    if !path.exists() {
        return "".to_string();  
    }

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return "".to_string(), 
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return "".to_string(); 
    }

    contents 
}

fn set_keybox() -> bool {
    if read_keybox_file().is_empty(){
        return false;
    }
    else{
        unsafe{
            let keyboxs = parseXML::parse_xml(read_keybox_file().as_str());
            let ec_private_key = &keyboxs[0];
            let certificate_1 = &keyboxs[1];
            let certificate_2 = &keyboxs[2];
            let certificate_3 = &keyboxs[3];
            EC_PRIVATE_KEY = convert_key::base64_to_bytes(ec_private_key.as_str());;
            CERTIFICATE_1 = convert_key::base64_to_bytes(certificate_1.as_str());
            CERTIFICATE_2 = convert_key::base64_to_bytes(certificate_2.as_str());;
            CERTIFICATE_3 = convert_key::base64_to_bytes(certificate_3.as_str());;
        }
        return true;
    }
}

fn main() {
    if set_keybox(){
        unsafe {
            println!("EC_PRIVATE_KEY: {:?}", EC_PRIVATE_KEY);
            println!("CERTIFICATE_1: {:?}", CERTIFICATE_1);
            println!("CERTIFICATE_2: {:?}", CERTIFICATE_2);
            println!("CERTIFICATE_3: {:?}", CERTIFICATE_3);
        }
    }
}
