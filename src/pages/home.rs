use crate::components::counter_btn::Button;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <h1>"KISS Timer"</h1>
                <br />
                <div class="quick-links">
                    <a href="/kiss-timer/timer/?5:00" class="duration-link">
                        "5 Minutes"
                    </a>
                    <br />
                    <a href="/kiss-timer/timer/?15:00" class="duration-link">
                        "15 Minutes"
                    </a>
                    <br />
                    <a href="/kiss-timer/timer/?25:00" class="duration-link">
                        "25 Minutes"
                    </a>
                </div>

            </div>
        </ErrorBoundary>
    }
}
