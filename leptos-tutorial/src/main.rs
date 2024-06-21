use leptos::*;
use leptos::ev::MouseEvent;

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled? " {toggled}</p>
        <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}


#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView
{
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}

fn main() {
    leptos::mount_to_body(App)
}