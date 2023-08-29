use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <ul>{"Hello World"}</ul>
    }

}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}