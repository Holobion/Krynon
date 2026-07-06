use crate::components::{ProductCard, WeightSlider};
use crate::model::{
    calculate_score, get_combined_criteria, load_app_data, Category, Criterion, Product,
    ProductType,
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum WorkspaceMode {
    ProductType,
    Category,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CreationMode {
    None,
    Category,
    ProductType,
}

#[component]
pub fn Compare() -> Element {
    let data = use_resource(move || async move { load_app_data().await });

    let mut categories = use_signal(Vec::<Category>::new);
    let mut product_types = use_signal(Vec::<ProductType>::new);
    let mut products = use_signal(Vec::<Product>::new);

    let mut workspace_mode = use_signal(|| WorkspaceMode::ProductType);
    let mut selected_prod_type_idx = use_signal(|| 0);
    let mut selected_category_idx = use_signal(|| 0);
    let mut search_query = use_context::<Signal<String>>();

    let mut creation_mode = use_signal(|| CreationMode::None);
    let mut new_name = use_signal(|| "".to_string());
    let mut new_description = use_signal(|| "".to_string());
    let mut new_emoji = use_signal(|| "".to_string());
    let mut selected_criteria = use_signal(|| Vec::<String>::new());
    let mut selected_categories = use_signal(|| Vec::<String>::new());
    let mut custom_criteria = use_signal(|| Vec::<Criterion>::new());
    let mut temp_crit_name = use_signal(|| "".to_string());
    let mut temp_crit_desc = use_signal(|| "".to_string());
    let mut temp_crit_emoji = use_signal(|| "".to_string());

    // Initial weights setup. Populated once DB data is loaded.
    let mut weights = use_signal(HashMap::<String, f64>::new);

    use_effect(move || {
        if let Some(Ok(app_data)) = data() {
            categories.set(app_data.categories.clone());
            product_types.set(app_data.product_types.clone());
            products.set(app_data.products.clone());
            selected_prod_type_idx.set(0);
            selected_category_idx.set(0);

            let mut initial_weights = HashMap::new();
            if let Some(first_type) = app_data.product_types.first() {
                let combined = get_combined_criteria(first_type, &app_data.categories);
                for crit in combined {
                    initial_weights.insert(crit.id, 5.0);
                }
            }
            weights.set(initial_weights);
        }
    });

    // Synchronize weights when mode or tab selections change
    use_effect(move || {
        let mode = workspace_mode();
        let mut new_weights = HashMap::new();
        match mode {
            WorkspaceMode::ProductType => {
                let pts = product_types.read();
                if selected_prod_type_idx() < pts.len() {
                    let pt = &pts[selected_prod_type_idx()];
                    let combined = get_combined_criteria(pt, &categories.read());
                    for crit in combined {
                        new_weights.insert(crit.id, 5.0);
                    }
                }
            }
            WorkspaceMode::Category => {
                let cats = categories.read();
                if selected_category_idx() < cats.len() {
                    let cat = &cats[selected_category_idx()];
                    for crit in &cat.criteria {
                        new_weights.insert(crit.id.clone(), 5.0);
                    }
                }
            }
        }
        weights.set(new_weights);
    });

    if let Some(Err(_)) = data() {
        return rsx! { div { class: "max-w-6xl mx-auto px-6 py-20 text-rose-600", "Failed to load product data." } };
    }

    if categories.read().is_empty() || product_types.read().is_empty() {
        return rsx! { div { class: "max-w-6xl mx-auto px-6 py-20 text-hb-matrix", "Loading product data..." } };
    }

    // Get active entities and active criteria
    let (active_name, active_emoji, active_desc, active_criteria, active_presets) =
        match workspace_mode() {
            WorkspaceMode::ProductType => {
                let pt = product_types.read()[selected_prod_type_idx()].clone();
                let crit = get_combined_criteria(&pt, &categories.read());
                (pt.name, pt.emoji, pt.description, crit, pt.presets)
            }
            WorkspaceMode::Category => {
                let cat = categories.read()[selected_category_idx()].clone();
                let crit = cat.criteria.clone();
                // Generate some dynamic category presets
                let mut presets = vec![crate::model::WeightProfile {
                    name: "Balanced Benchmark".to_string(),
                    weights: crit.iter().map(|c| (c.id.clone(), 5.0)).collect(),
                }];
                // Add environment or ethics focused preset depending on criteria
                if crit.iter().any(|c| {
                    c.id == "carbon_footprint" || c.id == "e_waste" || c.id == "durability"
                }) {
                    presets.push(crate::model::WeightProfile {
                        name: "Climate First".to_string(),
                        weights: crit
                            .iter()
                            .map(|c| {
                                let w = if c.id == "carbon_footprint"
                                    || c.id == "e_waste"
                                    || c.id == "durability"
                                {
                                    10.0
                                } else {
                                    2.0
                                };
                                (c.id.clone(), w)
                            })
                            .collect(),
                    });
                }
                if crit.iter().any(|c| c.id == "sourcing_ethics") {
                    presets.push(crate::model::WeightProfile {
                        name: "Ethics First".to_string(),
                        weights: crit
                            .iter()
                            .map(|c| {
                                let w = if c.id == "sourcing_ethics" { 10.0 } else { 3.0 };
                                (c.id.clone(), w)
                            })
                            .collect(),
                    });
                }
                (cat.name, cat.emoji, cat.description, crit, presets)
            }
        };

    // Filter and Sort products (Search query suggestions are handled separately below)
    let current_weights = weights();
    let mut sorted_products = match workspace_mode() {
        WorkspaceMode::ProductType => {
            let pt = &product_types.read()[selected_prod_type_idx()];
            products
                .read()
                .iter()
                .filter(|p| p.product_type_id == pt.id)
                .cloned()
                .collect::<Vec<Product>>()
        }
        WorkspaceMode::Category => {
            let cat = &categories.read()[selected_category_idx()];
            products
                .read()
                .iter()
                .filter(|p| {
                    if let Some(pt) = product_types
                        .read()
                        .iter()
                        .find(|t| t.id == p.product_type_id)
                    {
                        pt.category_ids.contains(&cat.id)
                    } else {
                        false
                    }
                })
                .cloned()
                .collect::<Vec<Product>>()
        }
    };

    sorted_products.sort_by(|a, b| {
        let score_a = calculate_score(a, &current_weights);
        let score_b = calculate_score(b, &current_weights);
        score_b
            .partial_cmp(&score_a)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Compute suggestion query matches
    let query_str = search_query.read().to_lowercase();
    let matched_categories = if !query_str.is_empty() {
        categories
            .read()
            .iter()
            .enumerate()
            .filter(|(_, cat)| {
                cat.name.to_lowercase().contains(&query_str)
                    || cat.description.to_lowercase().contains(&query_str)
            })
            .map(|(idx, cat)| (idx, cat.clone()))
            .collect::<Vec<(usize, Category)>>()
    } else {
        Vec::new()
    };

    let matched_product_types = if !query_str.is_empty() {
        product_types
            .read()
            .iter()
            .enumerate()
            .filter(|(_, pt)| {
                pt.name.to_lowercase().contains(&query_str)
                    || pt.description.to_lowercase().contains(&query_str)
            })
            .map(|(idx, pt)| (idx, pt.clone()))
            .collect::<Vec<(usize, ProductType)>>()
    } else {
        Vec::new()
    };

    let prod_type_btn_class = if workspace_mode() == WorkspaceMode::ProductType {
        "bg-hb-primary text-white shadow-md hb-halo"
    } else {
        "text-hb-matrix hover:text-hb-nucleus"
    };

    let category_btn_class = if workspace_mode() == WorkspaceMode::Category {
        "bg-hb-primary text-white shadow-md hb-halo"
    } else {
        "text-hb-matrix hover:text-hb-nucleus"
    };

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-6 py-8",

            // Page Header
            div {
                class: "flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-8",
                div {
                    h1 { class: "text-3xl font-black text-hb-nucleus tracking-tight font-display", "Classification Workspace" }
                    p { class: "text-hb-matrix text-sm mt-1", "Inherit global criteria, configure custom weights, and perform deep comparative ranking." }
                }

                // Workspace Mode Toggle
                div {
                    class: "flex bg-hb-cytoplasm border border-hb-matrix/20 p-1 rounded-full shrink-0 shadow-sm",
                    button {
                        class: "px-5 py-2 rounded-full text-xs font-bold transition-all {prod_type_btn_class}",
                        onclick: move |_| {
                            workspace_mode.set(WorkspaceMode::ProductType);
                        },
                        "Compare by Product Type"
                    }
                    button {
                        class: "px-5 py-2 rounded-full text-xs font-bold transition-all {category_btn_class}",
                        onclick: move |_| {
                            workspace_mode.set(WorkspaceMode::Category);
                        },
                        "Compare by Category"
                    }
                }
            }

            // Prominent Workspace Search Bar
            div {
                class: "mb-8",
                div {
                    class: "relative flex items-center bg-hb-cytoplasm border border-hb-matrix/10 focus-within:border-hb-primary rounded-2xl px-5 py-4 text-hb-matrix focus-within:text-hb-nucleus transition-all shadow-sm",
                    // Large magnifying glass icon
                    svg {
                        class: "w-6 h-6 mr-4 shrink-0 text-hb-matrix/60",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2.5",
                            d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                        }
                    }
                    input {
                        class: "bg-transparent border-none outline-none text-base sm:text-lg w-full text-hb-nucleus placeholder-hb-matrix/60 font-medium",
                        value: "{search_query}",
                        placeholder: "Search product types or categories (e.g. coffee, technology)...",
                        oninput: move |e| {
                            search_query.set(e.value());
                        }
                    }
                    if !search_query.read().is_empty() {
                        button {
                            class: "hover:text-hb-primary text-hb-matrix transition-colors p-1",
                            onclick: move |_| {
                                search_query.set("".to_string());
                            },
                            // Large close icon
                            svg {
                                class: "w-5 h-5",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2.5",
                                    d: "M6 18L18 6M6 6l12 12"
                                }
                            }
                        }
                    }
                }
            }

            if !query_str.is_empty() {
                // Command Palette Suggestions View
                div {
                    class: "max-w-3xl mx-auto space-y-6 animate-fade-in-down",

                    div {
                        class: "flex justify-between items-center px-1",
                        h3 { class: "text-xs font-bold text-hb-matrix uppercase tracking-widest font-display", "Search Results" }
                        span {
                            class: "text-xs text-hb-matrix font-semibold",
                            "{matched_categories.len() + matched_product_types.len()} results found"
                        }
                    }

                    if matched_categories.is_empty() && matched_product_types.is_empty() {
                        div {
                            class: "flex flex-col items-center justify-center py-16 px-6 text-center bg-hb-cytoplasm border border-hb-matrix/10 hb-squarcle space-y-4 shadow-sm",
                            span { class: "text-4xl", "🔍" }
                            h4 { class: "text-lg font-bold text-hb-nucleus", "No matches found" }
                            p { class: "text-hb-matrix text-xs max-w-sm leading-relaxed", "We couldn't find any category or product type matching \"{search_query}\". Try searching for 'coffee', 'rice', 'tech', or 'office'." }
                            button {
                                class: "hb-btn-pill px-5 py-2.5 text-white active:scale-95 text-xs shadow-md shadow-hb-primary/10",
                                onclick: move |_| {
                                    search_query.set("".to_string());
                                },
                                "Clear Search"
                            }
                        }
                    } else {
                        div {
                            class: "space-y-3",

                            // Render Category Matches
                            for (idx, cat) in matched_categories {
                                div {
                                    key: "cat-{cat.id}",
                                    class: "bg-hb-cytoplasm hover:bg-hb-membrane border border-hb-matrix/10 hover:border-hb-primary/35 p-5 hb-squarcle cursor-pointer transition-all duration-200 group flex justify-between items-center shadow-xs",
                                    onclick: move |_| {
                                        workspace_mode.set(WorkspaceMode::Category);
                                        selected_category_idx.set(idx);
                                        search_query.set("".to_string());
                                    },
                                    div {
                                        class: "space-y-1.5 pr-4",
                                        div {
                                            class: "flex flex-wrap items-center gap-2",
                                            span { class: "text-lg", "{cat.emoji}" }
                                            h4 { class: "font-bold text-hb-nucleus group-hover:text-hb-primary transition-colors font-display", "{cat.name}" }
                                            span {
                                                class: "text-[9px] uppercase font-extrabold px-2 py-0.5 rounded bg-emerald-100 border border-emerald-200/50 text-emerald-700 tracking-wider",
                                                "Category Benchmark"
                                            }
                                        }
                                        p { class: "text-hb-matrix text-xs mt-1 line-clamp-1", "{cat.description}" }
                                    }
                                    span {
                                        class: "text-hb-matrix group-hover:text-hb-primary group-hover:translate-x-1 transition-all text-xs font-bold shrink-0",
                                        "Go to Benchmark ➔"
                                    }
                                }
                            }

                            // Render Product Type Matches
                            for (idx, pt) in matched_product_types {
                                div {
                                    key: "pt-{pt.id}",
                                    class: "bg-hb-cytoplasm hover:bg-hb-membrane border border-hb-matrix/10 hover:border-hb-primary/35 p-5 hb-squarcle cursor-pointer transition-all duration-200 group flex justify-between items-center shadow-xs",
                                    onclick: move |_| {
                                        workspace_mode.set(WorkspaceMode::ProductType);
                                        selected_prod_type_idx.set(idx);
                                        search_query.set("".to_string());
                                    },
                                    div {
                                        class: "space-y-1.5 pr-4",
                                        div {
                                            class: "flex flex-wrap items-center gap-2",
                                            span { class: "text-lg", "{pt.emoji}" }
                                            h4 { class: "font-bold text-hb-nucleus group-hover:text-hb-primary transition-colors font-display", "{pt.name}" }
                                            span {
                                                class: "text-[9px] uppercase font-extrabold px-2 py-0.5 rounded bg-hb-primary/10 border border-hb-primary/20 text-hb-primary tracking-wider",
                                                "Product Type Workspace"
                                            }
                                        }
                                        p { class: "text-hb-matrix text-xs mt-1 line-clamp-1", "{pt.description}" }
                                    }
                                    span {
                                        class: "text-hb-matrix group-hover:text-hb-primary group-hover:translate-x-1 transition-all text-xs font-bold shrink-0",
                                        "Go to Workspace ➔"
                                    }
                                }
                            }
                        }
                    }

                    // Dynamic creation option at the bottom of the results
                    if creation_mode() == CreationMode::None {
                        div {
                            class: "bg-hb-cytoplasm border border-hb-matrix/10 p-6 hb-squarcle shadow-md space-y-4 border-dashed border-2 hover:border-hb-primary/30 transition-all",
                            div {
                                class: "flex items-start gap-4",
                                span { class: "text-2xl p-2 bg-hb-membrane rounded-xl", "✨" }
                                div {
                                    h4 { class: "font-bold text-hb-nucleus font-display", "Can't find what you need?" }
                                    p { class: "text-hb-matrix text-xs mt-1 leading-relaxed", "Define a custom Category or Product Type with your own criteria to evaluate products." }
                                }
                            }
                            div {
                                class: "flex flex-wrap gap-3 pt-2",
                                button {
                                    class: "px-4 py-2 bg-hb-primary hover:bg-hb-primary/95 text-white text-xs font-bold rounded-lg transition-all active:scale-95 shadow-sm",
                                    onclick: move |_| {
                                        new_name.set(search_query());
                                        new_description.set("".to_string());
                                        new_emoji.set("📂".to_string());
                                        selected_criteria.write().clear();
                                        custom_criteria.write().clear();
                                        creation_mode.set(CreationMode::Category);
                                    },
                                    "Create Category"
                                }
                                button {
                                    class: "px-4 py-2 bg-hb-membrane hover:bg-hb-cytoplasm border border-hb-matrix/20 hover:border-hb-primary/30 text-hb-nucleus text-xs font-bold rounded-lg transition-all active:scale-95 shadow-xs",
                                    onclick: move |_| {
                                        new_name.set(search_query());
                                        new_description.set("".to_string());
                                        new_emoji.set("📦".to_string());
                                        selected_categories.write().clear();
                                        selected_criteria.write().clear();
                                        custom_criteria.write().clear();
                                        creation_mode.set(CreationMode::ProductType);
                                    },
                                    "Create Product Type"
                                }
                            }
                        }
                    }

                    if creation_mode() == CreationMode::Category {
                        div {
                            class: "bg-hb-cytoplasm border border-hb-primary/35 p-6 hb-squarcle shadow-lg space-y-6 animate-fade-in-down",
                            div {
                                class: "flex justify-between items-center border-b border-hb-matrix/10 pb-4",
                                div {
                                    h3 { class: "font-bold text-hb-nucleus text-lg font-display flex items-center gap-2",
                                        span { "📂" }
                                        span { "Create New Category" }
                                    }
                                    p { class: "text-hb-matrix text-xs mt-0.5", "Define a new evaluation category." }
                                }
                                button {
                                    class: "text-hb-matrix hover:text-hb-nucleus text-xs font-bold",
                                    onclick: move |_| {
                                        creation_mode.set(CreationMode::None);
                                    },
                                    "Cancel"
                                }
                            }

                            // Form fields
                            div {
                                class: "space-y-4",

                                div {
                                    class: "grid grid-cols-4 gap-4",
                                    div {
                                        class: "col-span-1 space-y-1.5",
                                        label { class: "text-xs font-bold text-hb-matrix", "Emoji" }
                                        input {
                                            class: "bg-hb-membrane border border-hb-matrix/10 rounded-xl px-4 py-2.5 text-center text-lg w-full focus:outline-none focus:border-hb-primary text-hb-nucleus",
                                            value: "{new_emoji}",
                                            oninput: move |e| new_emoji.set(e.value()),
                                        }
                                    }
                                    div {
                                        class: "col-span-3 space-y-1.5",
                                        label { class: "text-xs font-bold text-hb-matrix", "Category Name" }
                                        input {
                                            class: "bg-hb-membrane border border-hb-matrix/10 rounded-xl px-4 py-2.5 text-sm w-full focus:outline-none focus:border-hb-primary text-hb-nucleus font-medium",
                                            placeholder: "e.g., Household Appliances",
                                            value: "{new_name}",
                                            oninput: move |e| new_name.set(e.value()),
                                        }
                                    }
                                }

                                div {
                                    class: "space-y-1.5",
                                    label { class: "text-xs font-bold text-hb-matrix", "Description" }
                                    textarea {
                                        class: "bg-hb-membrane border border-hb-matrix/10 rounded-xl px-4 py-2.5 text-sm w-full h-20 focus:outline-none focus:border-hb-primary text-hb-nucleus resize-none",
                                        placeholder: "Explain what this category evaluates...",
                                        value: "{new_description}",
                                        oninput: move |e| new_description.set(e.value()),
                                    }
                                }

                                // Choose from existing criteria
                                div {
                                    class: "space-y-2",
                                    label { class: "text-xs font-bold text-hb-matrix block", "Inherit Existing Criteria" }
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 gap-2 bg-hb-membrane p-4 rounded-xl border border-hb-matrix/10 max-h-48 overflow-y-auto",
                                        {
                                            let all_existing_criteria = {
                                                let mut map = HashMap::new();
                                                for c in categories.read().iter() {
                                                    for crit in &c.criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                for pt in product_types.read().iter() {
                                                    for crit in &pt.specific_criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                let mut list: Vec<Criterion> = map.into_values().collect();
                                                list.sort_by(|a, b| a.name.cmp(&b.name));
                                                list
                                            };

                                            all_existing_criteria.into_iter().map(|crit| {
                                                let is_selected = selected_criteria.read().contains(&crit.id);
                                                let crit_id_clone = crit.id.clone();
                                                rsx! {
                                                    div {
                                                        key: "{crit.id}",
                                                        class: "flex items-start gap-2.5 p-2 rounded-lg hover:bg-hb-cytoplasm cursor-pointer transition-colors text-xs text-hb-nucleus",
                                                        onclick: move |_| {
                                                            let mut list = selected_criteria.read().clone();
                                                            if list.contains(&crit_id_clone) {
                                                                list.retain(|id| id != &crit_id_clone);
                                                            } else {
                                                                list.push(crit_id_clone.clone());
                                                            }
                                                            selected_criteria.set(list);
                                                        },
                                                        input {
                                                            type: "checkbox",
                                                            class: "mt-0.5 accent-hb-primary",
                                                            checked: is_selected,
                                                            readonly: true,
                                                        }
                                                        div {
                                                            class: "font-semibold flex items-center gap-1",
                                                            span { "{crit.emoji}" }
                                                            span { "{crit.name}" }
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                }

                                // Create & add a custom criteria inline
                                div {
                                    class: "space-y-3 border-t border-hb-matrix/10 pt-4",
                                    label { class: "text-xs font-bold text-hb-matrix block", "Add Custom Criteria" }

                                    // Custom criteria list added so far
                                    if !custom_criteria.read().is_empty() {
                                        div {
                                            class: "space-y-1.5",
                                            for crit in custom_criteria.read().iter() {
                                                div {
                                                    key: "{crit.id}",
                                                    class: "flex justify-between items-center bg-hb-membrane px-3 py-2 rounded-lg text-xs text-hb-nucleus border border-hb-matrix/5",
                                                    div {
                                                        class: "flex items-center gap-2",
                                                        span { "{crit.emoji}" }
                                                        span { class: "font-bold", "{crit.name}" }
                                                        span { class: "text-hb-matrix text-[10px]", "- {crit.description}" }
                                                    }
                                                    button {
                                                        class: "text-rose-500 hover:text-rose-700 font-bold",
                                                        onclick: {
                                                            let target_id = crit.id.clone();
                                                            move |_| {
                                                                custom_criteria.write().retain(|c| c.id != target_id);
                                                            }
                                                        },
                                                        "Remove"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Form fields to add a custom criterion
                                    div {
                                        class: "bg-hb-membrane p-4 rounded-xl border border-hb-matrix/10 space-y-3",
                                        div {
                                            class: "grid grid-cols-4 gap-3",
                                            div {
                                                class: "col-span-1 space-y-1",
                                                label { class: "text-[10px] font-bold text-hb-matrix", "Emoji" }
                                                input {
                                                    class: "bg-hb-cytoplasm border border-hb-matrix/10 rounded-lg px-2.5 py-1.5 text-center text-sm w-full focus:outline-none focus:border-hb-primary text-hb-nucleus",
                                                    placeholder: "✨",
                                                    value: "{temp_crit_emoji}",
                                                    oninput: move |e| temp_crit_emoji.set(e.value()),
                                                }
                                            }
                                            div {
                                                class: "col-span-3 space-y-1",
                                                label { class: "text-[10px] font-bold text-hb-matrix", "Criterion Name" }
                                                input {
                                                    class: "bg-hb-cytoplasm border border-hb-matrix/10 rounded-lg px-3 py-1.5 text-xs w-full focus:outline-none focus:border-hb-primary text-hb-nucleus font-medium",
                                                    placeholder: "e.g., Water Conservation",
                                                    value: "{temp_crit_name}",
                                                    oninput: move |e| temp_crit_name.set(e.value()),
                                                }
                                            }
                                        }
                                        div {
                                            class: "space-y-1",
                                            label { class: "text-[10px] font-bold text-hb-matrix", "Criterion Description" }
                                            input {
                                                class: "bg-hb-cytoplasm border border-hb-matrix/10 rounded-lg px-3 py-1.5 text-xs w-full focus:outline-none focus:border-hb-primary text-hb-nucleus",
                                                placeholder: "e.g., Gallons of water saved during production...",
                                                value: "{temp_crit_desc}",
                                                oninput: move |e| temp_crit_desc.set(e.value()),
                                            }
                                        }
                                        button {
                                            type: "button",
                                            class: "px-3 py-1.5 bg-hb-nucleus hover:bg-black text-white text-xs font-bold rounded-lg transition-all active:scale-95 shadow-xs w-full",
                                            onclick: move |_| {
                                                let name_val = temp_crit_name.read().trim().to_string();
                                                let desc_val = temp_crit_desc.read().trim().to_string();
                                                let emoji_val = temp_crit_emoji.read().trim().to_string();
                                                if !name_val.is_empty() {
                                                    let clean_id: String = name_val.to_lowercase()
                                                        .chars()
                                                        .map(|c| if c == ' ' { '_' } else { c })
                                                        .filter(|c| c.is_alphanumeric() || *c == '_')
                                                        .collect();
                                                    let final_emoji = if emoji_val.is_empty() { "✨".to_string() } else { emoji_val };
                                                    let new_crit = Criterion {
                                                        id: clean_id,
                                                        name: name_val,
                                                        description: desc_val,
                                                        emoji: final_emoji,
                                                    };
                                                    custom_criteria.write().push(new_crit);
                                                    temp_crit_name.set("".to_string());
                                                    temp_crit_desc.set("".to_string());
                                                    temp_crit_emoji.set("".to_string());
                                                }
                                            },
                                            "+ Add Custom Criterion"
                                        }
                                    }
                                }
                            }

                            // Save button
                            div {
                                class: "flex justify-end gap-3 border-t border-hb-matrix/10 pt-4",
                                button {
                                    class: "px-5 py-2.5 bg-hb-membrane hover:bg-hb-cytoplasm border border-hb-matrix/20 text-hb-nucleus text-xs font-bold rounded-lg transition-all active:scale-95",
                                    onclick: move |_| {
                                        creation_mode.set(CreationMode::None);
                                    },
                                    "Cancel"
                                }
                                button {
                                    class: "hb-btn-pill px-6 py-2.5 text-white active:scale-95 text-xs shadow-md shadow-hb-primary/10",
                                    onclick: move |_| {
                                        let name_val = new_name.read().trim().to_string();
                                        if !name_val.is_empty() {
                                            let clean_id: String = name_val.to_lowercase()
                                                .chars()
                                                .map(|c| if c == ' ' { '_' } else { c })
                                                .filter(|c| c.is_alphanumeric() || *c == '_')
                                                .collect();

                                            // Gather all criteria selected or custom-created
                                            let mut final_criteria = Vec::new();
                                            let all_existing_criteria = {
                                                let mut map = HashMap::new();
                                                for c in categories.read().iter() {
                                                    for crit in &c.criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                for pt in product_types.read().iter() {
                                                    for crit in &pt.specific_criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                map
                                            };
                                            for id in selected_criteria.read().iter() {
                                                if let Some(crit) = all_existing_criteria.get(id) {
                                                    final_criteria.push(crit.clone());
                                                }
                                            }
                                            final_criteria.extend(custom_criteria.read().clone());

                                            let new_cat = Category {
                                                id: clean_id,
                                                name: name_val,
                                                description: new_description.read().trim().to_string(),
                                                emoji: new_emoji.read().trim().to_string(),
                                                criteria: final_criteria,
                                            };

                                            categories.write().push(new_cat);
                                            let new_idx = categories.read().len() - 1;
                                            workspace_mode.set(WorkspaceMode::Category);
                                            selected_category_idx.set(new_idx);
                                            search_query.set("".to_string());
                                            creation_mode.set(CreationMode::None);
                                        }
                                    },
                                    "Save Category"
                                }
                            }
                        }
                    }

                    if creation_mode() == CreationMode::ProductType {
                        div {
                            class: "bg-hb-cytoplasm border border-hb-primary/35 p-6 hb-squarcle shadow-lg space-y-6 animate-fade-in-down",
                            div {
                                class: "flex justify-between items-center border-b border-hb-matrix/10 pb-4",
                                div {
                                    h3 { class: "font-bold text-hb-nucleus text-lg font-display flex items-center gap-2",
                                        span { "📦" }
                                        span { "Create New Product Type" }
                                    }
                                    p { class: "text-hb-matrix text-xs mt-0.5", "Establish a product type inheriting from categories." }
                                }
                                button {
                                    class: "text-hb-matrix hover:text-hb-nucleus text-xs font-bold",
                                    onclick: move |_| {
                                        creation_mode.set(CreationMode::None);
                                    },
                                    "Cancel"
                                }
                            }

                            // Form fields
                            div {
                                class: "space-y-4",

                                div {
                                    class: "grid grid-cols-4 gap-4",
                                    div {
                                        class: "col-span-1 space-y-1.5",
                                        label { class: "text-xs font-bold text-hb-matrix", "Emoji" }
                                        input {
                                            class: "bg-hb-membrane border border-hb-matrix/10 rounded-xl px-4 py-2.5 text-center text-lg w-full focus:outline-none focus:border-hb-primary text-hb-nucleus",
                                            value: "{new_emoji}",
                                            oninput: move |e| new_emoji.set(e.value()),
                                        }
                                    }
                                    div {
                                        class: "col-span-3 space-y-1.5",
                                        label { class: "text-xs font-bold text-hb-matrix", "Product Type Name" }
                                        input {
                                            class: "bg-hb-membrane border border-hb-matrix/10 rounded-xl px-4 py-2.5 text-sm w-full focus:outline-none focus:border-hb-primary text-hb-nucleus font-medium",
                                            placeholder: "e.g., Air Purifiers",
                                            value: "{new_name}",
                                            oninput: move |e| new_name.set(e.value()),
                                        }
                                    }
                                }

                                div {
                                    class: "space-y-1.5",
                                    label { class: "text-xs font-bold text-hb-matrix", "Description" }
                                    textarea {
                                        class: "bg-hb-membrane border border-hb-matrix/10 rounded-xl px-4 py-2.5 text-sm w-full h-20 focus:outline-none focus:border-hb-primary text-hb-nucleus resize-none",
                                        placeholder: "Explain what this product type evaluates...",
                                        value: "{new_description}",
                                        oninput: move |e| new_description.set(e.value()),
                                    }
                                }

                                // Choose categories (for inheritance of criteria)
                                div {
                                    class: "space-y-2",
                                    label { class: "text-xs font-bold text-hb-matrix block", "Parent Categories (Inherit Criteria)" }
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 gap-2 bg-hb-membrane p-4 rounded-xl border border-hb-matrix/10 max-h-40 overflow-y-auto",
                                        for cat in categories.read().iter() {
                                            {
                                                let is_selected = selected_categories.read().contains(&cat.id);
                                                let cat_id_clone = cat.id.clone();
                                                rsx! {
                                                    div {
                                                        key: "{cat.id}",
                                                        class: "flex items-start gap-2.5 p-2 rounded-lg hover:bg-hb-cytoplasm cursor-pointer transition-colors text-xs text-hb-nucleus",
                                                        onclick: move |_| {
                                                            let mut list = selected_categories.read().clone();
                                                            if list.contains(&cat_id_clone) {
                                                                list.retain(|id| id != &cat_id_clone);
                                                            } else {
                                                                list.push(cat_id_clone.clone());
                                                            }
                                                            selected_categories.set(list);
                                                        },
                                                        input {
                                                            type: "checkbox",
                                                            class: "mt-0.5 accent-hb-primary",
                                                            checked: is_selected,
                                                            readonly: true,
                                                        }
                                                        div {
                                                            class: "font-semibold flex items-center gap-1",
                                                            span { "{cat.emoji}" }
                                                            span { "{cat.name}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Specific Criteria checkboxes
                                div {
                                    class: "space-y-2",
                                    label { class: "text-xs font-bold text-hb-matrix block", "Include Specific Criteria" }
                                    div {
                                        class: "grid grid-cols-1 md:grid-cols-2 gap-2 bg-hb-membrane p-4 rounded-xl border border-hb-matrix/10 max-h-40 overflow-y-auto",
                                        {
                                            let all_existing_criteria = {
                                                let mut map = HashMap::new();
                                                for c in categories.read().iter() {
                                                    for crit in &c.criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                for pt in product_types.read().iter() {
                                                    for crit in &pt.specific_criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                let mut list: Vec<Criterion> = map.into_values().collect();
                                                list.sort_by(|a, b| a.name.cmp(&b.name));
                                                list
                                            };

                                            all_existing_criteria.into_iter().map(|crit| {
                                                let is_selected = selected_criteria.read().contains(&crit.id);
                                                let crit_id_clone = crit.id.clone();
                                                rsx! {
                                                    div {
                                                        key: "{crit.id}",
                                                        class: "flex items-start gap-2.5 p-2 rounded-lg hover:bg-hb-cytoplasm cursor-pointer transition-colors text-xs text-hb-nucleus",
                                                        onclick: move |_| {
                                                            let mut list = selected_criteria.read().clone();
                                                            if list.contains(&crit_id_clone) {
                                                                list.retain(|id| id != &crit_id_clone);
                                                            } else {
                                                                list.push(crit_id_clone.clone());
                                                            }
                                                            selected_criteria.set(list);
                                                        },
                                                        input {
                                                            type: "checkbox",
                                                            class: "mt-0.5 accent-hb-primary",
                                                            checked: is_selected,
                                                            readonly: true,
                                                        }
                                                        div {
                                                            class: "font-semibold flex items-center gap-1",
                                                            span { "{crit.emoji}" }
                                                            span { "{crit.name}" }
                                                        }
                                                    }
                                                }
                                            })
                                        }
                                    }
                                }

                                // Create & add a custom criteria inline
                                div {
                                    class: "space-y-3 border-t border-hb-matrix/10 pt-4",
                                    label { class: "text-xs font-bold text-hb-matrix block", "Add Custom Specific Criteria" }

                                    // Custom criteria list added so far
                                    if !custom_criteria.read().is_empty() {
                                        div {
                                            class: "space-y-1.5",
                                            for crit in custom_criteria.read().iter() {
                                                div {
                                                    key: "{crit.id}",
                                                    class: "flex justify-between items-center bg-hb-membrane px-3 py-2 rounded-lg text-xs text-hb-nucleus border border-hb-matrix/5",
                                                    div {
                                                        class: "flex items-center gap-2",
                                                        span { "{crit.emoji}" }
                                                        span { class: "font-bold", "{crit.name}" }
                                                        span { class: "text-hb-matrix text-[10px]", "- {crit.description}" }
                                                    }
                                                    button {
                                                        class: "text-rose-500 hover:text-rose-700 font-bold",
                                                        onclick: {
                                                            let target_id = crit.id.clone();
                                                            move |_| {
                                                                custom_criteria.write().retain(|c| c.id != target_id);
                                                            }
                                                        },
                                                        "Remove"
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Form fields to add a custom specific criterion
                                    div {
                                        class: "bg-hb-membrane p-4 rounded-xl border border-hb-matrix/10 space-y-3",
                                        div {
                                            class: "grid grid-cols-4 gap-3",
                                            div {
                                                class: "col-span-1 space-y-1",
                                                label { class: "text-[10px] font-bold text-hb-matrix", "Emoji" }
                                                input {
                                                    class: "bg-hb-cytoplasm border border-hb-matrix/10 rounded-lg px-2.5 py-1.5 text-center text-sm w-full focus:outline-none focus:border-hb-primary text-hb-nucleus",
                                                    placeholder: "✨",
                                                    value: "{temp_crit_emoji}",
                                                    oninput: move |e| temp_crit_emoji.set(e.value()),
                                                }
                                            }
                                            div {
                                                class: "col-span-3 space-y-1",
                                                label { class: "text-[10px] font-bold text-hb-matrix", "Criterion Name" }
                                                input {
                                                    class: "bg-hb-cytoplasm border border-hb-matrix/10 rounded-lg px-3 py-1.5 text-xs w-full focus:outline-none focus:border-hb-primary text-hb-nucleus font-medium",
                                                    placeholder: "e.g., Filtration Efficiency",
                                                    value: "{temp_crit_name}",
                                                    oninput: move |e| temp_crit_name.set(e.value()),
                                                }
                                            }
                                        }
                                        div {
                                            class: "space-y-1",
                                            label { class: "text-[10px] font-bold text-hb-matrix", "Criterion Description" }
                                            input {
                                                class: "bg-hb-cytoplasm border border-hb-matrix/10 rounded-lg px-3 py-1.5 text-xs w-full focus:outline-none focus:border-hb-primary text-hb-nucleus",
                                                placeholder: "e.g., HEPA filter capture rate of fine dust particles...",
                                                value: "{temp_crit_desc}",
                                                oninput: move |e| temp_crit_desc.set(e.value()),
                                            }
                                        }
                                        button {
                                            type: "button",
                                            class: "px-3 py-1.5 bg-hb-nucleus hover:bg-black text-white text-xs font-bold rounded-lg transition-all active:scale-95 shadow-xs w-full",
                                            onclick: move |_| {
                                                let name_val = temp_crit_name.read().trim().to_string();
                                                let desc_val = temp_crit_desc.read().trim().to_string();
                                                let emoji_val = temp_crit_emoji.read().trim().to_string();
                                                if !name_val.is_empty() {
                                                    let clean_id: String = name_val.to_lowercase()
                                                        .chars()
                                                        .map(|c| if c == ' ' { '_' } else { c })
                                                        .filter(|c| c.is_alphanumeric() || *c == '_')
                                                        .collect();
                                                    let final_emoji = if emoji_val.is_empty() { "✨".to_string() } else { emoji_val };
                                                    let new_crit = Criterion {
                                                        id: clean_id,
                                                        name: name_val,
                                                        description: desc_val,
                                                        emoji: final_emoji,
                                                    };
                                                    custom_criteria.write().push(new_crit);
                                                    temp_crit_name.set("".to_string());
                                                    temp_crit_desc.set("".to_string());
                                                    temp_crit_emoji.set("".to_string());
                                                }
                                            },
                                            "+ Add Custom Criterion"
                                        }
                                    }
                                }
                            }

                            // Save button
                            div {
                                class: "flex justify-end gap-3 border-t border-hb-matrix/10 pt-4",
                                button {
                                    class: "px-5 py-2.5 bg-hb-membrane hover:bg-hb-cytoplasm border border-hb-matrix/20 text-hb-nucleus text-xs font-bold rounded-lg transition-all active:scale-95",
                                    onclick: move |_| {
                                        creation_mode.set(CreationMode::None);
                                    },
                                    "Cancel"
                                }
                                button {
                                    class: "hb-btn-pill px-6 py-2.5 text-white active:scale-95 text-xs shadow-md shadow-hb-primary/10",
                                    onclick: move |_| {
                                        let name_val = new_name.read().trim().to_string();
                                        if !name_val.is_empty() {
                                            let clean_id: String = name_val.to_lowercase()
                                                .chars()
                                                .map(|c| if c == ' ' { '_' } else { c })
                                                .filter(|c| c.is_alphanumeric() || *c == '_')
                                                .collect();

                                            // Gather specific criteria: selected or custom-created
                                            let mut final_specific_criteria = Vec::new();
                                            let all_existing_criteria = {
                                                let mut map = HashMap::new();
                                                for c in categories.read().iter() {
                                                    for crit in &c.criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                for pt in product_types.read().iter() {
                                                    for crit in &pt.specific_criteria {
                                                        map.insert(crit.id.clone(), crit.clone());
                                                    }
                                                }
                                                map
                                            };
                                            for id in selected_criteria.read().iter() {
                                                if let Some(crit) = all_existing_criteria.get(id) {
                                                    final_specific_criteria.push(crit.clone());
                                                }
                                            }
                                            final_specific_criteria.extend(custom_criteria.read().clone());

                                            // Generate initial weight profile map
                                            let mut weights_map = HashMap::new();
                                            for cat_id in selected_categories.read().iter() {
                                                if let Some(cat) = categories.read().iter().find(|c| &c.id == cat_id) {
                                                    for crit in &cat.criteria {
                                                        weights_map.insert(crit.id.clone(), 5.0);
                                                    }
                                                }
                                            }
                                            for crit in &final_specific_criteria {
                                                weights_map.insert(crit.id.clone(), 5.0);
                                            }

                                            let default_preset = crate::model::WeightProfile {
                                                name: "Balanced Default".to_string(),
                                                weights: weights_map,
                                            };

                                            let new_pt = ProductType {
                                                id: clean_id,
                                                name: name_val,
                                                description: new_description.read().trim().to_string(),
                                                emoji: new_emoji.read().trim().to_string(),
                                                category_ids: selected_categories.read().clone(),
                                                specific_criteria: final_specific_criteria,
                                                presets: vec![default_preset],
                                            };

                                            product_types.write().push(new_pt);
                                            let new_idx = product_types.read().len() - 1;
                                            workspace_mode.set(WorkspaceMode::ProductType);
                                            selected_prod_type_idx.set(new_idx);
                                            search_query.set("".to_string());
                                            creation_mode.set(CreationMode::None);
                                        }
                                    },
                                    "Save Product Type"
                                }
                            }
                        }
                    }
                }
            } else {
                // Tab Selector based on workspace mode
                    div {
                        class: "flex flex-wrap border-b border-hb-matrix/10 mb-8 gap-1",
                        match workspace_mode() {
                            WorkspaceMode::ProductType => {
                                rsx! {
                                    for (idx, pt) in product_types.read().iter().enumerate() {
                                        {
                                            let is_active = selected_prod_type_idx() == idx;
                                            let tab_class = if is_active {
                                                "border-hb-primary text-hb-primary font-extrabold"
                                            } else {
                                                "border-transparent text-hb-matrix hover:text-hb-nucleus hover:border-hb-matrix/30"
                                            };
                                            rsx! {
                                                button {
                                                    key: "{idx}",
                                                    class: "px-5 py-3 text-sm font-bold flex items-center gap-2 border-b-2 transition-all duration-200 -mb-[2px] {tab_class}",
                                                    onclick: move |_| {
                                                        selected_prod_type_idx.set(idx);
                                                    },
                                                    span { "{pt.emoji}" }
                                                    span { "{pt.name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            WorkspaceMode::Category => {
                                rsx! {
                                    for (idx, cat) in categories.read().iter().enumerate() {
                                        {
                                            let is_active = selected_category_idx() == idx;
                                            let tab_class = if is_active {
                                                "border-hb-primary text-hb-primary font-extrabold"
                                            } else {
                                                "border-transparent text-hb-matrix hover:text-hb-nucleus hover:border-hb-matrix/30"
                                            };
                                            rsx! {
                                                button {
                                                    key: "{idx}",
                                                    class: "px-5 py-3 text-sm font-bold flex items-center gap-2 border-b-2 transition-all duration-200 -mb-[2px] {tab_class}",
                                                    onclick: move |_| {
                                                        selected_category_idx.set(idx);
                                                    },
                                                    span { "{cat.emoji}" }
                                                    span { "{cat.name}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Workspace Layout Grid
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-start",

                        // Left Column: Weight Sliders & Presets (5 cols)
                        div {
                            class: "lg:col-span-5 space-y-6 lg:sticky lg:top-24",

                            // Selected entity card (Product Type or Category)
                            div {
                                class: "bg-hb-cytoplasm border border-hb-matrix/10 p-5 hb-squarcle shadow-sm",
                                div {
                                    class: "flex justify-between items-start",
                                    h2 { class: "text-lg font-bold text-hb-nucleus flex items-center gap-2 font-display",
                                        span { "{active_emoji}" }
                                        span { "{active_name}" }
                                    }
                                    span {
                                        class: "text-[10px] uppercase font-extrabold px-2 py-0.5 rounded bg-hb-membrane border border-hb-matrix/10 text-hb-primary",
                                        match workspace_mode() {
                                            WorkspaceMode::ProductType => "Product Type",
                                            WorkspaceMode::Category => "Global Category",
                                        }
                                    }
                                }
                                p { class: "text-hb-matrix text-xs mt-2.5 leading-relaxed", "{active_desc}" }
                            }

                            // Presets Selection
                            if !active_presets.is_empty() {
                                div {
                                    class: "bg-hb-cytoplasm border border-hb-matrix/10 p-5 hb-squarcle space-y-3 shadow-sm",
                                    h3 { class: "text-xs font-bold text-hb-matrix uppercase tracking-widest font-display", "Quick Weight Presets" }
                                    div {
                                        class: "flex flex-wrap gap-2",
                                        for preset in &active_presets {
                                            {
                                                let preset = preset.clone();
                                                rsx! {
                                                    button {
                                                        key: "{preset.name}",
                                                        class: "px-4 py-1.5 bg-hb-membrane hover:bg-hb-cytoplasm border border-hb-matrix/15 hover:border-hb-primary/30 rounded-full text-xs font-semibold text-hb-matrix hover:text-hb-nucleus transition-all active:scale-95",
                                                        onclick: move |_| {
                                                            weights.set(preset.weights.clone());
                                                        },
                                                        "{preset.name}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Sliders Container
                            div {
                                class: "space-y-3.5",
                                div {
                                    class: "flex justify-between items-center px-1",
                                    h3 { class: "text-xs font-bold text-hb-matrix uppercase tracking-widest font-display", "Customize Criteria Weight" }
                                    button {
                                        class: "text-[10px] text-hb-primary hover:text-hb-primary/80 font-bold",
                                        onclick: move |_| {
                                            let mut reset_map = HashMap::new();
                                            for crit in &active_criteria {
                                                reset_map.insert(crit.id.clone(), 5.0);
                                            }
                                            weights.set(reset_map);
                                        },
                                        "Reset All to 5.0"
                                    }
                                }

                                div {
                                    class: "space-y-3",
                                    for criterion in &active_criteria {
                                        {
                                            let criterion = criterion.clone();
                                            let weight_val = weights().get(&criterion.id).copied().unwrap_or(5.0);
                                            rsx! {
                                                WeightSlider {
                                                    key: "{criterion.id}",
                                                    id: criterion.id.clone(),
                                                    name: criterion.name.clone(),
                                                    description: criterion.description.clone(),
                                                    emoji: criterion.emoji.clone(),
                                                    weight: weight_val,
                                                    onchange: move |new_val| {
                                                        let mut w_map = weights.read().clone();
                                                        w_map.insert(criterion.id.clone(), new_val);
                                                        weights.set(w_map);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                    // Right Column: Ranked Results (7 cols)
                    div {
                        class: "lg:col-span-7 space-y-4",

                        div {
                            class: "flex justify-between items-center px-2",
                            h3 { class: "text-xs font-bold text-hb-matrix uppercase tracking-widest font-display", "Analytical Ranking" }
                            span { class: "text-xs text-hb-matrix font-semibold", "{sorted_products.len()} Items Sorted" }
                        }

                        div {
                            class: "space-y-4",
                            for (idx, product) in sorted_products.iter().enumerate() {
                                ProductCard {
                                    key: "{product.id}",
                                    rank: idx + 1,
                                    product: product.clone(),
                                    criteria: active_criteria.clone(),
                                    weights: weights()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
