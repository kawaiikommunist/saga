pub mod bind {
    wit_bindgen::generate!({
        path: "wit/adder/world.wit",
    });

    use super::AdderComponent;
    export!(AdderComponent);
}

struct AdderComponent;

use bind::exports::docs;

impl docs::adder::add::Guest for AdderComponent {
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }
}
