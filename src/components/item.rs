use leptos::*;
use crate::huaxu::models::calendar::Item;

#[component]
pub fn ItemComponent<'a>(item: &'a Item) -> impl IntoView {
    view! {
        <div class="h-14 w-14 rounded-lg bg-base-100">
            <img src={item.get_icon_link()} class="m-0 p-1 rounded-lg"/>
        </div>
    }
}