use leptos::prelude::*;

#[component]
pub fn Console() -> impl IntoView {
    view! {
        <section class="console">
            <textarea readonly=true />
        </section>
    }
}
