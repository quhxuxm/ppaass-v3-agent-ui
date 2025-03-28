use leptos::prelude::*;

use crate::components::navi::{NavigationBar, NavigationBarDirection};

#[component]
pub fn PpaassAgentApplication() -> impl IntoView {
    view! {
        <main>
            <NavigationBar direction=NavigationBarDirection::Horizontal/>
            <section class="content">
                <div class="input">
                    <input type="text" placeholder="Enter command here" />
                </div>
                <div class="button">
                    <button>Send</button>
                </div>
            </section>
            <section class="console">
                <textarea readonly=true />
            </section>
        </main>
    }
}
