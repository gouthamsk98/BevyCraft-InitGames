use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn get_cursor_type() -> String;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_cursor_type() -> String {
    "Cuboid".to_string()
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn get_mesh_type() -> String;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_mesh_type() -> String {
    "Cube".to_string()
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn get_tool_type() -> String;
}
#[cfg(not(target_arch = "wasm32"))]
pub fn get_tool_type() -> String {
    "None".to_string()
}
