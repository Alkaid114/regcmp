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
pub fn compare_fsm(fsm1: &str, fsm2: &str) -> String {
    match regcmp::compare_fsm(fsm1, fsm2) {
        Ok(true) => "等价".to_string(),
        Ok(false) => "不等价".to_string(),
        Err(e) => format!("错误: {}", e),
    }
}

#[wasm_bindgen]
pub fn compare_verbose(regex1: &str, regex2: &str) -> String {
    regcmp::compare_verbose(regex1, regex2)
}

#[wasm_bindgen]
pub fn compare_fsm_verbose(fsm1: &str, fsm2: &str) -> String {
    regcmp::compare_fsm_verbose(fsm1, fsm2)
}

#[wasm_bindgen]
pub fn compare_typed(input1: &str, type1: &str, input2: &str, type2: &str) -> String {
    let f1 = type1 == "fsm";
    let f2 = type2 == "fsm";
    match regcmp::compare_with_type(input1, f1, input2, f2) {
        Ok(true) => "等价".to_string(),
        Ok(false) => "不等价".to_string(),
        Err(e) => format!("错误: {}", e),
    }
}

#[wasm_bindgen]
pub fn compare_typed_verbose(input1: &str, type1: &str, input2: &str, type2: &str) -> String {
    let f1 = type1 == "fsm";
    let f2 = type2 == "fsm";
    regcmp::compare_verbose_with_type(input1, f1, input2, f2)
}
