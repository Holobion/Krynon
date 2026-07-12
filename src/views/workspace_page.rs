use crate::components::{ProductCard, WeightSlider};
use crate::model::{
    calculate_score, create_product, get_combined_criteria, load_app_data, Category,
    NewProductInput, Product, ProductType,
};
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn WorkspacePage(id: String) -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();
    let mut search_query = use_context::<Signal<String>>();
    let data = use_resource(move || async move { load_app_data(lang().as_code().to_string()).await });
    let pt_id = id.clone();

    let mut categories = use_signal(Vec::<Category>::new);
    let mut product_types = use_signal(Vec::<ProductType>::new);
    let mut products = use_signal(Vec::<Product>::new);
    let mut weights = use_signal(HashMap::<String, f64>::new);
    let mut show_add_product_form = use_signal(|| false);
    let mut new_product_name = use_signal(|| "".to_string());
    let mut new_product_price = use_signal(|| "".to_string());
    let mut new_product_quantity = use_signal(|| "".to_string());
    let mut new_product_unit = use_signal(|| "".to_string());
    let mut new_product_description = use_signal(|| "".to_string());
    let mut new_product_scores = use_signal(HashMap::<String, f64>::new);
    let mut is_saving_product = use_signal(|| false);
    let mut add_product_error = use_signal(|| None::<String>);

    let pt_id_effect = pt_id.clone();
    use_effect(move || {
        if let Some(Ok(app_data)) = data() {
            categories.set(app_data.categories.clone());
            product_types.set(app_data.product_types.clone());
            products.set(app_data.products.clone());
            if let Some(pt) = app_data.product_types.iter().find(|p| p.id == pt_id_effect) {
                let combined = get_combined_criteria(pt, &app_data.categories);
                let initial: HashMap<String, f64> = combined.iter().map(|c| (c.id.clone(), 5.0)).collect();
                weights.set(initial);
            }
        }
    });

    if let Some(Err(_)) = data() {
        return rsx! { div { class: "max-w-6xl mx-auto px-6 py-20 text-rose-600", "{lang().t(\"Failed to load product data.\")}" } };
    }
    if categories.read().is_empty() || product_types.read().is_empty() {
        return rsx! { div { class: "max-w-6xl mx-auto px-6 py-20 text-kr-text-matrix font-mono text-xs animate-pulse", "{lang().t(\"Loading classification data...\")}" } };
    }

    let maybe_pt = product_types.read().iter().find(|p| p.id == pt_id).cloned();
    let pt = match maybe_pt {
        Some(p) => p,
        None => return rsx! {
            div { class: "max-w-6xl mx-auto px-6 py-20 text-center space-y-4",
                h2 { class: "text-xl font-bold text-kr-text-nucleus font-display", "{lang().t(\"Product type not found\")}" }
                Link { to: Route::Compare {}, class: "text-kr-turquoise text-sm hover:underline", "{lang().t(\"← Back to Workspace\")}" }
            }
        },
    };

    let active_criteria = get_combined_criteria(&pt, &categories.read());
    let active_presets = pt.presets.clone();

    // Parent category for breadcrumb
    let parent_cat = pt.category_ids.first().and_then(|cat_id| {
        categories.read().iter().find(|c| &c.id == cat_id).cloned()
    });

    let current_weights = weights();
    let query_str = search_query.read().to_lowercase();
    let save_error = add_product_error.read().clone();

    let mut sorted_products: Vec<Product> = products.read().iter()
        .filter(|p| p.product_type_id == pt.id)
        .filter(|p| {
            query_str.is_empty()
                || p.name.to_lowercase().contains(&query_str)
                || p.description.to_lowercase().contains(&query_str)
        })
        .cloned()
        .collect();
    sorted_products.sort_by(|a, b| {
        calculate_score(b, &current_weights).partial_cmp(&calculate_score(a, &current_weights))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let active_criteria_for_reset = active_criteria.clone();
    let active_criteria_for_sliders = active_criteria.clone();
    let active_criteria_for_save = active_criteria.clone();
    let active_criteria_for_cards = active_criteria.clone();

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-6 py-8",

            // Breadcrumb
            div {
                class: "flex items-center gap-2 font-mono text-xs uppercase tracking-wider text-kr-text-matrix mb-6",
                Link { to: Route::Compare {}, class: "hover:text-kr-turquoise transition-colors", "{lang().t(\"Workspace\")}" }
                span { class: "text-kr-text-nucleus/30", "/" }
                if let Some(ref cat) = parent_cat {
                    Link {
                        to: Route::CategoryPage { id: cat.id.clone() },
                        class: "hover:text-kr-turquoise transition-colors",
                        "{lang().tr(&cat.name)}"
                    }
                    span { class: "text-kr-text-nucleus/30", "/" }
                }
                span { class: "text-kr-text-nucleus font-bold", "{lang().tr(&pt.name)}" }
            }

            // Page header
            div {
                class: "border-b-1.5 border-kr-text-nucleus pb-6 mb-8",
                div {
                    class: "flex items-center gap-3 mb-2",
                    div { class: "w-2 h-2 bg-kr-turquoise border border-kr-text-nucleus shrink-0" }
                    span { class: "font-mono text-[10px] uppercase tracking-widest text-kr-text-matrix", "{lang().t(\"Product Type // Analytical Workspace\")}" }
                }
                div {
                    class: "flex items-start gap-4",
                    span { class: "text-4xl leading-none mt-1", "{pt.emoji}" }
                    div {
                        h1 { class: "text-3xl font-black text-kr-text-nucleus tracking-tight font-display uppercase", "{lang().tr(&pt.name)}" }
                        p { class: "font-serif italic text-kr-text-matrix mt-1 leading-relaxed max-w-2xl text-sm", "{lang().tr(&pt.description)}" }
                    }
                }
            }

            // Search bar
            div {
                class: "mb-8",
                div {
                    class: "relative flex items-center bg-kr-cytoplasm border border-kr-text-nucleus focus-within:border-kr-turquoise px-5 py-4 bg-grid-pattern transition-all",
                    svg { class: "w-6 h-6 mr-4 shrink-0 text-kr-text-nucleus", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2.5", d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" }
                    }
                    input {
                        class: "bg-transparent border-none outline-none text-base w-full text-kr-text-nucleus placeholder-kr-text-matrix/60 font-medium",
                        value: "{search_query}",
                        placeholder: "{lang().t(\"Filter products...\")  }",
                        oninput: move |e| search_query.set(e.value()),
                    }
                    if !search_query.read().is_empty() {
                        button {
                            class: "hover:text-kr-turquoise text-kr-text-matrix transition-colors p-1",
                            onclick: move |_| search_query.set("".to_string()),
                            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                                path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2.5", d: "M6 18L18 6M6 6l12 12" }
                            }
                        }
                    }
                }
            }

            // Two-column layout
            div {
                class: "grid grid-cols-1 lg:grid-cols-12 gap-8 items-start",

                // Left: Sliders + Presets
                div {
                    class: "lg:col-span-5 space-y-6 lg:sticky lg:top-24",

                    // Entity info card
                    div {
                        class: "bg-kr-cytoplasm border border-kr-text-nucleus/20 p-5",
                        div {
                            class: "flex justify-between items-start",
                            h2 { class: "text-base font-bold text-kr-text-nucleus flex items-center gap-2 font-display",
                                span { "{pt.emoji}" }
                                span { "{lang().tr(&pt.name)}" }
                            }
                            span {
                                class: "text-[10px] uppercase font-extrabold px-2 py-0.5 border border-kr-text-nucleus/20 text-kr-turquoise font-mono",
                                "{lang().t(\"Product Type\")}"
                            }
                        }
                        p { class: "text-kr-text-matrix text-xs mt-2.5 leading-relaxed font-serif italic", "{lang().tr(&pt.description)}" }
                    }

                    // Presets
                    if !active_presets.is_empty() {
                        div {
                            class: "bg-kr-cytoplasm border border-kr-text-nucleus/20 p-5 space-y-3",
                            h3 { class: "text-xs font-bold text-kr-text-matrix uppercase tracking-widest font-display", "{lang().t(\"Quick Weight Presets\")}" }
                            div {
                                class: "flex flex-wrap gap-2",
                                for preset in &active_presets {
                                    {
                                        let preset = preset.clone();
                                        rsx! {
                                            button {
                                                key: "{preset.name}",
                                                class: "px-4 py-1 bg-kr-membrane hover:bg-kr-cytoplasm border border-kr-text-nucleus/15 hover:border-kr-turquoise/30 text-xs font-semibold text-kr-text-matrix hover:text-kr-text-nucleus transition-all active:scale-95",
                                                onclick: move |_| weights.set(preset.weights.clone()),
                                                "{lang().tr(&preset.name)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Sliders
                    div {
                        class: "space-y-3.5",
                        div {
                            class: "flex justify-between items-center px-1",
                            h3 { class: "text-xs font-bold text-kr-text-matrix uppercase tracking-widest font-display", "{lang().t(\"Customize Criteria Weight\")}" }
                            button {
                                class: "text-[10px] text-kr-turquoise hover:text-kr-turquoise/80 font-bold",
                                onclick: move |_| {
                                    let reset: HashMap<String, f64> = active_criteria_for_reset.iter().map(|c| (c.id.clone(), 5.0)).collect();
                                    weights.set(reset);
                                },
                                "{lang().t(\"Reset All to 5.0\")}"
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
                                                let mut w = weights.read().clone();
                                                w.insert(criterion.id.clone(), new_val);
                                                weights.set(w);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Right: Ranked results
                div {
                    class: "lg:col-span-7 space-y-4",

                    // Header row
                    div {
                        class: "flex justify-between items-center px-2",
                        h3 { class: "text-xs font-bold text-kr-text-matrix uppercase tracking-widest font-display", "{lang().t(\"Analytical Ranking\")}" }
                        div {
                            class: "flex items-center gap-3",
                            span { class: "text-xs text-kr-text-matrix font-semibold", "{sorted_products.len()}{lang().t(\" Items Sorted\")}" }
                            button {
                                style: if show_add_product_form() { "background-color: transparent !important; color: var(--color-kr-turquoise) !important; border-color: var(--color-kr-turquoise) !important;" } else { "background-color: var(--color-kr-turquoise) !important; color: white !important;" },
                                class: "kr-btn-pill px-3 py-1 text-[11px] transition-all active:scale-95 shadow-sm",
                                onclick: move |_| show_add_product_form.set(!show_add_product_form()),
                                if show_add_product_form() { "{lang().t(\"Cancel\")}" } else { "{lang().t(\"+ Add Product\")}" }
                            }
                        }
                    }

                    // Add product form
                    if show_add_product_form() {
                        div {
                            class: "bg-kr-cytoplasm border border-kr-turquoise p-6 space-y-4 animate-fade-in-down bg-dot-pattern",
                            h4 { class: "font-bold text-kr-text-nucleus text-base font-display flex items-center gap-2",
                                span { "✨" }
                                span { "{lang().t(\"Add Product to \")}{lang().tr(&pt.name)}" }
                            }
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    class: "space-y-1.5",
                                    label { class: "text-xs font-bold text-kr-text-matrix", "{lang().t(\"Product Name\")}" }
                                    input {
                                        class: "bg-kr-membrane border border-kr-text-nucleus/10 px-4 py-2 text-xs w-full focus:outline-none focus:border-kr-turquoise text-kr-text-nucleus font-medium",
                                        placeholder: "{lang().t(\"e.g., Fairphone 6\")  }",
                                        value: "{new_product_name}",
                                        oninput: move |e| new_product_name.set(e.value()),
                                    }
                                }
                                div {
                                    class: "grid grid-cols-3 gap-2",
                                    div {
                                        class: "col-span-2 space-y-1.5",
                                        label { class: "text-xs font-bold text-kr-text-matrix", "{lang().t(\"Price ($)\")}" }
                                        input {
                                            type: "number", step: "0.01", min: "0.0",
                                            class: "bg-kr-membrane border border-kr-text-nucleus/10 px-4 py-2 text-xs w-full focus:outline-none focus:border-kr-turquoise text-kr-text-nucleus font-medium",
                                            placeholder: "0.00",
                                            value: "{new_product_price}",
                                            oninput: move |e| new_product_price.set(e.value()),
                                        }
                                    }
                                    div {
                                        class: "col-span-1 space-y-1.5",
                                        label { class: "text-xs font-bold text-kr-text-matrix", "{lang().t(\"Qty\")}" }
                                        input {
                                            type: "number", step: "0.01", min: "0.0",
                                            class: "bg-kr-membrane border border-kr-text-nucleus/10 px-3 py-2 text-xs w-full focus:outline-none focus:border-kr-turquoise text-kr-text-nucleus font-medium",
                                            placeholder: "1",
                                            value: "{new_product_quantity}",
                                            oninput: move |e| new_product_quantity.set(e.value()),
                                        }
                                    }
                                }
                            }
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                div {
                                    class: "space-y-1.5",
                                    label { class: "text-xs font-bold text-kr-text-matrix", "{lang().t(\"Unit\")}" }
                                    select {
                                        class: "bg-kr-membrane border border-kr-text-nucleus/10 px-4 py-2 text-xs w-full focus:outline-none focus:border-kr-turquoise text-kr-text-nucleus font-medium",
                                        value: "{new_product_unit}",
                                        onchange: move |e| new_product_unit.set(e.value()),
                                        option { value: "", "{lang().t(\"None (per piece)\")}" }
                                        option { value: "kg", "{lang().t(\"Kilogram (kg)\")}" }
                                        option { value: "L", "{lang().t(\"Liter (L)\")}" }
                                        option { value: "m", "{lang().t(\"Meter (m)\")}" }
                                    }
                                }
                                div {
                                    class: "space-y-1.5",
                                    label { class: "text-xs font-bold text-kr-text-matrix", "{lang().t(\"Description\")}" }
                                    input {
                                        class: "bg-kr-membrane border border-kr-text-nucleus/10 px-4 py-2 text-xs w-full focus:outline-none focus:border-kr-turquoise text-kr-text-nucleus font-medium",
                                        placeholder: "{lang().t(\"Brief description...\")  }",
                                        value: "{new_product_description}",
                                        oninput: move |e| new_product_description.set(e.value()),
                                    }
                                }
                            }
                            // Criteria scores
                            div {
                                class: "space-y-2 border-t border-kr-text-matrix/10 pt-4",
                                label { class: "text-xs font-bold text-kr-text-matrix block", "{lang().t(\"Evaluate Criteria Scores (0 - 10)\")}" }
                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 gap-4 bg-kr-membrane p-4 max-h-60 overflow-y-auto bg-grid-pattern",
                                    for crit in &active_criteria_for_sliders {
                                        {
                                            let crit = crit.clone();
                                            let score = new_product_scores.read().get(&crit.id).copied().unwrap_or(5.0);
                                            rsx! {
                                                div {
                                                    key: "{crit.id}",
                                                    class: "space-y-1",
                                                    div {
                                                        class: "flex justify-between items-center",
                                                        span { class: "text-xs font-semibold text-kr-text-nucleus flex items-center gap-1.5",
                                                            span { "{crit.emoji}" }
                                                            span { "{lang().tr(&crit.name)}" }
                                                        }
                                                        span { class: "text-xs font-bold text-kr-turquoise tabular-nums", "{score:.1}" }
                                                    }
                                                    input {
                                                        type: "range", min: "0.0", max: "10.0", step: "0.1",
                                                        class: "w-full h-1.5 bg-kr-membrane border border-kr-text-matrix/10 appearance-none cursor-pointer accent-kr-turquoise outline-none",
                                                        value: "{score}",
                                                        oninput: move |e| {
                                                            if let Ok(val) = e.value().parse::<f64>() {
                                                                new_product_scores.write().insert(crit.id.clone(), val);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            // Actions
                            div {
                                class: "flex justify-end gap-3 border-t border-kr-text-matrix/10 pt-4",
                                button {
                                    class: "kr-btn-pill px-4 py-1.5 text-xs transition-all active:scale-95",
                                    onclick: move |_| show_add_product_form.set(false),
                                    "{lang().t(\"Cancel\")}"
                                }
                                button {
                                    disabled: is_saving_product(),
                                    style: "background-color: var(--color-kr-turquoise) !important; color: white !important;",
                                    class: "kr-btn-pill px-5 py-1.5 text-xs transition-all active:scale-95",
                                    onclick: move |_| {
                                        if is_saving_product() {
                                            return;
                                        }

                                        let name_val = new_product_name.read().trim().to_string();
                                        if !name_val.is_empty() {
                                            let price_val = new_product_price.read().parse::<f64>().unwrap_or(0.0);
                                            let qty_val = new_product_quantity.read().parse::<f64>().ok();
                                            let unit_val = { let u = new_product_unit.read(); if u.is_empty() { None } else { Some(u.clone()) } };
                                            let mut final_scores = HashMap::new();
                                            for crit in &active_criteria_for_save {
                                                let score = new_product_scores.read().get(&crit.id).copied().unwrap_or(5.0);
                                                final_scores.insert(crit.id.clone(), score);
                                            }

                                            let payload = NewProductInput {
                                                product_type_id: pt.id.clone(),
                                                name: name_val,
                                                description: new_product_description.read().trim().to_string(),
                                                price: price_val,
                                                quantity: qty_val,
                                                unit: unit_val,
                                                scores: final_scores,
                                            };

                                            is_saving_product.set(true);
                                            add_product_error.set(None);

                                            let mut products = products;
                                            let mut new_product_name = new_product_name;
                                            let mut new_product_price = new_product_price;
                                            let mut new_product_quantity = new_product_quantity;
                                            let mut new_product_unit = new_product_unit;
                                            let mut new_product_description = new_product_description;
                                            let mut new_product_scores = new_product_scores;
                                            let mut show_add_product_form = show_add_product_form;
                                            let mut is_saving_product = is_saving_product;
                                            let mut add_product_error = add_product_error;

                                            spawn(async move {
                                                match create_product(payload).await {
                                                    Ok(new_prod) => {
                                                        products.write().push(new_prod);
                                                        new_product_name.set(String::new());
                                                        new_product_price.set(String::new());
                                                        new_product_quantity.set(String::new());
                                                        new_product_unit.set(String::new());
                                                        new_product_description.set(String::new());
                                                        new_product_scores.write().clear();
                                                        show_add_product_form.set(false);
                                                    }
                                                    Err(err) => {
                                                        add_product_error.set(Some(err.to_string()));
                                                    }
                                                }
                                                is_saving_product.set(false);
                                            });
                                        }
                                    },
                                    if is_saving_product() { "{lang().t(\"Saving...\")}" } else { "{lang().t(\"Save Product\")}" }
                                }
                            }
                            if let Some(error) = save_error {
                                p { class: "text-rose-600 text-xs font-medium", "{error}" }
                            }
                        }
                    }

                    // Product cards
                    if sorted_products.is_empty() {
                        div {
                            class: "py-16 text-center space-y-3",
                            span { class: "text-3xl", "📦" }
                            p { class: "text-kr-text-matrix font-serif italic text-sm",
                                if query_str.is_empty() {
                                    "{lang().t(\"No products yet. Add the first one above.\")}"
                                } else {
                                    "{lang().t(\"No products match your search.\")}"
                                }
                            }
                        }
                    } else {
                        div {
                            class: "space-y-4",
                            for (idx, product) in sorted_products.iter().enumerate() {
                                ProductCard {
                                    key: "{product.id}",
                                    rank: idx + 1,
                                    product: product.clone(),
                                    criteria: active_criteria_for_cards.clone(),
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
