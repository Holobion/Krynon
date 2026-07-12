use crate::model::{
    create_category, create_product, create_product_type, get_combined_criteria, load_app_data,
    tr_description, tr_name, AppData, Category, NewCategoryInput, NewProductInput,
    NewProductTypeInput, ProductType, TranslationEntry,
};
use crate::Route;
use dioxus::prelude::*;
use std::collections::HashMap;

// ==========================================
// Compare = Workspace Home (Category Grid)
// ==========================================

#[component]
pub fn Compare() -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();
    let default_locale = use_context::<Signal<String>>();
    let mut search_query = use_context::<Signal<String>>();
    let mut show_create_product_modal = use_signal(|| false);
    let mut refresh_token = use_signal(|| 0_u64);
    let refresh_nonce = refresh_token;
    let data = use_resource(move || async move {
        let _ = refresh_nonce();
        load_app_data(lang().as_code().to_string()).await
    });

    // Note: we do NOT reset the shared `search_query` here. The user expects
    // their search to persist while they navigate to a product type, view it,
    // and come back to the workspace. The signal is initialized to an empty
    // string in `App` and only modified by the search input itself.

    let query_str = search_query.read().to_lowercase();

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-6 py-8",

            // Page Header
            div {
                class: "mb-8",
                div {
                    class: "flex items-center gap-3 mb-2",
                    div { class: "w-2 h-2 bg-kr-turquoise border border-kr-text-nucleus" }
                    span {
                        class: "font-mono text-xs uppercase tracking-widest text-kr-text-matrix border-l-2 border-kr-text-nucleus pl-3 py-0.5",
                        "{lang().t(\"Classification Workspace // Category Index\")}"
                    }
                }
                div {
                    class: "flex flex-wrap items-end justify-between gap-4",
                    div {
                        h1 {
                            class: "text-3xl font-black text-kr-text-nucleus tracking-tight font-display uppercase",
                            "{lang().t(\"Workspace\")}"
                        }
                        p {
                            class: "text-kr-text-matrix text-sm mt-1 font-serif italic",
                            "{lang().t(\"Select a category to explore product types and begin your comparative analysis.\")}"
                        }
                    }
                    button {
                        style: "background-color: var(--color-kr-turquoise) !important; color: white !important;",
                        class: "kr-btn-pill px-4 py-2 text-xs transition-all active:scale-95 shadow-sm",
                        onclick: move |_| show_create_product_modal.set(true),
                        "{lang().t(\"+ Add Product\")}"
                    }
                }
            }

            // Search bar
            div {
                class: "mb-8",
                div {
                    class: "relative flex items-center bg-kr-cytoplasm border border-kr-text-nucleus focus-within:border-kr-turquoise px-5 py-4 text-kr-text-matrix focus-within:text-kr-text-nucleus transition-all bg-grid-pattern",
                    svg {
                        class: "w-6 h-6 mr-4 shrink-0 text-kr-text-nucleus",
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
                        class: "bg-transparent border-none outline-none text-base sm:text-lg w-full text-kr-text-nucleus placeholder-kr-text-matrix/60 font-medium",
                        value: "{search_query}",
                        placeholder: "{lang().t(\"Search categories or product types...\")}",
                        oninput: move |e| {
                            search_query.set(e.value());
                        }
                    }
                    if !search_query.read().is_empty() {
                        button {
                            class: "hover:text-kr-turquoise text-kr-text-matrix transition-colors p-1",
                            onclick: move |_| {
                                search_query.set("".to_string());
                            },
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

            match data() {
                None => rsx! {
                    div {
                        class: "flex items-center justify-center py-32",
                        div {
                            class: "text-center space-y-4",
                            div {
                                class: "font-mono text-xs uppercase tracking-widest text-kr-text-matrix animate-pulse",
                                "{lang().t(\"Loading classification data...\")}"
                            }
                        }
                    }
                },
                Some(Err(_)) => rsx! {
                    div {
                        class: "py-20 text-rose-600 font-mono text-sm",
                        "{lang().t(\"Failed to load product data.\")}"
                    }
                },
                Some(Ok(app_data)) => {
                    let active = lang().as_code().to_string();
                    let default_locale_val = default_locale();
                    let categories = app_data.categories.clone();
                    let product_types = app_data.product_types.clone();

                    // Filter by search if query is active
                    let filtered_categories: Vec<Category> = if query_str.is_empty() {
                        categories.clone()
                    } else {
                        categories.iter().filter(|cat| {
                            tr_name(&cat.translations, &active, &default_locale_val)
                                .to_lowercase()
                                .contains(&query_str)
                                || tr_description(&cat.translations, &active, &default_locale_val)
                                    .to_lowercase()
                                    .contains(&query_str)
                        }).cloned().collect()
                    };

                    // Also find product types that match directly
                    let matching_product_types: Vec<ProductType> = if query_str.is_empty() {
                        Vec::new()
                    } else {
                        product_types.iter().filter(|pt| {
                            tr_name(&pt.translations, &active, &default_locale_val)
                                .to_lowercase()
                                .contains(&query_str)
                                || tr_description(&pt.translations, &active, &default_locale_val)
                                    .to_lowercase()
                                    .contains(&query_str)
                        }).cloned().collect()
                    };

                    rsx! {
                        // If search is active and has direct product type matches not covered by categories
                        if !query_str.is_empty() && filtered_categories.is_empty() && matching_product_types.is_empty() {
                            div {
                                class: "flex flex-col items-center justify-center py-20 text-center space-y-4",
                                span { class: "text-4xl", "🔍" }
                                h3 { class: "text-lg font-bold text-kr-text-nucleus font-display", "{lang().t(\"No matches found\")}" }
                                p { class: "text-kr-text-matrix text-sm max-w-sm font-serif italic",
                                    "{lang().t(\"Try a different search term.\")}"
                                }
                                button {
                                    style: "background-color: var(--color-kr-turquoise) !important; color: white !important;",
                                    class: "kr-btn-pill px-5 py-2 text-xs transition-all active:scale-95 mt-2",
                                    onclick: move |_| { search_query.set("".to_string()); },
                                    "{lang().t(\"Clear Search\")}"
                                }
                            }
                        }

                        // Categories section (hidden only if there are no category matches and a search is active)
                        if query_str.is_empty() || !filtered_categories.is_empty() {
                            div {
                                class: "border-b-1.5 border-kr-text-nucleus pb-3 mb-8 flex justify-between items-end",
                                h2 {
                                    class: "text-lg font-bold tracking-widest uppercase font-display",
                                    if query_str.is_empty() {
                                        "{lang().t(\"All Categories\")}"
                                    } else {
                                        "{lang().t(\"Matching Categories\")}"
                                    }
                                }
                                span {
                                    class: "font-mono text-xs text-kr-text-matrix hidden sm:inline",
                                    "{filtered_categories.len()} {lang().t(\"categories\")}"
                                }
                            }

                            // Category cards grid
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                                for cat in filtered_categories.iter() {
                                    {
                                        let cat = cat.clone();
                                        let cat_id = cat.id.clone();
                                        let cat_pts: Vec<ProductType> = product_types.iter()
                                            .filter(|pt| pt.category_ids.contains(&cat.id))
                                            .cloned()
                                            .collect();
                                        let pt_count = cat_pts.len();
                                        rsx! {
                                            CategoryCard {
                                                key: "{cat.id}",
                                                category: cat,
                                                product_types: cat_pts,
                                                pt_count,
                                                target_id: cat_id,
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Matching product types section (only when searching and there are matches)
                        if !query_str.is_empty() && !matching_product_types.is_empty() {
                            div {
                                class: "mt-12",
                                div {
                                    class: "border-b-1.5 border-kr-text-nucleus pb-3 mb-6 flex justify-between items-end",
                                    h2 {
                                        class: "text-lg font-bold tracking-widest uppercase font-display",
                                        "{lang().t(\"Matching Product Types\")}"
                                    }
                                    span {
                                        class: "font-mono text-xs text-kr-text-matrix hidden sm:inline",
                                        "{matching_product_types.len()} {lang().t(\"product types\")}"
                                    }
                                }
                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    for pt in matching_product_types.iter() {
                                        {
                                            let pt = (*pt).clone();
                                            let pt_id = pt.id.clone();
                                            rsx! {
                                                Link {
                                                    key: "{pt.id}",
                                                    to: Route::WorkspacePage { id: pt_id },
                                                    class: "kr-grid-box p-0 overflow-hidden group cursor-pointer hover:border-kr-turquoise transition-all duration-200",
                                                    div {
                                                        class: "kr-header-bar bg-kr-text-nucleus/5 flex justify-between items-center",
                                                        span {
                                                            class: "flex items-center gap-2",
                                                            span { "{pt.emoji}" }
                                                            span { class: "font-bold text-kr-text-nucleus group-hover:text-kr-turquoise transition-colors",
                                                                "{tr_name(&pt.translations, &active, &default_locale_val)}"
                                                            }
                                                        }
                                                        span {
                                                            class: "text-[9px] uppercase font-extrabold px-2 py-0.5 border border-kr-text-nucleus text-kr-text-nucleus",
                                                            "WORKSPACE →"
                                                        }
                                                    }
                                                    div {
                                                        class: "p-4",
                                                        p {
                                                            class: "font-serif italic text-xs text-kr-text-matrix leading-relaxed line-clamp-2",
                                                            "{tr_description(&pt.translations, &active, &default_locale_val)}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if show_create_product_modal() {
                            CreateProductModal {
                                app_data: app_data.clone(),
                                on_close: move |_| show_create_product_modal.set(false),
                                on_saved: move |_| {
                                    show_create_product_modal.set(false);
                                    refresh_token.set(refresh_token() + 1);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

// ==========================================
// Category Card Component
// ==========================================

#[component]
fn CategoryCard(
    category: Category,
    product_types: Vec<ProductType>,
    pt_count: usize,
    target_id: String,
) -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();
    let default_locale = use_context::<Signal<String>>();
    let active = lang().as_code().to_string();
    let default_locale_val = default_locale();

    // Accent color based on category index (cycle through design system colors)
    let accent_colors = [
        "var(--color-kr-turquoise)",
        "var(--color-kr-sage)",
        "var(--color-kr-clay)",
        "var(--color-kr-slate)",
    ];
    // Pick a color deterministically from category id
    let color_idx = category.id.len() % accent_colors.len();
    let accent_color = accent_colors[color_idx];

    rsx! {
        Link {
            to: Route::CategoryPage { id: target_id },
            class: "kr-grid-box p-0 overflow-hidden group cursor-pointer hover:border-kr-turquoise transition-all duration-200 flex flex-col",

            // Card header with accent bar
            div {
                class: "kr-header-bar flex justify-between items-center",
                style: "background-color: {accent_color}20; border-bottom: 1.5px solid {accent_color}40;",
                div {
                    class: "flex items-center gap-2",
                    span { class: "text-base", "{category.emoji}" }
                    span {
                        class: "font-bold text-kr-text-nucleus group-hover:text-kr-turquoise transition-colors uppercase tracking-wide text-xs font-display",
                        "{tr_name(&category.translations, &active, &default_locale_val)}"
                    }
                }
                div {
                    style: "background-color: {accent_color};",
                    class: "w-2.5 h-2.5 border border-kr-text-nucleus shrink-0"
                }
            }

            // Description
            div {
                class: "px-5 pt-4 pb-3",
                p {
                    class: "font-serif italic text-xs text-kr-text-matrix leading-relaxed line-clamp-2",
                    "{tr_description(&category.translations, &active, &default_locale_val)}"
                }
            }

            // Divider
            div { class: "border-t border-kr-text-nucleus/10 mx-5" }

            // Product types vertical list
            div {
                class: "px-5 py-3 flex-grow",
                div {
                    class: "flex justify-between items-center mb-2",
                    span {
                        class: "font-mono text-[10px] uppercase tracking-widest text-kr-text-matrix",
                        "{lang().t(\"Product Types\")}"
                    }
                    span {
                        class: "font-mono text-[10px] text-kr-turquoise font-bold",
                        "{pt_count}"
                    }
                }

                if product_types.is_empty() {
                    p {
                        class: "text-xs text-kr-text-matrix/50 font-serif italic",
                        "{lang().t(\"No product types yet.\")}"
                    }
                } else {
                    ul {
                        class: "space-y-1.5",
                        for pt in product_types.iter().take(5) {
                            li {
                                key: "{pt.id}",
                                class: "flex items-center gap-2 text-xs text-kr-text-matrix group-hover:text-kr-text-nucleus transition-colors",
                                span {
                                    class: "text-[11px] shrink-0",
                                    style: "color: {accent_color};",
                                    "▸"
                                }
                                span { class: "font-medium",
                                    "{tr_name(&pt.translations, &active, &default_locale_val)}"
                                }
                            }
                        }
                        if pt_count > 5 {
                            li {
                                class: "flex items-center gap-2 text-[10px] text-kr-text-matrix/50 font-mono",
                                span { "▸" }
                                span { "+ {pt_count - 5} {lang().t(\"more\")}" }
                            }
                        }
                    }
                }
            }

            // Footer CTA
            div {
                class: "border-t border-kr-text-nucleus/10 px-5 py-3 flex justify-between items-center bg-kr-text-nucleus/3 group-hover:bg-kr-turquoise/5 transition-colors",
                span {
                    class: "font-mono text-[10px] uppercase tracking-widest text-kr-text-matrix group-hover:text-kr-turquoise transition-colors",
                    "{lang().t(\"Explore Category →\")}"
                }
                span {
                    class: "font-mono text-[9px] uppercase text-kr-text-matrix/50",
                    "{category.criteria.len()} {lang().t(\"criteria\")}"
                }
            }
        }
    }
}

// ==========================================
// Create Product Modal
// ==========================================

#[component]
fn CreateProductModal(
    app_data: AppData,
    on_close: EventHandler<()>,
    on_saved: EventHandler<()>,
) -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();
    let default_locale = use_context::<Signal<String>>();

    let active_locale = lang().as_code().to_string();
    let default_locale_val = default_locale();

    let default_product_type_id = app_data
        .product_types
        .first()
        .map(|pt| pt.id.clone())
        .unwrap_or_default();
    let default_category_ids = app_data
        .product_types
        .iter()
        .find(|pt| pt.id == default_product_type_id)
        .map(|pt| pt.category_ids.clone())
        .filter(|ids| !ids.is_empty())
        .or_else(|| app_data.categories.first().map(|cat| vec![cat.id.clone()]))
        .unwrap_or_default();

    let mut selected_category_ids = use_signal(|| default_category_ids.clone());
    let mut selected_product_type_id = use_signal(|| default_product_type_id.clone());
    let mut category_search = use_signal(String::new);
    let mut product_type_search = use_signal(String::new);
    let mut create_new_category = use_signal(|| false);
    let mut create_new_product_type = use_signal(|| false);
    let mut new_category_name = use_signal(|| "".to_string());
    let mut new_category_description = use_signal(|| "".to_string());
    let mut new_category_emoji = use_signal(|| "🧪".to_string());
    let mut new_product_type_name = use_signal(|| "".to_string());
    let mut new_product_type_description = use_signal(|| "".to_string());
    let mut new_product_type_emoji = use_signal(|| "📦".to_string());
    let mut new_product_name = use_signal(|| "".to_string());
    let mut new_product_description = use_signal(|| "".to_string());
    let mut new_product_price = use_signal(|| "".to_string());
    let mut new_product_quantity = use_signal(|| "".to_string());
    let mut new_product_unit = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| Option::<String>::None);

    let close_modal = move |_| on_close.call(());

    let category_query = category_search.read().trim().to_lowercase();
    let product_type_query = product_type_search.read().trim().to_lowercase();

    let filtered_categories: Vec<Category> = if category_query.is_empty() {
        app_data.categories.clone()
    } else {
        app_data
            .categories
            .iter()
            .filter(|category| {
                tr_name(&category.translations, &active_locale, &default_locale_val)
                    .to_lowercase()
                    .contains(&category_query)
                    || tr_description(&category.translations, &active_locale, &default_locale_val)
                        .to_lowercase()
                        .contains(&category_query)
            })
            .cloned()
            .collect()
    };

    let filtered_product_types: Vec<ProductType> = if product_type_query.is_empty() {
        app_data.product_types.clone()
    } else {
        app_data
            .product_types
            .iter()
            .filter(|product_type| {
                tr_name(
                    &product_type.translations,
                    &active_locale,
                    &default_locale_val,
                )
                .to_lowercase()
                .contains(&product_type_query)
                    || tr_description(
                        &product_type.translations,
                        &active_locale,
                        &default_locale_val,
                    )
                    .to_lowercase()
                    .contains(&product_type_query)
            })
            .cloned()
            .collect()
    };

    let selected_product_type = app_data
        .product_types
        .iter()
        .find(|pt| pt.id == selected_product_type_id())
        .cloned();

    // Build a preview ProductType for "create new product type" using translations
    // (we don't use a literal name/description; the form fields are kept separately).
    let active_criteria_product_type = if create_new_product_type() {
        let new_name = new_product_type_name.read().trim().to_string();
        let new_desc = new_product_type_description.read().trim().to_string();
        let mut translations = std::collections::HashMap::new();
        translations.insert(
            active_locale.clone(),
            crate::model::LocalizedText {
                name: new_name,
                description: if new_desc.is_empty() {
                    None
                } else {
                    Some(new_desc)
                },
            },
        );
        ProductType {
            id: "__preview__".to_string(),
            emoji: new_product_type_emoji.read().clone(),
            translations,
            category_ids: selected_category_ids.read().clone(),
            specific_criteria: Vec::new(),
            presets: Vec::new(),
        }
    } else {
        selected_product_type
            .clone()
            .unwrap_or_else(|| ProductType {
                id: "__preview__".to_string(),
                emoji: "📦".to_string(),
                translations: std::collections::HashMap::new(),
                category_ids: Vec::new(),
                specific_criteria: Vec::new(),
                presets: Vec::new(),
            })
    };

    let criteria = get_combined_criteria(&active_criteria_product_type, &app_data.categories);

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-start justify-center bg-black/70 backdrop-blur-sm p-4 overflow-y-auto",
            onclick: close_modal,
            div {
                class: "relative w-full max-w-5xl mt-8 mb-12 bg-kr-cytoplasm border border-kr-text-nucleus shadow-2xl",
                onclick: move |e| e.stop_propagation(),

                div {
                    class: "flex items-center justify-between gap-4 border-b border-kr-text-nucleus/10 px-6 py-4 bg-kr-text-nucleus/5",
                    div {
                        h3 { class: "text-base font-bold text-kr-text-nucleus font-display uppercase tracking-wide", "{lang().t(\"Add Product\")}" }
                        p { class: "text-xs text-kr-text-matrix font-serif italic", "{lang().t(\"Select categories and product type, or create them if needed.\")}" }
                    }
                    button {
                        class: "text-kr-text-matrix hover:text-kr-turquoise transition-colors text-sm px-2 py-1",
                        onclick: close_modal,
                        "✕"
                    }
                }

                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-6 p-6",

                    div {
                        class: "space-y-4",

                        div {
                            class: "border border-kr-text-nucleus/10 p-4 bg-kr-membrane space-y-3",
                            h4 { class: "text-xs font-bold uppercase tracking-widest text-kr-text-matrix", "{lang().t(\"Category\")}" }

                            div {
                                class: "relative flex items-center bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-kr-text-matrix focus-within:border-kr-turquoise transition-all",
                                svg {
                                    class: "w-4 h-4 mr-2 shrink-0 text-kr-text-nucleus",
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
                                    class: "bg-transparent border-none outline-none text-sm w-full text-kr-text-nucleus placeholder-kr-text-matrix/60 font-medium",
                                    placeholder: "{lang().t(\"Search categories...\")}",
                                    value: "{category_search}",
                                    oninput: move |e| category_search.set(e.value()),
                                }
                            }

                            div {
                                class: "max-h-56 overflow-y-auto space-y-2 pr-1",
                                for category in filtered_categories.iter() {
                                    {
                                        let category = category.clone();
                                        let category_id = category.id.clone();
                                        let checked = selected_category_ids.read().contains(&category.id);
                                        rsx! {
                                            label {
                                                key: "{category.id}",
                                                class: "flex items-center gap-3 text-sm text-kr-text-nucleus cursor-pointer px-3 py-2 border border-kr-text-nucleus/10 hover:border-kr-turquoise/50 transition-colors",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: checked,
                                                    onchange: move |_| {
                                                        let mut next = selected_category_ids.read().clone();
                                                        if next.contains(&category_id) {
                                                            next.retain(|id| id != &category_id);
                                                        } else {
                                                            next.push(category_id.clone());
                                                        }
                                                        selected_category_ids.set(next);
                                                    },
                                                }
                                                span { class: "text-lg", "{category.emoji}" }
                                                div {
                                                    class: "min-w-0",
                                                    span { class: "block font-medium",
                                                        "{tr_name(&category.translations, &active_locale, &default_locale_val)}"
                                                    }
                                                    p { class: "text-[11px] text-kr-text-matrix font-serif italic line-clamp-1",
                                                        "{tr_description(&category.translations, &active_locale, &default_locale_val)}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if filtered_categories.is_empty() && !category_query.is_empty() {
                                div {
                                    class: "border-t border-kr-text-nucleus/10 pt-3 space-y-2",
                                    p { class: "text-xs text-kr-text-matrix font-serif italic", "{lang().t(\"No categories found\")}" }
                                    button {
                                        class: "kr-btn-pill px-4 py-2 text-xs transition-all active:scale-95 bg-kr-turquoise text-white",
                                        onclick: move |_| {
                                            create_new_category.set(true);
                                            new_category_name.set(category_search.read().trim().to_string());
                                        },
                                        "{lang().t(\"Create Category\")}"
                                    }
                                }
                            }

                            if create_new_category() {
                                div {
                                    class: "grid grid-cols-1 gap-3 border-t border-kr-text-nucleus/10 pt-3",
                                    input {
                                        class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                        placeholder: "{lang().t(\"Category Name\")}",
                                        value: "{new_category_name}",
                                        oninput: move |e| new_category_name.set(e.value()),
                                    }
                                    input {
                                        class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                        placeholder: "{lang().t(\"Emoji\")}",
                                        value: "{new_category_emoji}",
                                        oninput: move |e| new_category_emoji.set(e.value()),
                                    }
                                    textarea {
                                        class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise min-h-24",
                                        placeholder: "{lang().t(\"Explain what this category evaluates...\")}",
                                        value: "{new_category_description}",
                                        oninput: move |e| new_category_description.set(e.value()),
                                    }
                                }
                            }
                        }

                        div {
                            class: "border border-kr-text-nucleus/10 p-4 bg-kr-membrane space-y-3",
                            h4 { class: "text-xs font-bold uppercase tracking-widest text-kr-text-matrix", "{lang().t(\"Product Type\")}" }

                            div {
                                class: "relative flex items-center bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-kr-text-matrix focus-within:border-kr-turquoise transition-all",
                                svg {
                                    class: "w-4 h-4 mr-2 shrink-0 text-kr-text-nucleus",
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
                                    class: "bg-transparent border-none outline-none text-sm w-full text-kr-text-nucleus placeholder-kr-text-matrix/60 font-medium",
                                    placeholder: "{lang().t(\"Search product types...\")}",
                                    value: "{product_type_search}",
                                    oninput: move |e| product_type_search.set(e.value()),
                                }
                            }

                            div {
                                class: "max-h-56 overflow-y-auto space-y-2 pr-1",
                                for product_type in filtered_product_types.iter() {
                                    {
                                        let product_type = product_type.clone();
                                        let product_type_id = product_type.id.clone();
                                        let active = selected_product_type_id.read().as_str() == product_type_id;
                                        rsx! {
                                            label {
                                                key: "{product_type.id}",
                                                class: "flex items-center gap-3 text-sm text-kr-text-nucleus cursor-pointer px-3 py-2 border border-kr-text-nucleus/10 hover:border-kr-turquoise/50 transition-colors",
                                                input {
                                                    r#type: "radio",
                                                    name: "product_type_choice",
                                                    checked: active,
                                                    onchange: move |_| {
                                                        selected_product_type_id.set(product_type_id.clone());
                                                        create_new_product_type.set(false);
                                                    },
                                                }
                                                span { class: "text-lg", "{product_type.emoji}" }
                                                div {
                                                    class: "min-w-0",
                                                    span { class: "block font-medium",
                                                        "{tr_name(&product_type.translations, &active_locale, &default_locale_val)}"
                                                    }
                                                    p { class: "text-[11px] text-kr-text-matrix font-serif italic line-clamp-1",
                                                        "{tr_description(&product_type.translations, &active_locale, &default_locale_val)}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if filtered_product_types.is_empty() && !product_type_query.is_empty() {
                                div {
                                    class: "border-t border-kr-text-nucleus/10 pt-3 space-y-2",
                                    p { class: "text-xs text-kr-text-matrix font-serif italic", "{lang().t(\"No product types found\")}" }
                                    button {
                                        class: "kr-btn-pill px-4 py-2 text-xs transition-all active:scale-95 bg-kr-turquoise text-white",
                                        onclick: move |_| {
                                            create_new_product_type.set(true);
                                            new_product_type_name.set(product_type_search.read().trim().to_string());
                                        },
                                        "{lang().t(\"Create Product Type\")}"
                                    }
                                }
                            }

                            if create_new_product_type() {
                                div {
                                    class: "grid grid-cols-1 gap-3 border-t border-kr-text-nucleus/10 pt-3",
                                    input {
                                        class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                        placeholder: "{lang().t(\"Product Type Name\")}",
                                        value: "{new_product_type_name}",
                                        oninput: move |e| new_product_type_name.set(e.value()),
                                    }
                                    input {
                                        class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                        placeholder: "{lang().t(\"Emoji\")}",
                                        value: "{new_product_type_emoji}",
                                        oninput: move |e| new_product_type_emoji.set(e.value()),
                                    }
                                    textarea {
                                        class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise min-h-24",
                                        placeholder: "{lang().t(\"Define a new evaluation category.\")}",
                                        value: "{new_product_type_description}",
                                        oninput: move |e| new_product_type_description.set(e.value()),
                                    }
                                }
                            }

                            div {
                                class: "text-[11px] text-kr-text-matrix font-serif italic leading-relaxed",
                                "{criteria.len()} {lang().t(\"criteria\")}"
                            }
                        }
                    }

                    div {
                        class: "space-y-4",
                        div {
                            class: "border border-kr-text-nucleus/10 p-4 bg-kr-membrane space-y-3",
                            h4 { class: "text-xs font-bold uppercase tracking-widest text-kr-text-matrix", "{lang().t(\"Product\")}" }
                            input {
                                class: "w-full bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                placeholder: "{lang().t(\"Product Name\")}",
                                value: "{new_product_name}",
                                oninput: move |e| new_product_name.set(e.value()),
                            }
                            input {
                                class: "w-full bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                placeholder: "{lang().t(\"Price ($)\")}",
                                r#type: "number",
                                step: "0.01",
                                min: "0",
                                value: "{new_product_price}",
                                oninput: move |e| new_product_price.set(e.value()),
                            }
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                                input {
                                    class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                    placeholder: "{lang().t(\"Quantity (optional)\")}",
                                    r#type: "number",
                                    step: "0.01",
                                    min: "0",
                                    value: "{new_product_quantity}",
                                    oninput: move |e| new_product_quantity.set(e.value()),
                                }
                                input {
                                    class: "bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise",
                                    placeholder: "{lang().t(\"Unit (e.g. kg, L)\")}",
                                    value: "{new_product_unit}",
                                    oninput: move |e| new_product_unit.set(e.value()),
                                }
                            }
                            textarea {
                                class: "w-full bg-kr-cytoplasm border border-kr-text-nucleus/10 px-3 py-2 text-sm text-kr-text-nucleus focus:outline-none focus:border-kr-turquoise min-h-28",
                                placeholder: "{lang().t(\"Product Description\")}",
                                value: "{new_product_description}",
                                oninput: move |e| new_product_description.set(e.value()),
                            }
                            p {
                                class: "text-[10px] text-kr-text-matrix font-serif italic",
                                "{lang().t(\"Adding product in\")} {active_locale}"
                            }
                        }

                        if let Some(error) = error_message.read().as_ref() {
                            div {
                                class: "border border-rose-400/40 bg-rose-500/10 text-rose-700 px-4 py-3 text-sm",
                                "{error}"
                            }
                        }

                        div {
                            class: "flex justify-end gap-3 pt-2",
                            button {
                                class: "kr-btn-pill px-4 py-2 text-xs transition-all active:scale-95",
                                onclick: close_modal,
                                "{lang().t(\"Cancel\")}"
                            }
                            button {
                                style: "background-color: var(--color-kr-turquoise) !important; color: white !important;",
                                class: "kr-btn-pill px-5 py-2 text-xs transition-all active:scale-95",
                                onclick: move |_| {
                                    let categories = app_data.categories.clone();
                                    let product_types = app_data.product_types.clone();
                                    let active_locale = active_locale.clone();
                                    async move {
                                        error_message.set(None);

                                        let mut final_category_ids = selected_category_ids.read().clone();

                                        if create_new_category() {
                                            let category_name = new_category_name.read().trim().to_string();
                                            if category_name.is_empty() {
                                                error_message.set(Some(lang().t("Category Name").to_string()));
                                                return;
                                            }

                                            let category_desc = new_category_description.read().trim().to_string();
                                            let translations = vec![TranslationEntry {
                                                locale: active_locale.clone(),
                                                name: category_name,
                                                description: if category_desc.is_empty() {
                                                    None
                                                } else {
                                                    Some(category_desc)
                                                },
                                            }];

                                            let created_category = match create_category(NewCategoryInput {
                                                emoji: new_category_emoji.read().trim().to_string(),
                                                translations,
                                            }).await {
                                                Ok(category) => category,
                                                Err(err) => {
                                                    error_message.set(Some(err.to_string()));
                                                    return;
                                                }
                                            };
                                            final_category_ids.push(created_category.id);
                                        }

                                        let product_type_id = if create_new_product_type() {
                                            let product_type_name = new_product_type_name.read().trim().to_string();
                                            if product_type_name.is_empty() {
                                                error_message.set(Some(lang().t("Product Type Name").to_string()));
                                                return;
                                            }

                                            if final_category_ids.is_empty() {
                                                error_message.set(Some(lang().t("Select at least one category.").to_string()));
                                                return;
                                            }

                                            let pt_desc = new_product_type_description.read().trim().to_string();
                                            let pt_translations = vec![TranslationEntry {
                                                locale: active_locale.clone(),
                                                name: product_type_name,
                                                description: if pt_desc.is_empty() {
                                                    None
                                                } else {
                                                    Some(pt_desc)
                                                },
                                            }];

                                            match create_product_type(NewProductTypeInput {
                                                emoji: new_product_type_emoji.read().trim().to_string(),
                                                translations: pt_translations,
                                                category_ids: final_category_ids.clone(),
                                            }).await {
                                                Ok(product_type) => product_type.id,
                                                Err(err) => {
                                                    error_message.set(Some(err.to_string()));
                                                    return;
                                                }
                                            }
                                        } else {
                                            let id = selected_product_type_id.read().clone();
                                            if id.is_empty() {
                                                error_message.set(Some(lang().t("Select Product Type").to_string()));
                                                return;
                                            }
                                            id
                                        };

                                        // Build a preview ProductType from translations to compute
                                        // combined criteria. We construct a HashMap seeded with the
                                        // active UI language so tr_name/tr_description work.
                                        let criteria_source = if create_new_product_type() {
                                            let new_name = new_product_type_name.read().trim().to_string();
                                            let new_desc = new_product_type_description.read().trim().to_string();
                                            let mut map = std::collections::HashMap::new();
                                            map.insert(
                                                active_locale.clone(),
                                                crate::model::LocalizedText {
                                                    name: new_name,
                                                    description: if new_desc.is_empty() {
                                                        None
                                                    } else {
                                                        Some(new_desc)
                                                    },
                                                },
                                            );
                                            ProductType {
                                                id: "__preview__".to_string(),
                                                emoji: new_product_type_emoji.read().clone(),
                                                translations: map,
                                                category_ids: final_category_ids.clone(),
                                                specific_criteria: Vec::new(),
                                                presets: Vec::new(),
                                            }
                                        } else {
                                            match product_types.iter().find(|pt| pt.id == product_type_id) {
                                                Some(pt) => pt.clone(),
                                                None => {
                                                    error_message.set(Some(lang().t("Select Product Type").to_string()));
                                                    return;
                                                }
                                            }
                                        };

                                        let criteria = get_combined_criteria(&criteria_source, &categories);
                                        let mut scores = HashMap::new();
                                        for criterion in &criteria {
                                            scores.insert(criterion.id.clone(), 5.0);
                                        }

                                        let price = new_product_price.read().trim().parse::<f64>().unwrap_or(0.0);
                                        let quantity = {
                                            let qty = new_product_quantity.read().trim().to_string();
                                            if qty.is_empty() {
                                                None
                                            } else {
                                                qty.parse::<f64>().ok()
                                            }
                                        };
                                        let unit = {
                                            let unit = new_product_unit.read().trim().to_string();
                                            if unit.is_empty() { None } else { Some(unit) }
                                        };

                                        let product_name = new_product_name.read().trim().to_string();
                                        if product_name.is_empty() {
                                            error_message.set(Some(lang().t("Product Name").to_string()));
                                            return;
                                        }

                                        let product_description = new_product_description.read().trim().to_string();
                                        let product_translations = vec![TranslationEntry {
                                            locale: active_locale.clone(),
                                            name: product_name,
                                            description: if product_description.is_empty() {
                                                None
                                            } else {
                                                Some(product_description)
                                            },
                                        }];

                                        match create_product(NewProductInput {
                                            product_type_id,
                                            translations: product_translations,
                                            price,
                                            quantity,
                                            unit,
                                            scores,
                                        }).await {
                                            Ok(_) => on_saved.call(()),
                                            Err(err) => error_message.set(Some(err.to_string())),
                                        }
                                    }
                                },
                                "{lang().t(\"Save Product\")}"
                            }
                        }
                    }
                }
            }
        }
    }
}
