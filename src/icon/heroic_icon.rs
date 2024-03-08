use leptos::{component, IntoView, view};
use crate::icon::heroic_micro_icon::HeroicMicroIcon;
use crate::icon::heroic_mini_icon::HeroicMiniIcon;
use crate::icon::heroic_outline_icon::HeroicOutlineIcon;
use crate::icon::heroic_solid_icon::HeroicSolidIcon;

pub enum HeroicIconsType {
    Outline,
    Solid,
    Mini,
    Micro,
}

pub enum HeroicIcons {
    Home,
}

#[component]
pub fn Icon(icon: HeroicIcons, icon_type: HeroicIconsType) -> impl IntoView {
    match icon_type {
        HeroicIconsType::Outline => view! { <HeroicOutlineIcon icon={icon}/> },
        HeroicIconsType::Solid => view! { <HeroicSolidIcon icon={icon}/> },
        HeroicIconsType::Mini => view! { <HeroicMiniIcon icon={icon}/> },
        HeroicIconsType::Micro => view! { <HeroicMicroIcon icon={icon}/> },
    }
}





