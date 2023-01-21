use tossicat::{modify_sentence, postfix};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fix(word: &str, tossi: &str) -> Result<String, JsError> {
    match postfix(word, tossi) {
        Ok(temp) => Ok(temp),
        Err(e) => Err(JsError::new(&e.to_string())),
    }
}

#[wasm_bindgen]
pub fn fix_sentence(sentence: &str) -> Result<String, JsError> {
    match modify_sentence(sentence) {
        Ok(temp) => Ok(temp),
        Err(e) => Err(JsError::new(&e.to_string())),
    }
}
