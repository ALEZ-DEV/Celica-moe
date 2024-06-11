use leptos::{
    component, create_signal, leptos_dom::logging::console_log, spawn_local, view, IntoView,
    SignalUpdate,
};

use crate::huaxu::{
    api::fetch_banner,
    models::banners::{Banner, Banners},
};

use crate::kurogame::api::fetch_notice;

#[component]
pub fn BannersComponent() -> impl IntoView {
    let (banners, set_banners) = create_signal(None::<Banners>);

    spawn_local(async move {
        let future_banner = fetch_banner().await;

        console_log(&format!("{:?}", future_banner));

        if let Ok(mut b) = future_banner {
            let notice = fetch_notice().await;

            if let Ok(n) = notice {
                b.data.groups.iter_mut().for_each(|g| {
                    console_log(&format!("{:?}", g));
                    g.banners.iter_mut().for_each(|inner_b: &mut Banner| {
                        console_log("bon");
                        inner_b.fetch_main_banner(&n);
                        console_log(&format!("{:?}", inner_b));
                    })
                });
            }

            //new_c.data.initialize_signal_for_entries();
            set_banners.update(|old_b| *old_b = Some(b));
        }
    });

    view! {
        <img src={move || match banners() {
            Some(b) => match b.data.groups[0].banners[0].main_banner.clone() {
                    Some(s) => s,
                    None => "/no_more_work.png".to_string(),
                },
            None => "/no_more_work.png".to_string(),
        }}/>
    }
}
