use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    // let path = leptos_router::location::page();
    let location = use_location();

    view! {
        <h1>"Uh oh!" <br /> "We couldn't find that page!"</h1>

        <div class="current-url">
            <h2>{move || location.pathname.get()}</h2>

            <h3>"Usage:"</h3>
            <div class="usage-examples">
                "/?90 (90 seconds)"<br /> "/?1:30 (1 minute 30 seconds)"<br />
                "/?1:05:30 (1 hour, 5 minutes, 30 seconds)"
            </div>

        </div>
    }
}
