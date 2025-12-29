use wasm_bindgen::prelude::*;

pub const AGE: &str = "22";

#[wasm_bindgen]
pub fn hello() -> String {
    "Abhinandh S".to_string()
}

#[wasm_bindgen]
pub fn about_me() -> String {
    let mut text = String::new();
    text.push_str("I'm Abhinandh S. I am a ");
    text.push_str(AGE);
    text.push_str(" old guy from India, who loves computers and softwares.
    This place is home for all my psychological dysfunctioning. A place where I am in control, with no censorship or manupilation.");
text
}
