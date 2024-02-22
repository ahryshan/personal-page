use leptos::*;
use stylers::*;

use crate::model::job::Job;

#[component]
pub fn JobCard(job: Job) -> impl IntoView {
    let styles = style_sheet!("./src/components/job_card.scss");

    let company_frag = match job.company_link {
        Some(link) => view! { class = styles,
            <a class="company-name" href={ link } target="_blank">{ job.company_name }</a>
        }
        .into_any(),
        None => view! { class = styles,
            <span class="company-name">{ job.company_name }</span>
        }
        .into_any(),
    };

    let date_str = {
        let start_date_repr = job.start_date.format("%b %Y").to_string();
        let end_date_repr = match job.end_date {
            Some(date) => date.format("%b %Y").to_string(),
            None => "Present".into(),
        };

        let start_date = job.start_date;
        let end_date = job.end_date.unwrap_or(chrono::Local::now().date_naive());

        let years = (end_date - start_date).num_weeks() as f32 / 52.0;
        let months = (years.fract() * 12.0).floor() as i32;
        let years = years.floor() as i32;

        let years_str = match years {
            0 => "".into(),
            1 => "1 year".into(),
            _ => format!("{} years", years),
        };

        let months_str = match months {
            0 => "".into(),
            1 => "1 month".into(),
            _ => format!("{} months", months),
        };

        let duration_repr = match (years, months) {
            (0, 0) => "".into(),
            (0, _) => format!(" ({})", months_str),
            (_, 0) => format!(" ({})", years_str),
            (_, _) => format!(" ({} and {})", years_str, months_str),
        };

        format!("{} - {}{}", start_date_repr, end_date_repr, duration_repr)
    };

    view! { class = styles,
        <div class="card">
            <div class="card-body">
                <h3 class="title">
                    { job.title } " - " { company_frag }
                </h3>
                <p class="dates">
                    { date_str }
                </p>
                <p class="location">
                    { job.location }
                </p>
                <p class="description">
                    { job.description }
                </p>
            </div>
        </div>
    }
}
