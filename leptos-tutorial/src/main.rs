use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
// create a list of 5 signals
let length = 5;
let counters = (1..=length).map(|idx| create_signal(idx));

// each item manages a reactive view
// but the list itself will never change
let counter_buttons = counters
    .map(|(count, set_count)| {
        view! {
            <li>
                <button
                    on:click=move |_| set_count.update(|n| *n += 1)
                >
                    {count}
                </button>
            </li>
        }
    })
    .collect_view();

view! {
    <ul>{counter_buttons}</ul>
}
}