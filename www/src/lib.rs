use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compare(regex1: &str, regex2: &str) -> String {
    match regcmp::compare(regex1, regex2) {
        Ok(true) => "等价".to_string(),
        Ok(false) => "不等价".to_string(),
        Err(e) => format!("错误: {}", e),
    }
}

#[wasm_bindgen]
pub fn compare_verbose(regex1: &str, regex2: &str) -> String {
    regcmp::compare_verbose(regex1, regex2)
}
