use crate::icon::heroic_micro_icon::HeroicMicroIcon;
use crate::icon::heroic_mini_icon::HeroicMiniIcon;
use crate::icon::heroic_outline_icon::HeroicOutlineIcon;
use crate::icon::heroic_solid_icon::HeroicSolidIcon;
use leptos::{component, view, IntoView};

#[derive(Clone, Copy)]
pub enum HeroicIconsType {
    Outline,
    Solid,
    Mini,
    Micro,
}

#[derive(Clone, Copy)]
pub enum HeroicIcons {
    Home,
    Calendar,
    CalendarDays,
    Users,
    UserGroup,
    Bars3,
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
