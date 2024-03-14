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
            <div class="bg-base-content w-[2px] bg-base-content absolute inset-x-2/3" style={move || {
                            // set the height based on the number of item in the grid, one item height is 14 tailwind height * 0.25rem
                            let style = "height: ";
                            let height = match calendar() {
                                Some(c) => format!("{}rem;", c.data.entries.len() as f32 * 14.0 * 0.25),
                                None => "0rem;".to_string(),
                            };
                            format!("{} {}", style, height)
                        }}></div>
            <div class="grid grid-cols-1 grid-rows-2">
                {move || match calendar() {
                    Some(c) => c.data.entries.iter().map(|i| view! {
                            <div class="h-14">
                                <div class="h-12 my-1.5 rounded-lg bg-fixed bg-right bg-local" style="background-image: url('https://static.miraheze.org/pgrwiki/c/c3/Bannercweave.png')">
                                    <div class="h-full rounded-lg bg-transparent px-2 text-left text-white font-bold bg-gradient-to-r from-black">
                                        {&i.name}
                                    </div>
                                </div>
                            </div>
                    }).collect_view(),
                    None => view! {

                        <span class="loading loading-spinner loading-lg m-auto"></span>
                    }.into_view()
                }}
            </div>
        </div>
    }
}