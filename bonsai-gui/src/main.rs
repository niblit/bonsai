#![allow(clippy::multiple_crate_versions)]

use leptos::prelude::*;

use bonsai_gui::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
