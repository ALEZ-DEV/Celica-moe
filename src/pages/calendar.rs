use leptos::{component, create_signal, IntoView, SignalUpdate, spawn_local, view};
use crate::components::calendar::CalendarComponent;
use crate::huaxu::api::fetch_calendar;
use crate::huaxu::models::calendar::Calendar;

#[component]
pub fn CalendarPages() -> impl IntoView {

    let (calendar, set_calendar) = create_signal(None::<Calendar>);

    spawn_local(async move {
        let future_calendar = fetch_calendar().await;

        if let Ok(mut new_c) = future_calendar {
            new_c.data.filter_date();
            //new_c.data.initialize_signal_for_entries();
            set_calendar.update(|mut c| *c = Some(new_c));
        }
    });

    view! {
        <h1 class="text-center">Calendar</h1>
        <CalendarComponent calendar=calendar set_calendar=set_calendar/>
    }
}