use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <progress
            max="50"
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=double_count
        />
        <p>
            "Double Count: "
            // and again here
            {double_count}
        </p>
        <button
            on:click={move |_| {
                set_x.update(|n| *n += 10);
            }}
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click to Move"
        </button>
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.update(|n| *n += 1);
            }
            class:red=move || count() % 2 == 1
        >
            "Click me: "
            // on stable, this is move || count.get();
            {count}
        </button>
    }
}