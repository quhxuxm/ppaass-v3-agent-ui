use leptos::prelude::*;

pub enum NavigationBarDirection {
    Vertical,
    Horizontal,
}

#[component]
pub fn NavigationBar(direction: NavigationBarDirection) -> impl IntoView {
    match direction {
        NavigationBarDirection::Vertical => {
            view! {
                <div>
                    "Ppaass Agent"
                </div>
            }
        }
        NavigationBarDirection::Horizontal => {
            view! {
                <div>
                    "Ppaass Agent"
                </div>
            }
        }
    }
}
