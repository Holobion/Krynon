use dioxus::prelude::*;

#[component]
pub fn WeightSlider(
    id: String,
    name: String,
    description: String,
    emoji: String,
    weight: f64,
    onchange: EventHandler<f64>,
) -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();
    let rounded_weight = (weight * 10.0).round() / 10.0;
    let localized_name = lang().tr(&name);
    let localized_description = lang().tr(&description);

    rsx! {
        div {
            class: "bg-kr-cytoplasm border border-kr-matrix/10 p-4 kr-squarcle-sm transition-all duration-300 hover:border-kr-nucleus shadow-sm hover:kr-halo",
            div {
                class: "flex justify-between items-center",
                div {
                    class: "flex items-center gap-2",
                    span { class: "text-lg", "{emoji}" }
                    h4 { class: "font-semibold text-kr-nucleus text-sm tracking-wide font-display", "{localized_name}" }
                }
                span {
                    style: "background-color: var(--color-kr-turquoise) !important; color: white !important;",
                    class: "text-xs font-extrabold px-2.5 py-0.5 border border-kr-text-nucleus tabular-nums",
                    "W: {rounded_weight}"
                }
            }
            p { class: "text-kr-matrix text-xs mt-1 leading-relaxed", "{localized_description}" }

            div {
                class: "flex items-center gap-3 mt-3",
                span { class: "text-[10px] text-kr-matrix font-medium select-none", "{lang().t(\"Ignore\")}" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "10",
                    step: "0.5",
                    class: "w-full h-1.5 bg-kr-membrane border border-kr-matrix/10 rounded-none appearance-none cursor-pointer accent-kr-turquoise outline-none",
                    value: "{weight}",
                    oninput: move |event| {
                        if let Ok(val) = event.value().parse::<f64>() {
                            onchange.call(val);
                        }
                    }
                }
                span { class: "text-[10px] text-kr-nucleus font-medium select-none", "{lang().t(\"Critical\")}" }
            }
        }
    }
}
