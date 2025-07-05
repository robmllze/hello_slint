use wasm_bindgen::prelude::*;

slint::include_modules!();

#[wasm_bindgen(start)]
pub fn run_app() {
    let ui = App::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.on_button_pressed(move || {
        let ui = ui_handle.upgrade().unwrap();
        let current_counter = ui.get_counter();
        ui.set_counter(current_counter + 1);
    });

    ui.run().unwrap();
}