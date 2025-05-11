use leptos::prelude::*;
use leptos_router::hooks::use_location;
use std::time::Duration;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let location = use_location();

    // Countdown timer logic
    let initial_time = move || {
        let time_str = location.search.get();
        parse_duration_from_url(&time_str).unwrap_or(1500) as isize
    };

    let (remaining, set_remaining) = signal(initial_time() as isize);
    let (is_running, _set_is_running) = signal(true);

    let is_overtime = Memo::new(move |_| remaining.get() < 0);

    Effect::new(move |_| {
        if is_running.get() {
            let handle = set_interval_with_handle(
                move || set_remaining.update(|t| *t -= 1),
                Duration::from_secs(1),
            )
            .unwrap();

            on_cleanup(move || handle.clear());
        }
    });

    // Timer formatting
    let formatted_time = move || {
        let total_seconds = remaining.get();
        let hours = (total_seconds / 3600).abs();
        let remainder_after_hh = total_seconds % 3600;
        let minutes = remainder_after_hh / 60;
        let seconds = remainder_after_hh % 60; // Use abs() for seconds to ensure they are positive

        match hours {
            0 => format!("{:02}:{:02}", minutes.abs(), seconds.abs()),
            _ => format!(
                "{:02}:{:02}:{:02}",
                hours.abs(),
                minutes.abs(),
                seconds.abs()
            ),
        }
    };

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
                <div class="timer">
                    <div class="time-display">
                        <h1 class:overtime=is_overtime>{formatted_time}</h1>
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}

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
        _ => None,
    }
}
