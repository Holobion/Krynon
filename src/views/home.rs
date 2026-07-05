use crate::components::Hero;
use crate::model::{
    get_mock_categories, get_mock_product_types, get_mock_products,
    calculate_score, get_combined_criteria, Product
};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let categories = get_mock_categories();
    let product_types = get_mock_product_types();
    let products = get_mock_products();

    let smartphone_type = product_types.iter().find(|p| p.id == "smartphones").unwrap().clone();
    let smartphone_criteria = get_combined_criteria(&smartphone_type, &categories);
    let smartphone_products: Vec<Product> = products.iter()
        .filter(|p| p.product_type_id == "smartphones")
        .cloned()
        .collect();
    
    // Manage active preset index for the mini-demo
    let mut selected_preset_idx = use_signal(|| 1); // Default to Eco Advocate

    let current_preset = &smartphone_type.presets[selected_preset_idx()];
    
    // Sort products based on active preset weights
    let mut sorted_products = smartphone_products.clone();
    sorted_products.sort_by(|a, b| {
        let score_a = calculate_score(a, &current_preset.weights);
        let score_b = calculate_score(b, &current_preset.weights);
        score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    rsx! {
        div {
            class: "pb-20",
            
            // Hero Section
            Hero {}

            // Interactive Mini-Demo Section
            div {
                class: "max-w-6xl mx-auto px-6 py-12 border-t border-hb-matrix/10",
                div {
                    class: "text-center max-w-2xl mx-auto mb-10",
                    h2 { class: "text-3xl font-bold tracking-tight text-hb-nucleus sm:text-4xl font-display", "See It In Action" }
                    p { class: "mt-3 text-hb-matrix text-sm sm:text-base", "Select a consumer profile below to see how the smartphone ranking dynamically recalculates based on different priorities." }
                }

                // Preset selector buttons
                div {
                    class: "flex flex-wrap justify-center gap-3 mb-8",
                    for (idx, preset) in smartphone_type.presets.iter().enumerate() {
                        {
                            let is_selected = selected_preset_idx() == idx;
                            let btn_class = if is_selected {
                                "bg-hb-primary border-hb-primary text-white shadow-lg hb-halo"
                            } else {
                                "bg-hb-cytoplasm border-hb-matrix/20 text-hb-matrix hover:border-hb-primary hover:text-hb-nucleus"
                            };
                            rsx! {
                                button {
                                    key: "{idx}",
                                    class: "px-5 py-2.5 rounded-full text-xs sm:text-sm font-bold border transition-all duration-200 {btn_class}",
                                    onclick: move |_| {
                                        selected_preset_idx.set(idx);
                                    },
                                    "{preset.name}"
                                }
                            }
                        }
                    }
                }

                // Split demo block
                div {
                    class: "grid grid-cols-1 lg:grid-cols-12 gap-8 bg-hb-cytoplasm border border-hb-matrix/10 hb-squarcle p-6 sm:p-8 shadow-sm",
                    
                    // Left Column: Show the weights for this profile
                    div {
                        class: "lg:col-span-5 flex flex-col justify-between space-y-6",
                        div {
                            h3 { class: "text-lg font-bold text-hb-nucleus font-display", "Profile Configuration" }
                            p { class: "text-xs text-hb-matrix mt-1 leading-relaxed", "The relative weights assigned to each product attribute for this profile (on a scale of 0 to 10):" }
                        }
                        
                        div {
                            class: "space-y-4",
                            for criterion in &smartphone_criteria {
                                {
                                    let weight = current_preset.weights.get(&criterion.id).copied().unwrap_or(5.0);
                                    let percentage = weight * 10.0;
                                    rsx! {
                                        div {
                                            key: "{criterion.id}",
                                            class: "space-y-1.5",
                                            div {
                                                class: "flex justify-between items-center text-xs",
                                                div {
                                                    class: "flex items-center gap-1.5 font-medium text-hb-nucleus",
                                                    span { "{criterion.emoji}" }
                                                    span { "{criterion.name}" }
                                                }
                                                span { class: "font-bold text-hb-primary tabular-nums", "{weight}" }
                                            }
                                            div {
                                                class: "h-2 w-full bg-hb-membrane rounded-full overflow-hidden border border-hb-matrix/10",
                                                div {
                                                    class: "h-full bg-hb-primary rounded-full transition-all duration-500",
                                                    style: "width: {percentage}%"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div {
                            class: "text-xs text-hb-matrix border-t border-hb-matrix/10 pt-4",
                            span { "🧮 Score Formula: " }
                            code { class: "text-hb-primary bg-hb-membrane px-1.5 py-0.5 rounded font-mono hb-border-light", "sum(score * weight) / sum(weights)" }
                        }
                    }

                    // Right Column: Show the sorted products
                    div {
                        class: "lg:col-span-7 space-y-3",
                        h3 { class: "text-lg font-bold text-hb-nucleus mb-4 font-display", "Resulting Rank Order" }
                        
                        for (rank_idx, product) in sorted_products.iter().take(3).enumerate() {
                            {
                                let rank = rank_idx + 1;
                                let score = calculate_score(product, &current_preset.weights);
                                let score_rounded = (score * 10.0).round() / 10.0;

                                let (rank_bg, text_highlight) = match rank {
                                    1 => ("bg-amber-100 text-amber-700 border-amber-200/50", "text-hb-nucleus font-bold"),
                                    2 => ("bg-slate-100 text-slate-600 border-slate-200/50", "text-hb-nucleus font-bold"),
                                    3 => ("bg-orange-50 text-orange-700 border-orange-200/50", "text-hb-nucleus"),
                                    _ => ("bg-hb-membrane text-hb-matrix border-hb-matrix/10", "text-hb-matrix"),
                                };

                                rsx! {
                                    div {
                                        key: "{product.id}",
                                        class: "flex items-center justify-between p-4 hb-squarcle-sm bg-hb-membrane border border-hb-matrix/10 transition-all duration-300 hover:border-hb-primary/30 hover:bg-hb-cytoplasm shadow-sm",
                                        div {
                                            class: "flex items-center gap-3",
                                            span {
                                                class: "flex items-center justify-center w-8 h-8 rounded-lg text-sm border font-black {rank_bg}",
                                                "#{rank}"
                                            }
                                            div {
                                                span { class: "text-sm {text_highlight}", "{product.name}" }
                                                p { class: "text-[11px] text-hb-matrix line-clamp-1 mt-0.5", "{product.description}" }
                                            }
                                        }
                                        div {
                                            class: "bg-hb-cytoplasm border border-hb-matrix/10 rounded-xl px-3 py-1.5 text-center shrink-0 shadow-xs",
                                            span { class: "block text-xs font-black text-hb-primary tabular-nums", "{score_rounded}" }
                                            span { class: "block text-[9px] uppercase tracking-wider text-hb-matrix font-semibold", "Score" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Features Grid Section
            div {
                class: "max-w-6xl mx-auto px-6 py-16 border-t border-hb-matrix/10 grid grid-cols-1 md:grid-cols-3 gap-8",
                
                div {
                    class: "flex flex-col gap-3 p-6 hb-squarcle bg-hb-cytoplasm border border-hb-matrix/10 shadow-sm hover:border-hb-primary/20 hover:scale-[1.01] transition-all duration-300",
                    span { class: "text-2xl", "🔬" }
                    h3 { class: "text-lg font-bold text-hb-nucleus font-display", "Granular Classification" }
                    p { class: "text-hb-matrix text-sm leading-relaxed", "Items are separated into category-specific parameters. We evaluate smartphones on battery life and repair index, but coffee beans on fragrance and body." }
                }

                div {
                    class: "flex flex-col gap-3 p-6 hb-squarcle bg-hb-cytoplasm border border-hb-matrix/10 shadow-sm hover:border-hb-primary/20 hover:scale-[1.01] transition-all duration-300",
                    span { class: "text-2xl", "🎛️" }
                    h3 { class: "text-lg font-bold text-hb-nucleus font-display", "Custom Weighting" }
                    p { class: "text-hb-matrix text-sm leading-relaxed", "Define what matters to you. Move sliders to set the importance of each metric and watch the sorting dynamically adjust to your lifestyle." }
                }

                div {
                    class: "flex flex-col gap-3 p-6 hb-squarcle bg-hb-cytoplasm border border-hb-matrix/10 shadow-sm hover:border-hb-primary/20 hover:scale-[1.01] transition-all duration-300",
                    span { class: "text-2xl", "🛡️" }
                    h3 { class: "text-lg font-bold text-hb-nucleus font-display", "Zero Marketing Noise" }
                    p { class: "text-hb-matrix text-sm leading-relaxed", "Cut through sponsored ads and sponsored hype. Krynon uses mathematical models to calculate exact value fits based purely on hard data." }
                }
            }

            // Call to Action Card
            div {
                class: "max-w-6xl mx-auto px-6 pt-8",
                div {
                    class: "p-8 sm:p-12 hb-squarcle bg-gradient-to-r from-hb-cytoplasm via-emerald-50/20 to-teal-50/10 border border-hb-primary/10 text-center relative overflow-hidden shadow-sm",
                    div { class: "absolute -right-24 -bottom-24 w-80 h-80 bg-hb-primary/5 rounded-full blur-3xl -z-10" }
                    div { class: "absolute -left-24 -top-24 w-80 h-80 bg-teal-500/5 rounded-full blur-3xl -z-10" }
                    
                    h2 { class: "text-2xl sm:text-3xl font-extrabold text-hb-nucleus tracking-tight font-display", "Ready to discover your perfect match?" }
                    p { class: "text-hb-matrix text-sm sm:text-base max-w-xl mx-auto mt-4 leading-relaxed", "Enter the comparison workspace to tweak weights, review detailed product matrices, and find the perfect smartphone, specialty coffee, or running shoe." }
                    
                    div {
                        class: "mt-8 flex justify-center",
                        Link {
                            to: Route::Compare {},
                            class: "hb-btn-pill px-8 py-4 text-center text-white shadow-md shadow-hb-primary/10 active:scale-[0.98]",
                            "Launch Compare Engine 🚀"
                        }
                    }
                }
            }
        }
    }
}
