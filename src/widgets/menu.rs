use crate::icon::heroic_icon::{HeroicIcons, HeroicIconsType, Icon};
use leptos::{component, view, CollectView, IntoView};
use rand::Rng;

struct Page {
    pub label: String,
    pub link: String,
    pub icon: HeroicIcons,
    pub icon_type: HeroicIconsType,
}

impl Page {
    pub fn new(name: &str, link: &str, icon: HeroicIcons, icon_type: HeroicIconsType) -> Self {
        Self {
            label: name.to_string(),
            link: link.to_string(),
            icon,
            icon_type,
        }
    }
}

#[component]
pub fn MenuComponent() -> impl IntoView {
    let linked_pages = [
        Page::new("Home", "/", HeroicIcons::Home, HeroicIconsType::Outline),
        Page::new(
            "Events",
            "/events",
            HeroicIcons::CalendarDays,
            HeroicIconsType::Solid,
        ),
        Page::new(
            "Your characters",
            "/characters",
            HeroicIcons::UserGroup,
            HeroicIconsType::Solid,
        ),
    ];

    let img_src = [
        //2nd anniversary custom stickers
        "/dont_drag_me_down.png",
        "/feed_your_cube.png",
        "/feeling_creative.png",
        "/fight_hard_eat_hard.png",
        "/happy.png",
        "/need_that.png",
        "/not_this.png",
        "/pat_pat.png",
        "/reliable.png",
        "/resigned.png",
        "/shy.png",
        "/winky_face.png",
        // If you want to add some stickers, put it in the /public folder and make a PR
    ];
    let mut rng = rand::thread_rng();

    view! {
        <aside class="menu bg-base-200 flex justify-start fixed top-0 left-0 z-40 w-64 h-screen">

            <div class="prose">
                <img src={*img_src.get(rng.gen_range(0..img_src.len())).unwrap()} class="size-40 mx-auto my-1 rounded-md"/>
                <h2 class="mx-auto my-0 text-center">Celica.moe</h2>
            </div>

            {linked_pages.iter().map(|p| {
                view! {
                    <li>
                        <a href={p.link.clone()}>
                            <Icon icon=p.icon icon_type=p.icon_type/>
                            {p.label.clone()}
                        </a>
                    </li>
                }
            }).collect_view()}
        </aside>
    }
}
