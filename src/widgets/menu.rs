use crate::icon::heroic_icon::{HeroicIcons, HeroicIconsType, Icon};
use leptos::{component, view, CollectView, IntoView};

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
        Page::new("Home", "", HeroicIcons::Home, HeroicIconsType::Outline),
        Page::new(
            "Events",
            "",
            HeroicIcons::CalendarDays,
            HeroicIconsType::Solid,
        ),
        Page::new(
            "Your characters",
            "",
            HeroicIcons::UserGroup,
            HeroicIconsType::Solid,
        ),
    ];

    view! {
        <aside class="menu bg-base-200 flex justify-start fixed top-0 left-0 z-40 w-64 h-screen">
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
