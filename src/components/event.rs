use leptos::{CollectView, component, create_signal, IntoView, ReadSignal, SignalGet, SignalUpdate, spawn_local, view, WriteSignal};
use crate::components::base::DEFAULT_TAILWIND_CLASS;
use crate::huaxu::api::fetch_calendar;
use crate::huaxu::models::calendar::Calendar;

#[component]
pub fn EventComponent() -> impl IntoView {

    let (calendar, set_calendar) = create_signal(None::<Calendar>);

    spawn_local(async move {
        let future_calendar = fetch_calendar().await;

        if let Ok(new_c) = future_calendar {
            set_calendar.update(|mut c| *c = Some(new_c));
        }
    });

    view! {
        <div class={DEFAULT_TAILWIND_CLASS}>
            <div class="h-full w-[2px] bg-primary relative flex justify-center"></div>
            <div class="grid grid-cols-1 grid-rows-2">
                {move || match calendar() {
                    Some(c) => c.data.entries.iter().map(|i| view! {
                        <h3>{&i.name}</h3>
                    }).collect_view(),
                    None => view! {
                        <span class="loading loading-spinner loading-lg m-auto"></span>
                    }.into_view()
                }}
                //<div class="h-24 bg-secondary"></div>
            </div>
        </div>
    }
}