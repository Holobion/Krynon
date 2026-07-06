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

    // Calculate score and contribution breakdown based on active weights
    let overall_score = calculate_score(&product, &weights);
    let rounded_score = (overall_score * 10.0).round() / 10.0;

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
            "border-hb-matrix/10 hover:border-hb-matrix/20",
            "bg-hb-membrane text-hb-matrix border border-hb-matrix/10 font-semibold",
            "text-hb-matrix",
        ),
    };

    let arrow_class = if is_expanded() { "rotate-180" } else { "" };

    rsx! {
        div {
            class: "bg-hb-cytoplasm border {card_border} p-5 hb-squarcle transition-all duration-300 hover:scale-[1.005] hover:shadow-md hover:border-hb-primary/30 cursor-pointer relative overflow-hidden group shadow-sm hover:hb-halo",
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
                        class: "flex items-center justify-center w-8 h-8 rounded-lg shrink-0 text-sm tracking-tighter {rank_bg}",
                        "#{rank}"
                    }
                    div {
                        h3 { class: "font-bold text-hb-nucleus text-lg group-hover:text-hb-primary transition-colors font-display", "{product.name}" }
                        p { class: "text-hb-matrix text-xs mt-0.5 line-clamp-1", "{product.description}" }
                    }
                }

                // Score Badge
                div {
                    class: "flex flex-col items-end",
                    span {
                        class: "bg-hb-primary text-white font-extrabold text-lg px-3.5 py-1 rounded-xl border border-hb-primary/20 tabular-nums shadow-sm hb-halo",
                        "{rounded_score}"
                    }
                    span { class: "text-[10px] text-hb-matrix uppercase tracking-widest mt-1", "Score" }
                }
            }

            // Small mini progress bars to show weight proportion visually
            div {
                class: "flex w-full h-1.5 bg-hb-membrane rounded-full mt-4 overflow-hidden border border-hb-matrix/10",
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

                        rsx! {
                            div {
                                key: "{criterion.id}",
                                class: "h-full {color} transition-all duration-300",
                                style: "width: {percentage}%",
                                title: "{criterion.name}: score {score} x weight {weight}"
                            }
                        }
                    }
                }
            }

            // Expandable details drawer
            if is_expanded() {
                div {
                    class: "mt-5 pt-4 border-t border-hb-matrix/10 space-y-4 animate-fade-in-down",
                    onclick: move |e| {
                        // Prevent clicking detail drawer from closing card
                        e.stop_propagation();
                    },

                    p { class: "text-hb-nucleus/90 text-sm leading-relaxed", "{product.description}" }

                    h4 { class: "text-xs font-bold text-hb-matrix uppercase tracking-widest mt-4 font-display", "Detailed Criteria Breakdown" }

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

                                rsx! {
                                    div {
                                        key: "{criterion.id}",
                                        class: "bg-hb-membrane border border-hb-matrix/10 p-3 hb-squarcle-sm flex flex-col justify-between shadow-xs",
                                        div {
                                            class: "flex justify-between items-center",
                                            div {
                                                class: "flex items-center gap-1.5",
                                                span { class: "text-sm", "{criterion.emoji}" }
                                                span { class: "text-xs font-semibold text-hb-nucleus", "{criterion.name}" }
                                            }
                                            span { class: "text-xs font-bold text-hb-nucleus tabular-nums", "{score} / 10" }
                                        }

                                        // Score Bar
                                        div {
                                            class: "w-full h-1.5 bg-hb-cytoplasm border border-hb-matrix/10 rounded-full mt-2 overflow-hidden",
                                            div {
                                                class: "h-full {color}",
                                                style: "width: {score_percentage}%"
                                            }
                                        }

                                        // Weight and Contribution Info
                                        div {
                                            class: "flex justify-between items-center text-[10px] text-hb-matrix mt-2 font-medium",
                                            span { "Weight factor: x{weight}" }
                                            span { class: "text-hb-primary font-bold", "Adds +{contrib_rounded} to score" }
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
                    class: "w-4 h-4 text-hb-matrix group-hover:text-hb-primary transition-all duration-300 {arrow_class}",
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
