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
    let rounded_weight = (weight * 10.0).round() / 10.0;

    rsx! {
        div {
            class: "bg-hb-cytoplasm border border-hb-matrix/10 p-4 hb-squarcle-sm transition-all duration-300 hover:border-hb-primary/30 shadow-sm hover:hb-halo",
            div {
                class: "flex justify-between items-center",
                div {
                    class: "flex items-center gap-2",
                    span { class: "text-lg", "{emoji}" }
                    h4 { class: "font-semibold text-hb-nucleus text-sm tracking-wide font-display", "{name}" }
                }
                span {
                    class: "bg-hb-primary/10 text-hb-primary text-xs font-semibold px-2.5 py-1 rounded-full border border-hb-primary/20 tabular-nums",
                    "W: {rounded_weight}"
                }
            }
            p { class: "text-hb-matrix text-xs mt-1 leading-relaxed", "{description}" }
            
            div {
                class: "flex items-center gap-3 mt-3",
                span { class: "text-[10px] text-hb-matrix font-medium select-none", "Ignore" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "10",
                    step: "0.5",
                    class: "w-full h-1.5 bg-hb-membrane border border-hb-matrix/10 rounded-full appearance-none cursor-pointer accent-hb-primary outline-none focus:ring-1 focus:ring-hb-primary/30",
                    value: "{weight}",
                    oninput: move |event| {
                        if let Ok(val) = event.value().parse::<f64>() {
                            onchange.call(val);
                        }
                    }
                }
                span { class: "text-[10px] text-hb-primary font-medium select-none", "Critical" }
            }
        }
    }
}
