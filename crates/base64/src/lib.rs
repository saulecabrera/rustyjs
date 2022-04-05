pub mod b64;

wit_bindgen_rust::export!("../../wit/base64.wit");


struct Base64 { }

impl base64::Base64 for Base64 {
    fn encode(input: String) -> String {
        crate::b64::encode(input)
    }
}

