use crate::components::base::DEFAULT_COMPONENT_TAILWIND_CLASS;
use crate::components::item::ItemComponent;
use crate::huaxu::models::calendar::{Calendar, Entry};
use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::{
    component, view, Callback, CollectView, IntoView, ReadSignal, SignalUpdate, WriteSignal,
};

#[component]
pub fn CalendarComponent(
    calendar: ReadSignal<Option<Calendar>>,
    set_calendar: WriteSignal<Option<Calendar>>,
) -> impl IntoView {
    view! {
        <div class={DEFAULT_COMPONENT_TAILWIND_CLASS}>
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
                                    console_log("updating selection");
                                    c.data.entries.iter_mut().for_each(|e| e.selected = false);
                                    c.data.entries[i].selected = true;
                                    set_calendar.update(|old_c| *old_c = Some(c));
                                }
                            })/>
                        }).collect_view(),
                        None => view! {
                            {(0..4).map(|i| view! {
                                <div class=format!("skeleton w-full h-14 {}", if i == 0 { "mt-5" } else { "" })></div>
                            }).collect_view()}
                        }.into_view()
                    }}
                </div>

                {move || if calendar().is_some() {
                    view! {
                        <div class="h-full mx-auto col-start-1 row-start-1 hover:pointer-events-none">
                            <div class="bg-base-content w-[2px] bg-base-content col-span-1 h-full mx-auto"></div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }}
            </div>
        </div>
    }
}

#[component]
pub fn CalendarItemComponent(
    index: usize,
    entry: Entry,
    onclick: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <div class={if index.clone() == 0 { "h-14 mt-5" } else { "h-14" }}>
            <div
                style={format!("background-image: url('{}')", entry.get_banner_link())}
                class={
                    let class = "h-16 rounded-lg bg-fixed bg-right bg-local";
                    let child_style = format!("child-with-margin-{}", index);
                    let select_style = if entry.selected {
                        "border-2 border-white"
                    } else {
                        "hover:border-2 hover:border-white cursor-pointer"
                    };

                format!("{} {} {}", class, child_style, select_style)
                } >
                <div on:click=onclick class="h-full rounded-lg bg-transparent px-2 text-left text-white font-bold bg-gradient-to-r from-black">
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

#[component]
pub fn CalendarDetailComponent(calendar: ReadSignal<Option<Calendar>>) -> impl IntoView {
    let entry = move || {
        calendar()
            .unwrap()
            .data
            .entries
            .iter()
            .find(|e| e.selected)
            .unwrap()
            .clone()
    };

    console_log(&entry().description);

    view! {
        <div class={format!("{} mt-5", DEFAULT_COMPONENT_TAILWIND_CLASS)}>
            <h3 class="text-center" style="margin-top: 0px;" /* mt-0 was not working, why ? ¯\_(ツ)_/¯ */ >{move || entry().name}</h3>
            <div class="h-16 rounded-lg bg-fixed bg-right bg-local" style={format!("background-image: url('{}')", entry().get_banner_link())}></div>
            {move || entry().description.split("\n").map(|s| s.to_string()).map(|s| view! {
                <p>{s.clone()}</p>
            }).collect_view()}
            <div class="flex justify-start justify-items-center">
                <h3 class="m-0 my-auto">"Rewards:"</h3>
                {move || entry().items.iter().map(|item| view! {
                    <div class="mx-1.5">
                        <ItemComponent item=item/>
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
