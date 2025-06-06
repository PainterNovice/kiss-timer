use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options islands=true />
                <link rel="stylesheet" id="leptos" href="/pkg/islands.css" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="KISS Timer" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| {
                view! { <NotFound /> }
            }>
                <Route path=path!("kiss-timer/") view=Home />
            </Routes>
        </Router>
    }
}
