use crate::error_template::{AppError, ErrorTemplate};
use crate::widgets::menu::MenuComponent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/celica-moe.css"/>

        // sets the document title
        <Title text="Celica.moe"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main class="inline-block">
                <MenuComponent/>
                <div class="pl-64 w-screen">
                    <div class="flex justify-center">
                        <Routes>
                                <Route path="" view=crate::pages::home::HomePage/>
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}
