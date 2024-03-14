use leptos::{component, IntoView, view};
use crate::components::calendar::CalendarComponent;

#[component]
pub fn CalendarPages() -> impl IntoView {
    view! {
        <h1>This is the event page</h1>
        <CalendarComponent/>
    }
}