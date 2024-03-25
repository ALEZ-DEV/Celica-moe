use leptos::{component, create_signal, spawn_local, view, IntoView, SignalUpdate};

use crate::huaxu::{api::fetch_banner, models::banners::Banners};

#[component]
pub fn BannersComponent() -> impl IntoView {
    let (banners, set_banners) = create_signal(None::<Banners>);

    spawn_local(async move {
        let future_banner = fetch_banner().await;

        if let Ok(mut b) = future_banner {
            b.data.groups.iter_mut().for_each(|g| {
                g.banners.iter_mut().for_each(|inner_b| {
                    inner_b.fetch_main_banner();
                })
            });
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
