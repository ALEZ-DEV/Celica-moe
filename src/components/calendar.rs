use leptos::{Callable, Callback, CollectView, component, create_signal, IntoView, ReadSignal, SignalGet, SignalUpdate, spawn_local, view, WriteSignal};
use leptos::ev::MouseEvent;
use crate::components::base::DEFAULT_TAILWIND_CLASS;
use crate::huaxu::api::fetch_calendar;
use crate::huaxu::models::calendar::{Calendar, Entry};

#[component]
pub fn CalendarComponent(calendar: ReadSignal<Option<Calendar>>, set_calendar: WriteSignal<Option<Calendar>>) -> impl IntoView {

    view! {
        <div class={DEFAULT_TAILWIND_CLASS}>
            <div class="grid grid-cols-5 text-center mb-0">
                <div class="badge mx-auto badge-outline">"2 days ago"</div>
                <div class="badge mx-auto badge-outline">"Yesterday"</div>
                <div class="badge mx-auto badge-neutral">"Today"</div>
                <div class="badge mx-auto badge-outline">"Tomorrow"</div>
                <div class="badge mx-auto badge-outline">"In 2 days"</div>
            </div>
            <div class="grid grid-rows-1 w-full">
                <div class={
                        let class = "flex flex-col space-y-4 col-start-1 row-start-1 mb-4".to_string();
                        match calendar() {
                            Some(c) =>  format!("{} grid-rows-{}", class, c.data.entries.len()),
                            None => class,
                        }
                    }>
                    {move || match calendar() {
                        Some(c) => c.data.entries.iter().enumerate().map(|(i, d)| view! {
                            <CalendarItemComponent index=i entry=d.clone() onclick=Callback::from(move |_| {
                                if let Some(mut c) = calendar() {
                                    c.data.entries.iter_mut().for_each(|e| e.selected = false);
                                    c.data.entries[i].selected = true;
                                    set_calendar.update(|mut old_c| *old_c = Some(c));
                                }
                            })/>
                        }).collect_view(),
                        None => view! {
                            <span class="loading loading-spinner loading-lg m-auto"></span>
                        }.into_view()
                    }}
                </div>

                <div class="h-full w-full col-start-1 row-start-1 hover:pointer-events-none">
                    <div class="bg-base-content w-[2px] bg-base-content col-span-1 h-full mx-auto"></div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CalendarItemComponent(index: usize, entry: Entry, onclick: Callback<MouseEvent>) -> impl IntoView {
    view! {
        <div
            on:click=onclick
            class={if index.clone() == 0 { "h-14 mt-5" } else { "h-14" }}>
            <div
                style={format!("background-image: url('{}')", entry.get_banner_link())}
                class={
                    let class = "h-16 my-1.5 rounded-lg bg-fixed bg-right bg-local";
                    let child_style = format!("child-with-margin-{}", index);
                    let select_style = if entry.selected {
                        "border-2 border-white"
                    } else {
                        "hover:border-2 hover:border-white"
                    };

                format!("{} {} {}", class, child_style, select_style)
                } >
                <div class="h-full rounded-lg bg-transparent px-2 text-left text-white font-bold bg-gradient-to-r from-black">
                    <h3 class="my-0">{&entry.name}</h3>
                    {if entry.has_left(0) {
                        view! {
                            <h4>Ended</h4>
                        }
                    } else {
                    if entry.has_few_hour_left() {
                    view! {
                        <h4>Time left : few hours left</h4>
                    }
                    } else {
                        view! {
                            // add 1 because when a day is like: 1 days and 5 hours,
                            // we want to count the ignored hours, so we add 1
                            <h4>Time left : {entry.time_left() + 1} days</h4>
                        }
                    }
                    }}
                </div>
            </div>
        </div>
        <style>{format!(r#"
              .child-with-margin-{} {{
                margin-right: calc(100% / 5 * {});
                margin-left: calc(100% / 5 * {});
              }}"#, index,
                if entry.time_left() < 3 { 3 - entry.time_left() } else { 0 },
                if entry.time_passed() < 3 { 3 - entry.time_passed() } else { 0 })}
        </style>
    }
}