use leptos::{component, view, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>Welcom to celica.moe!</h1>
        <img src="/celica_pointing.jpeg"/>
    }
}
