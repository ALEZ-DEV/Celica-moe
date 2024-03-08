use leptos::{CollectView, component, IntoView, view};
use crate::icon::heroic_icon::{HeroicIcons, HeroicIconsType, Icon};

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
        Page::new("Home", "", HeroicIcons::Home, HeroicIconsType::Solid),
        Page::new("Ho", "", HeroicIcons::Home, HeroicIconsType::Mini),
        Page::new("Ho", "", HeroicIcons::Home, HeroicIconsType::Micro),
    ];
    
    view! {
        <ul class="menu bg-base-200 w-56 flex justify-start h-96">
            {linked_pages.iter().map(|p| {
                view! {
                    <li>
                        <a href={p.link.clone()}>
                            <Icon icon=HeroicIcons::Home icon_type=HeroicIconsType::Outline/>
                            {p.label.clone()}
                        </a>
                    </li>
                }
            }).collect_view()}
        </ul>
    }
}