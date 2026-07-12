use crate::model::{calculate_score, Criterion, Product};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn ProductCard(
    rank: usize,
    product: Product,
    criteria: Vec<Criterion>,
    weights: HashMap<String, f64>,
) -> Element {
    let mut is_expanded = use_signal(|| false);
    let lang = use_context::<Signal<crate::i18n::Language>>();

    // Calculate score and contribution breakdown based on active weights
    let overall_score = calculate_score(&product, &weights);
    let rounded_score = (overall_score * 10.0).round() / 10.0;

    let formatted_price = format!("${:.2}", product.price);
    let unit_price_str = if let (Some(qty), Some(unit)) = (product.quantity, product.unit.as_ref())
    {
        if qty > 0.0 && !unit.is_empty() {
            Some(format!("(${:.2} / {})", product.price / qty, unit))
        } else {
            None
        }
    } else {
        None
    };

    let sum_weights: f64 = criteria
        .iter()
        .map(|c| weights.get(&c.id).copied().unwrap_or(5.0))
        .sum();

    // Ranks visual config
    let (card_border, rank_bg, _rank_text_color) = match rank {
        1 => (
            "border-amber-200/60",
            "bg-amber-100 text-amber-700 font-black border border-amber-200/50",
            "text-amber-700",
        ),
        2 => (
            "border-slate-200/60",
            "bg-slate-100 text-slate-600 font-black border border-slate-200/50",
            "text-slate-600",
        ),
        3 => (
            "border-orange-200/60",
            "bg-orange-50 text-orange-700 font-bold border border-orange-200/50",
            "text-orange-700",
        ),
        _ => (
            "border-kr-matrix/10 hover:border-kr-matrix/20",
            "bg-kr-membrane text-kr-matrix border border-kr-matrix/10 font-semibold",
            "text-kr-matrix",
        ),
    };

    let arrow_class = if is_expanded() { "rotate-180" } else { "" };
    
    let localized_product_name = lang().tr(&product.name);
    let localized_product_description = lang().tr(&product.description);

    rsx! {
        div {
            class: "bg-kr-cytoplasm border {card_border} p-5 kr-squarcle transition-all duration-300 hover:scale-[1.005] hover:shadow-md hover:border-kr-turquoise cursor-pointer relative overflow-hidden group shadow-sm hover:kr-halo",
            onclick: move |_| {
                is_expanded.set(!is_expanded());
            },

            // Inner card header
            div {
                class: "flex justify-between items-start gap-4",
                div {
                    class: "flex items-start gap-3",
                    // Rank badge
                    span {
                        class: "flex items-center justify-center w-8 h-8 rounded-none border border-kr-nucleus shrink-0 text-sm tracking-tighter {rank_bg}",
                        "#{rank}"
                    }
                    div {
                        h3 { class: "font-bold text-kr-nucleus text-lg group-hover:text-kr-turquoise transition-colors font-display", "{localized_product_name}" }
                        div { class: "flex flex-wrap items-center gap-2 mt-1",
                            span {
                                class: "px-2 py-0.5 rounded-none text-xs font-extrabold bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 shrink-0",
                                "{formatted_price}"
                            }
                            if let Some(ref unit_price) = unit_price_str {
                                span {
                                    class: "text-[10px] font-bold text-kr-matrix bg-kr-membrane px-1.5 py-0.5 rounded-none border border-kr-matrix/10 shrink-0",
                                    "{unit_price}"
                                }
                            }
                            span { class: "text-kr-matrix/30 text-xs shrink-0", "|" }
                            p { class: "text-kr-matrix text-xs line-clamp-1", "{localized_product_description}" }
                        }
                    }
                }

                // Score Badge
                div {
                    class: "flex flex-col items-end",
                    span {
                        style: "background-color: var(--color-kr-turquoise) !important; color: white !important;",
                        class: "font-extrabold text-lg px-3.5 py-1 border border-kr-text-nucleus tabular-nums shadow-sm",
                        "{rounded_score}"
                    }
                    span { class: "text-[10px] text-kr-matrix uppercase tracking-widest mt-1", "{lang().t(\"Score\")}" }
                }
            }

            // Small mini progress bars to show weight proportion visually
            div {
                class: "flex w-full h-1.5 bg-kr-membrane rounded-none mt-4 overflow-hidden border border-kr-matrix/10",
                for criterion in &criteria {
                    {
                        let weight = weights.get(&criterion.id).copied().unwrap_or(5.0);
                        let score = product.scores.get(&criterion.id).copied().unwrap_or(5.0);
                        let contrib = score * weight;
                        let percentage = if sum_weights > 0.0 {
                            (contrib / (sum_weights * 10.0)) * 100.0
                        } else {
                            0.0
                        };

                        // Color per criterion type
                        let color = match criterion.id.as_str() {
                            "carbon_footprint" => "bg-teal-500",
                            "sourcing_ethics" => "bg-emerald-500",
                            "e_waste" => "bg-purple-500",
                            "durability" => "bg-sky-500",
                            "camera" | "aroma" | "texture" => "bg-indigo-400",
                            "battery" | "acidity" | "fragrance" => "bg-amber-500",
                            "reparability" | "body" | "ergonomics" => "bg-rose-500",
                            "performance" | "sweetness" | "adjustability" => "bg-sky-400",
                            _ => "bg-slate-400",
                        };

                        let localized_crit_name = lang().tr(&criterion.name);

                        rsx! {
                            div {
                                key: "{criterion.id}",
                                class: "h-full {color} transition-all duration-300",
                                style: "width: {percentage}%",
                                title: "{localized_crit_name}: score {score} x weight {weight}"
                            }
                        }
                    }
                }
            }

            // Expandable details drawer
            if is_expanded() {
                div {
                    class: "mt-5 pt-4 border-t border-kr-matrix/10 space-y-4 animate-fade-in-down",
                    onclick: move |e| {
                        // Prevent clicking detail drawer from closing card
                        e.stop_propagation();
                    },

                    div {
                        class: "flex items-center gap-3 bg-kr-membrane p-3 kr-squarcle-sm text-xs font-medium text-kr-nucleus bg-dot-pattern",
                        span { class: "text-kr-matrix", "{lang().t(\"Market Pricing\")}" }
                        span { class: "font-extrabold text-emerald-600 text-sm", "{formatted_price}" }
                        if let (Some(qty), Some(unit)) = (product.quantity, product.unit.as_ref()) {
                            if qty > 0.0 && !unit.is_empty() {
                                {
                                    let unit_price = product.price / qty;
                                    rsx! {
                                        span { class: "text-kr-matrix",
                                            "({qty} {unit} @ ${unit_price:.2} / {unit})"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    p { class: "text-kr-nucleus/90 text-sm leading-relaxed", "{localized_product_description}" }

                    h4 { class: "text-xs font-bold text-kr-matrix uppercase tracking-widest mt-4 font-display", "{lang().t(\"Detailed Criteria Breakdown\")}" }

                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 gap-4 mt-2",
                        for criterion in &criteria {
                            {
                                let score = product.scores.get(&criterion.id).copied().unwrap_or(5.0);
                                let weight = weights.get(&criterion.id).copied().unwrap_or(5.0);
                                let contribution = if sum_weights > 0.0 {
                                    (score * weight) / sum_weights
                                } else {
                                    0.0
                                };
                                let contrib_rounded = (contribution * 10.0).round() / 10.0;
                                let score_percentage = score * 10.0;

                                let color = match criterion.id.as_str() {
                                    "carbon_footprint" => "bg-teal-500",
                                    "sourcing_ethics" => "bg-emerald-500",
                                    "e_waste" => "bg-purple-500",
                                    "durability" => "bg-sky-500",
                                    "camera" | "aroma" | "texture" => "bg-indigo-400",
                                    "battery" | "acidity" | "fragrance" => "bg-amber-500",
                                    "reparability" | "body" | "ergonomics" => "bg-rose-500",
                                    "performance" | "sweetness" | "adjustability" => "bg-sky-400",
                                    _ => "bg-slate-400",
                                };

                                let localized_crit_name = lang().tr(&criterion.name);

                                rsx! {
                                    div {
                                        key: "{criterion.id}",
                                        class: "bg-kr-membrane border border-kr-matrix/10 p-3 kr-squarcle-sm flex flex-col justify-between shadow-xs",
                                        div {
                                            class: "flex justify-between items-center",
                                            div {
                                                class: "flex items-center gap-1.5",
                                                span { class: "text-sm", "{criterion.emoji}" }
                                                span { class: "text-xs font-semibold text-kr-nucleus", "{localized_crit_name}" }
                                            }
                                            span { class: "text-xs font-bold text-kr-nucleus tabular-nums", "{score} / 10" }
                                        }

                                        // Score Bar
                                        div {
                                            class: "w-full h-1.5 bg-kr-cytoplasm border border-kr-matrix/10 rounded-none mt-2 overflow-hidden",
                                            div {
                                                class: "h-full {color}",
                                                style: "width: {score_percentage}%"
                                            }
                                        }

                                        // Weight and Contribution Info
                                        {
                                            let weight_label = lang().t("Weight factor: x");
                                            let adds_label = lang().t("Adds +");
                                            let score_label = lang().t(" to score");
                                            rsx! {
                                                div {
                                                    class: "flex justify-between items-center text-[10px] text-kr-matrix mt-2 font-medium",
                                                    span { "{weight_label}{weight}" }
                                                    span { class: "text-kr-turquoise font-bold", "{adds_label}{contrib_rounded}{score_label}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Dropdown Toggle hint at bottom
            div {
                class: "flex justify-center mt-3",
                svg {
                    class: "w-4 h-4 text-kr-matrix group-hover:text-kr-turquoise transition-all duration-300 {arrow_class}",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M19 9l-7 7-7-7"
                    }
                }
            }
        }
    }
}
