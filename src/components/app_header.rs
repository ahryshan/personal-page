use leptos::{html::Div, *};
use stylers::*;

#[component]
pub fn TitleHeader() -> impl IntoView {
    let styles = style_sheet!("./src/components/app_header.scss");
    let header_ref = create_node_ref::<Div>();

    let (header_offset, set_header_offset) = create_signal(0.0);

    let handle = window_event_listener(ev::scroll, move |event: ev::Event| {
        let header = header_ref.get().unwrap();
        let scroll = window().scroll_y().unwrap();
        let header_height = header.client_height();

        if scroll > header_height as f64 {
            return;
        }

        let header_width = header.client_width();
        let move_factor = header_width as f64 / header_height as f64;
        let offset = scroll * move_factor;

        set_header_offset.set(offset);
    });

    view! { class = styles,
                        <div class="header-wrapper" style={move || format!("--offset: -{}px", header_offset.get())} _ref=header_ref>
                <div class="header-content">
                    <h1 class="name">"Alex Hryshan"</h1>
                    <h2 class="job-title">"\"Software Engineer\""</h2>
                    <div class="links">
                        <a href="https://github.com/nightingazer" target="_blank">Github</a>
                        <a href="https://linkedin.com/in/alex-hryshan" target="_blank">Linkedin</a>
                    </div>
                </div>
            </div>
    }
}
