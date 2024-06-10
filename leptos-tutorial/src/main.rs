use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    /* ... */
    view! {
        <progress
            max=100
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
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}