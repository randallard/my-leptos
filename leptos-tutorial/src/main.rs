use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| panic!("something");)
}