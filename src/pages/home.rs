use leptos::{component, view, IntoView, CollectView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>Welcom to celica.moe!</h1>

        {(0..15).map(|_| view! {
            <div class="content-end">
                <img src="/celica_pointing.png"/>
            </div>
        }).collect_view()}
    }
}
