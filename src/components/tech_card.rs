use leptos::*;

use crate::model::tech_item::TechItem;

#[component]
pub fn TechCard(#[prop(into)] tech_item: MaybeSignal<TechItem>) -> impl IntoView {
    let tech_item = tech_item.get();

    let related_chips = tech_item
        .related_tech
        .iter()
        .cloned()
        .map(|item| {
            view! {
                <div class="related-chip">
                    {item.name}
                </div>
            }
        })
        .collect_view();

    view! {
        <div class="card-body">
            <h3 class="card-title">{tech_item.name}</h3>
            <p class="card-description">{tech_item.desc}</p>
            {related_chips}
        </div>
    }
}
