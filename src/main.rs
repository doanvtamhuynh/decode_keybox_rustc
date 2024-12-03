use std::fs::File;
use std::io::Read;
use std::path::Path;
use base64::{decode, encode};

static mut EC_PRIVATE_KEY: &[u8] = &[];
static mut CERTIFICATE_1: &[u8] = &[];
static mut CERTIFICATE_2: &[u8] = &[];
static mut CERTIFICATE_3: &[u8] = &[];

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
        let keybox = "keypriv|key1|key2|key3";


        let mut ec_private_key = "";
        let mut certificate_1 = "";
        let mut certificate_2 = "";
        let mut certificate_3 = "";
        
        let parts: Vec<&str> = keybox.split('|').collect();
        if parts.len() == 4 {
            ec_private_key = parts[0];
            certificate_1 = parts[1];
            certificate_2 = parts[2];
            certificate_3 = parts[3];
        }
    
        println!("EC Private Key: {}", ec_private_key);
        println!("Certificate 1: {}", certificate_1);
        println!("Certificate 2: {}", certificate_2);
        println!("Certificate 3: {}", certificate_3);
        
        let base64_string = "c2FmZGRmZ3w=";
        let decoded_bytes = base64::decode(base64_string).expect("Invalid base64 string");
        
        let hex_array: Vec<String> = decoded_bytes.iter()
            .map(|byte| format!("0x{:02x}", byte))
            .collect();
    
        let hex_output = hex_array.join(", ");
        println!("{}", hex_output);

        return true;
    }
}

fn main() {
    if set_keybox() {
        println!("true")
    } else {
        println!("false")
    }
    println!("{}", read_keybox_file())
}
