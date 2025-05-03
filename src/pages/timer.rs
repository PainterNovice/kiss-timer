/*
    Timer
    /1H1M1S
*/

use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_router::params::Params;
use std::time::Duration;
use gloo_timers::callback::Interval;


#[component]
pub fn Timer() -> impl IntoView {
    let location = use_location();
    
    // Countdown timer logic
    let initial_time = move || {
        let time_str = location.search.get();
        parse_duration_from_url(&time_str).unwrap_or(1500)
    };

    let (remaining, set_remaining) = create_signal(initial_time());
    let (is_running, set_is_running) = create_signal(true);

    Effect::new(move |_| {
        if is_running.get() && remaining.get() > 0 {
            let handle = set_interval_with_handle(
                move || set_remaining.update(|t| *t -= 1),
                Duration::from_secs(1)
            ).unwrap();
            
            on_cleanup(move || handle.clear());
        }
    });

    // Timer formatting
    let formatted_time = move || {
        let minutes = remaining.get() / 60;
        let seconds = remaining.get() % 60;
        format!("{:02}:{:02}", minutes, seconds)
    };

    view!{
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors.get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="container">
                <div class="timer">
                    <div class="time-display"><h1>{formatted_time}</h1></div>
                </div>
            </div>
        </ErrorBoundary>
    }
}

// Add this helper function outside the component
fn parse_duration_from_url(time_str: &str) -> Option<usize> {
    let parts: Vec<&str> = time_str.split(':').collect();
    match parts.len() {
        1 => time_str.parse().ok(), // seconds only
        2 => {
            let minutes = parts[0].parse::<usize>().ok()?;
            let seconds = parts[1].parse::<usize>().ok()?;
            Some(minutes * 60 + seconds)
        }
        3 => {
            let hours = parts[0].parse::<usize>().ok()?;
            let minutes = parts[1].parse::<usize>().ok()?;
            let seconds = parts[2].parse::<usize>().ok()?;
            Some(hours * 3600 + minutes * 60 + seconds)
        }
        _ => None
    }
}