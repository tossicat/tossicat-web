use tossicat::{modify_sentence, postfix};
use wasm_bindgen::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn fix(word: &str, tossi: &str) {
    let temp = postfix(word, tossi);
    let test = match temp {
        Err(e) => Err(e),
        Ok(temp) => Ok(temp),
    };

    alert(&format!(
        "단어:{}, 토시:{}의 결과는 {:?}!",
        word, tossi, test
    ));
}

#[wasm_bindgen]
pub fn fix_sentence(sentence: &str) {
    let temp = modify_sentence(sentence);
    let test = match temp {
        Err(e) => Err(e),
        Ok(temp) => Ok(temp),
    };

    alert(&format!("{} : {:?}!", sentence, test));
}
