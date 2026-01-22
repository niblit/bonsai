#![allow(clippy::multiple_crate_versions)] // leptos has lots of dependencies to warn about, nothing to do in my end
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-zinc-900">
            <img
                src="/static/bonsai.svg"
                alt="Bonsai Chess Logo"
                class="w-1/4 h-1/4"
            />
        </div>
    }
}
