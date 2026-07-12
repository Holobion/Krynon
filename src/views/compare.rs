use crate::model::{load_app_data, Category, ProductType};
use crate::Route;
use dioxus::prelude::*;

// ==========================================
// Compare = Workspace Home (Category Grid)
// ==========================================

#[component]
pub fn Compare() -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();
    let mut search_query = use_context::<Signal<String>>();
    let data = use_resource(move || async move { load_app_data().await });

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
                h1 {
                    class: "text-3xl font-black text-kr-text-nucleus tracking-tight font-display uppercase",
                    "{lang().t(\"Workspace\")}"
                }
                p {
                    class: "text-kr-text-matrix text-sm mt-1 font-serif italic",
                    "{lang().t(\"Select a category to explore product types and begin your comparative analysis.\")}"
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
                        placeholder: "{lang().t(\"Search categories or product types...\")  }",
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
                    let categories = app_data.categories.clone();
                    let product_types = app_data.product_types.clone();

                    // Filter by search if query is active
                    let filtered_categories: Vec<Category> = if query_str.is_empty() {
                        categories.clone()
                    } else {
                        categories.iter().filter(|cat| {
                            cat.name.to_lowercase().contains(&query_str)
                                || cat.description.to_lowercase().contains(&query_str)
                                || product_types.iter()
                                    .filter(|pt| pt.category_ids.contains(&cat.id))
                                    .any(|pt| pt.name.to_lowercase().contains(&query_str))
                        }).cloned().collect()
                    };

                    // Also find product types that match directly but whose category isn't listed
                    let direct_pt_matches: Vec<&ProductType> = if !query_str.is_empty() {
                        product_types.iter().filter(|pt| {
                            (pt.name.to_lowercase().contains(&query_str)
                                || pt.description.to_lowercase().contains(&query_str))
                                && !filtered_categories.iter().any(|cat| pt.category_ids.contains(&cat.id))
                        }).collect()
                    } else {
                        Vec::new()
                    };

                    rsx! {
                        // If search is active and has direct product type matches not covered by categories
                        if !query_str.is_empty() && filtered_categories.is_empty() && direct_pt_matches.is_empty() {
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

                        // Section header
                        div {
                            class: "border-b-1.5 border-kr-text-nucleus pb-3 mb-8 flex justify-between items-end",
                            h2 {
                                class: "text-lg font-bold tracking-widest uppercase font-display",
                                if query_str.is_empty() {
                                    "{lang().t(\"All Categories\")}"
                                } else {
                                    "{lang().t(\"Search Results\")}"
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
                                    // Get product types belonging to this category
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

                        // Direct product type matches (orphan from search)
                        if !direct_pt_matches.is_empty() {
                            div {
                                class: "mt-10",
                                div {
                                    class: "border-b-1.5 border-kr-text-nucleus pb-3 mb-6 flex justify-between items-end",
                                    h2 {
                                        class: "text-lg font-bold tracking-widest uppercase font-display",
                                        "{lang().t(\"Matching Product Types\")}"
                                    }
                                }
                                div {
                                    class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                    for pt in direct_pt_matches.iter() {
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
                                                            span { class: "font-bold text-kr-text-nucleus group-hover:text-kr-turquoise transition-colors", "{pt.name}" }
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
                                                            "{pt.description}"
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
                        "{lang().tr(&category.name)}"
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
                    "{lang().tr(&category.description)}"
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
                                span { class: "font-medium", "{lang().tr(&pt.name)}" }
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
