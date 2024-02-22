use leptos::*;
use stylers::*;

#[derive(Clone)]
pub struct CardSwitcherSection {
    pub title: String,
    pub view: View,
}

#[component]
pub fn CardSwitcher(
    #[prop(into)] sections: MaybeSignal<Vec<CardSwitcherSection>>,
) -> impl IntoView {
    let styles = style_sheet!("./src/components/card_switcher.scss");

    let (selected_idx, set_selected_idx) = create_signal(0usize);

    let mapped_sections = create_memo(move |_| {
        let sections = sections.get();
        let section_count = sections.len();

        sections.into_iter().fold(
            (
                Vec::with_capacity(section_count),
                Vec::with_capacity(section_count),
            ),
            |mut acc, i| {
                acc.0.push(i.title);
                acc.1.push(i.view);
                acc
            },
        )
    });

    let titles = move || {
        mapped_sections()
            .0
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                view! {class = styles,
                    <li on:click=move |_| set_selected_idx.set(idx)
                        class="switcher-title pointable"
                        class:selected=move || selected_idx() == idx
                    >
                        {item}
                    </li>
                }
            })
            .collect_view()
    };

    let views = move || mapped_sections().1;

    view! {class = styles,
        <div class="switcher-container">
            <ul class="switcher-titles">
                {titles}
            </ul>
           <div class="switcher-cards">
                <For    each = move || views().into_iter().enumerate()
                        key = move |item| item.0
                        children = move |(idx, view)| {
                            view! {class = styles,
                                <div    class="switcher-card"
                                        class:selected=move || idx == selected_idx()>
                                    {view}
                                </div>
                            }
                        }
                />
           </div>
        </div>
    }
}
