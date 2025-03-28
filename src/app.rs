use leptos::prelude::*;
use stylance::import_crate_style;

use crate::components::{
    console::Console,
    nav::{NavigationBar, NavigationBarDirection},
};

import_crate_style!(application_style, "src/app.module.scss");

#[component]
pub fn PpaassAgentApplication() -> impl IntoView {
    view! {
        <main>
            <NavigationBar direction=NavigationBarDirection::Horizontal/>
            <section class=application_style::container>
                <div class="input">
                    <label for="agent_server_port_input">"Agent server port:"</label>
                    <input id="agent_server_port_input" type="text" placeholder="Agent server port" />
                </div>
                <div class="input">
                    <label for="worker_thread_number_input">"Worker thread number:"</label>
                    <input id="worker_thread_number_input" type="text" placeholder="Worker thread number" />
                </div>
                <div class="input">
                    <label for="user_select">"Select user: "</label>
                    <select id="user_select">
                        <option value="0">"User1"</option>
                        <option value="1">"User2"</option>
                    </select>
                </div>
                <div class="input">
                    <label for="log_level_select">"Select log level: "</label>
                    <select id="log_level_select">
                        <option value="0">"INFO"</option>
                        <option value="1">"ERROR"</option>
                    </select>
                </div>
                <div class="input">
                    <label for="max_pool_size_input">"Max pool size:"</label>
                    <input id="max_pool_size_input" type="text" placeholder="Max pool size" />
                </div>
                <div class="button">
                    <button>"Import user"</button>
                    <button>"Start"</button>
                    <button>"Stop"</button>
                </div>
            </section>
            <Console />
        </main>
    }
}
