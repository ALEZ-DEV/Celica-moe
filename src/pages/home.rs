use leptos::{component, view, IntoView, CollectView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <div class="block">
                <img src="/celica_pointing.png" class="size-56 mx-auto mb-1"/>
                <h1 class="mx-auto text-center">Welcom to celica.moe!</h1>
            </div>
        </div>
    }
}
