use leptos::{CollectView, component, create_signal, IntoView, ReadSignal, SignalGet, SignalUpdate, spawn_local, view, WriteSignal};
use crate::components::base::DEFAULT_TAILWIND_CLASS;
use crate::huaxu::api::fetch_calendar;
use crate::huaxu::models::calendar::Calendar;

#[component]
pub fn CalendarComponent() -> impl IntoView {

    let (calendar, set_calendar) = create_signal(None::<Calendar>);

    spawn_local(async move {
        let future_calendar = fetch_calendar().await;

        if let Ok(mut new_c) = future_calendar {
            new_c.data.filter_date();
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
                        let class = "flex flex-col space-y-4 col-start-1 row-start-1 mb-4".to_string();
                        match calendar() {
                            Some(c) =>  format!("{} grid-rows-{}", class, c.data.entries.len()),
                            None => class,
                        }
                    }>
                    {move || match calendar() {
                        Some(c) => c.data.entries.iter().enumerate().map(|(i, d)| view! {
                                <div class={if i == 0 { "h-14 mt-5" } else { "h-14" }}>
                                    <div class={format!("h-16 my-1.5 rounded-lg bg-fixed bg-right bg-local child-with-margin-{}", i)} style={format!("background-image: url('{}')", d.get_banner_link())}>
                                        <div class="h-full rounded-lg bg-transparent px-2 text-left text-white font-bold bg-gradient-to-r from-black">
                                            <h3 class="my-0">{&d.name}</h3>
                                            {if d.has_left(0) {
                                                view! {
                                                    <h4>Ended</h4>
                                                }
                                            } else {
                                                view! {
                                                    <h4>Time left : {d.time_left()} days</h4>
                                                }
                                            }}
                                        </div>
                                    </div>
                                </div>
                                <style>{format!(r#"
                                  .child-with-margin-{} {{
                                    margin-right: calc(100% / 5 * {});
                                    margin-left: calc(100% / 5 * {});
                                  }}"#, i,
                                        if d.time_left() < 3 { 3 - d.time_left() } else { 0 },
                                        if d.time_passed() < 3 { 3 - d.time_passed() } else { 0 })}
                                </style>
                        }).collect_view(),
                        None => view! {
                            <span class="loading loading-spinner loading-lg m-auto"></span>
                        }.into_view()
                    }}
                </div>

                <div class="h-full w-full col-start-1 row-start-1">
                    <div class="bg-base-content w-[2px] bg-base-content col-span-1 h-full mx-auto"></div>
                </div>
            </div>
        </div>
    }
}