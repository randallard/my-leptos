use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn ProgressBar(
    #[prop(default = 100)]
    max: u16,
    progress: impl Fn() -> i32 + 'static
) -> impl IntoView {
    view! {
        <progress
            max="50"
            value=progress
            style="display: block"
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let html = "<p>This HTML will be injected.</p>";
    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {count}
        </button>
        <ProgressBar progress=count />
        <ProgressBar progress=double_count />
        <div inner_html=html/>
    }
}