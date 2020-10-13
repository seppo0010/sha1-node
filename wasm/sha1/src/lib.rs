use sha1::Sha1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn digest(string: &str) -> Option<String> {
    let mut m = Sha1::new();
    m.update(string.as_bytes());
    let dgst = m.digest().to_string();

    Some(dgst)
}
