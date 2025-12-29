use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    "Abhinandh S".to_string()
}

#[wasm_bindgen]
pub fn about_me() -> String {
    "I'm Abhinandh S. I am a 21 old guy from India, who loves computers and softwares.
    This place is home for all my psychological dysfunctioning. A place where I am in control, with no censorship or manupilation.
".to_string()
}
