use leptos::{component, IntoView, view};
use crate::components::event::EventComponent;

#[component]
pub fn EventsPages() -> impl IntoView {
    view! {
        <h1>This is the event page</h1>
        <EventComponent/>
    }
}