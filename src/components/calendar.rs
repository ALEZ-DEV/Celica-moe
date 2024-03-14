use leptos::{CollectView, component, create_signal, IntoView, ReadSignal, SignalGet, SignalUpdate, spawn_local, view, WriteSignal};
use crate::components::base::DEFAULT_TAILWIND_CLASS;
use crate::huaxu::api::fetch_calendar;
use crate::huaxu::models::calendar::Calendar;

#[component]
pub fn CalendarComponent() -> impl IntoView {

    let (calendar, set_calendar) = create_signal(None::<Calendar>);

    spawn_local(async move {
        let future_calendar = fetch_calendar().await;

        if let Ok(new_c) = future_calendar {
            set_calendar.update(|mut c| *c = Some(new_c));
        }
    });

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
                        let class = "flex flex-col col-start-1 row-start-1".to_string();
                        match calendar() {
                            Some(c) =>  format!("{} grid-rows-{}", class, c.data.entries.len()),
                            None => class,
                        }
                    }>
                    {move || match calendar() {
                        Some(c) => c.data.entries.iter().enumerate().map(|(i, d)| view! {
                                <div class={if i == 0 { "h-14 mt-5" } else { "h-14" }}>
                                    <div class={format!("h-12 my-1.5 rounded-lg bg-fixed bg-right bg-local child-with-margin-{}", i)} style={format!("background-image: url('{}')", d.get_banner_link())}>
                                        <div class="h-full rounded-lg bg-transparent px-2 text-left text-white font-bold bg-gradient-to-r from-black">
                                            {&d.name}
                                            <br></br>
                                            {d.time_left().to_string()}
                                            /
                                            {d.time_passed().to_string()}
                                        </div>
                                    </div>
                                </div>
                                <style>{format!(r#"
                                  .child-with-margin-{} {{
                                    margin-right: calc(100% / 5 * {});
                                    margin-left: calc(100% / 5 * {});
                                  }}"#, i,
                                        if d.time_left().is_negative() { 0 } else { d.time_left() },
                                        if d.time_passed().is_negative() { 0 } else { d.time_passed() })}
                                </style>
                        }).collect_view(),
                        None => view! {
                            <span class="loading loading-spinner loading-lg m-auto"></span>
                        }.into_view()
                    }}
                </div>

                <div class="#h-full w-full col-start-1 row-start-1">
                    <div class="bg-base-content w-[2px] bg-base-content col-span-1 h-full mx-auto"></div>
                </div>
            </div>
        </div>
    }
}