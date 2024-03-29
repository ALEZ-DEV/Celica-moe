use crate::icon::heroic_icon::{HeroicIcons, HeroicIconsType, Icon};
use leptos::*;
use rand::Rng;

#[derive(Clone)]
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
pub fn MenuComponent(main_content: View) -> impl IntoView {
    view! {
        <div class="drawer lg:drawer-open">
          <input id="responsive-drawer" type="checkbox" class="drawer-toggle" />
          <div class="drawer-content flex flex-col items-center justify-center">
            <MobileNavbarComponent/>
            <div class="h-full">
                {main_content}
            </div>
          </div>
          <div class="drawer-side">
            <label for="responsive-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
            <MenuContentComponent/>
          </div>
        </div>
    }
}

#[component]
fn MobileNavbarComponent() -> impl IntoView {
    view! {
        <div class="h-20 w-full bg-base-200 p-2 lg:hidden flex justify-between">
            <label for="responsive-drawer" class="btn btn-outline drawer-button">
                <Icon icon=HeroicIcons::Bars3 icon_type=HeroicIconsType::Solid/>
            </label>

            <a class="flex justify-end" href="/">
                <h4 class="h-auto my-auto text-center">Celica.moe</h4>
                <img src="/celica_pointing.png" class="size-12 mx-auto mb-1"/>
            </a>
        </div>
    }
}

#[component]
fn MenuContentComponent() -> impl IntoView {
    let linked_pages = vec![
        Page::new("Home", "/", HeroicIcons::Home, HeroicIconsType::Outline),
        Page::new(
            "Calendar",
            "/calendar",
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
    let random_img = *img_src.get(rng.gen_range(0..img_src.len())).unwrap();

    view! {
        <aside class="menu bg-base-200 flex justify-start fixed top-0 left-0 z-40 w-64 h-screen">

            <div class="prose">
                <img src={random_img} class="size-40 mx-auto my-1 rounded-md"/>
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

            <div class="grid flex flex-grow content-end inline">
                <a href="https://github.com/ALEZ-DEV/Celica-moe" target="_blank">
                    <div class="bg-base-100 p-1.5 rounded-md">
                        <div class="grid grid-rows-2 grid-flow-col">
                            // github logo
                            <svg height="48" aria-hidden="true" viewBox="0 0 16 16" version="1.1" width="48" class="text-content-100 fill-current row-span-2">
                                <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
                            </svg>
                            <h5 class="m-0">{format!("V{}", env!("CARGO_PKG_VERSION"))}</h5>
                            <h6 class="m-0">View on github</h6>
                        </div>
                    </div>
                </a>
            </div>
        </aside>
    }
}
