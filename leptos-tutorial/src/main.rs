use leptos::*;
use leptos::ev::MouseEvent;

#[component]
pub fn App() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <h3>"Button C"</h3>
        <p>"Toggled? " {toggled}</p>
        <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}


#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle"</button>
    }
}

fn main() {
    leptos::mount_to_body(App)
}