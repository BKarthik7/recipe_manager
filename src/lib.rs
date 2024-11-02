mod recipe;
mod manager;
mod ui; // Make sure this contains your RecipeManagerGUI
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Call run without arguments
    ui::run();
    Ok(())
}
