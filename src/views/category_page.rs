use crate::model::{load_app_data, Product, ProductType};
use crate::Route;
use dioxus::prelude::*;

// ==========================================
// Category Page : Product Types with top 3 products
// ==========================================

#[component]
pub fn CategoryPage(id: String) -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();
    let data = use_resource(move || async move { load_app_data().await });
    let cat_id = id.clone();

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-6 py-8",

            match data() {
                None => rsx! {
                    div {
                        class: "flex items-center justify-center py-32",
                        div {
                            class: "font-mono text-xs uppercase tracking-widest text-kr-text-matrix animate-pulse",
                            "{lang().t(\"Loading classification data...\")}"
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
                    let products = app_data.products.clone();

                    let maybe_cat = categories.iter().find(|c| c.id == cat_id).cloned();

                    match maybe_cat {
                        None => rsx! {
                            div {
                                class: "py-20 text-center space-y-4",
                                span { class: "text-4xl", "⚠️" }
                                h2 { class: "text-xl font-bold text-kr-text-nucleus font-display", "{lang().t(\"Category not found\")}" }
                                Link {
                                    to: Route::Compare {},
                                    class: "text-kr-turquoise text-sm hover:underline font-medium",
                                    "{lang().t(\"← Back to Workspace\")}"
                                }
                            }
                        },
                        Some(cat) => {
                            let cat_pts: Vec<ProductType> = product_types.iter()
                                .filter(|pt| pt.category_ids.contains(&cat.id))
                                .cloned()
                                .collect();

                            rsx! {
                                // Breadcrumb
                                div {
                                    class: "flex items-center gap-2 font-mono text-xs uppercase tracking-wider text-kr-text-matrix mb-6",
                                    Link {
                                        to: Route::Compare {},
                                        class: "hover:text-kr-turquoise transition-colors",
                                        "{lang().t(\"Workspace\")}"
                                    }
                                    span { class: "text-kr-text-nucleus/30", "/" }
                                    span { class: "text-kr-text-nucleus font-bold", "{lang().tr(&cat.name)}" }
                                }

                                // Category header
                                div {
                                    class: "border-b-1.5 border-kr-text-nucleus pb-6 mb-10",
                                    div {
                                        class: "flex items-center gap-3 mb-3",
                                        div { class: "w-2 h-2 bg-kr-turquoise border border-kr-text-nucleus shrink-0" }
                                        span {
                                            class: "font-mono text-[10px] uppercase tracking-widest text-kr-text-matrix",
                                            "{lang().t(\"Category // Classification Matrix\")}"
                                        }
                                    }
                                    div {
                                        class: "flex items-start gap-4",
                                        span { class: "text-4xl leading-none mt-1", "{cat.emoji}" }
                                        div {
                                            h1 {
                                                class: "text-3xl font-black text-kr-text-nucleus tracking-tight font-display uppercase leading-tight",
                                                "{lang().tr(&cat.name)}"
                                            }
                                            p {
                                                class: "font-serif italic text-kr-text-matrix mt-2 leading-relaxed max-w-2xl",
                                                "{lang().tr(&cat.description)}"
                                            }
                                        }
                                    }

                                    // Category metadata bar
                                    div {
                                        class: "flex flex-wrap gap-6 mt-5 font-mono text-[10px] uppercase tracking-wider text-kr-text-matrix",
                                        div {
                                            span { class: "block text-kr-text-matrix/60", "{lang().t(\"Criteria\")}" }
                                            span { class: "font-bold text-kr-text-nucleus", "{cat.criteria.len()}" }
                                        }
                                        div {
                                            span { class: "block text-kr-text-matrix/60", "{lang().t(\"Product Types\")}" }
                                            span { class: "font-bold text-kr-turquoise", "{cat_pts.len()}" }
                                        }
                                    }
                                }

                                // Section: Product Types grid
                                div {
                                    class: "flex justify-between items-end mb-6",
                                    h2 {
                                        class: "text-lg font-bold tracking-widest uppercase font-display",
                                        "{lang().t(\"Product Types\")}"
                                    }
                                    span {
                                        class: "font-mono text-xs text-kr-text-matrix hidden sm:inline",
                                        "{cat_pts.len()} {lang().t(\"types\")}"
                                    }
                                }

                                if cat_pts.is_empty() {
                                    div {
                                        class: "py-16 text-center space-y-3",
                                        span { class: "text-3xl", "📦" }
                                        p {
                                            class: "text-kr-text-matrix font-serif italic text-sm",
                                            "{lang().t(\"No product types in this category yet.\")}"
                                        }
                                    }
                                } else {
                                    div {
                                        class: "space-y-6",
                                        for pt in cat_pts.iter() {
                                            {
                                                let pt = pt.clone();
                                                let pt_id = pt.id.clone();
                                                // Top 3 products for this product type
                                                let top_products: Vec<Product> = products.iter()
                                                    .filter(|p| p.product_type_id == pt.id)
                                                    .take(3)
                                                    .cloned()
                                                    .collect();
                                                let total_count = products.iter()
                                                    .filter(|p| p.product_type_id == pt.id)
                                                    .count();

                                                rsx! {
                                                    ProductTypeCard {
                                                        key: "{pt.id}",
                                                        product_type: pt,
                                                        top_products,
                                                        total_count,
                                                        target_id: pt_id,
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
// Product Type Card with top 3 products
// ==========================================

#[component]
fn ProductTypeCard(
    product_type: ProductType,
    top_products: Vec<Product>,
    total_count: usize,
    target_id: String,
) -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();

    rsx! {
        Link {
            to: Route::WorkspacePage { id: target_id },
            class: "kr-grid-box p-0 overflow-hidden group cursor-pointer hover:border-kr-turquoise transition-all duration-200 block",

            // Header
            div {
                class: "kr-header-bar bg-kr-text-nucleus/5 flex justify-between items-center",
                div {
                    class: "flex items-center gap-2",
                    span { class: "text-base", "{product_type.emoji}" }
                    h3 {
                        class: "font-bold text-kr-text-nucleus group-hover:text-kr-turquoise transition-colors uppercase tracking-wide text-sm font-display",
                        "{lang().tr(&product_type.name)}"
                    }
                }
                div {
                    class: "flex items-center gap-3",
                    span {
                        class: "font-mono text-[9px] px-2 py-0.5 border border-kr-text-nucleus/30 text-kr-text-matrix",
                        "{total_count} {lang().t(\"products\")}"
                    }
                    span {
                        class: "font-mono text-[9px] text-kr-turquoise font-bold opacity-0 group-hover:opacity-100 transition-opacity",
                        "{lang().t(\"OPEN →\")}"
                    }
                }
            }

            // Body: description + top 3 products side by side
            div {
                class: "grid grid-cols-1 md:grid-cols-12 divide-y md:divide-y-0 md:divide-x divide-kr-text-nucleus/10",

                // Left: description
                div {
                    class: "md:col-span-4 p-5",
                    p {
                        class: "font-serif italic text-xs text-kr-text-matrix leading-relaxed line-clamp-3",
                        "{lang().tr(&product_type.description)}"
                    }
                    // Specific criteria count
                    div {
                        class: "mt-4 font-mono text-[10px] uppercase tracking-widest text-kr-text-matrix",
                        span { class: "text-kr-text-nucleus font-bold", "{product_type.specific_criteria.len()}" }
                        span { " {lang().t(\"specific criteria\")}" }
                    }
                }

                // Right: top 3 products preview
                div {
                    class: "md:col-span-8 p-5",
                    if top_products.is_empty() {
                        div {
                            class: "flex items-center justify-center h-full py-4",
                            p {
                                class: "text-kr-text-matrix/50 font-serif italic text-xs",
                                "{lang().t(\"No products yet — be the first to add one.\")}"
                            }
                        }
                    } else {
                        div {
                            class: "flex justify-between items-center mb-3",
                            span {
                                class: "font-mono text-[10px] uppercase tracking-widest text-kr-text-matrix",
                                "{lang().t(\"Top Products\")}"
                            }
                            if total_count > 3 {
                                span {
                                    class: "font-mono text-[10px] text-kr-turquoise",
                                    "+ {total_count - 3} {lang().t(\"more\")}"
                                }
                            }
                        }
                        div {
                            class: "grid grid-cols-1 sm:grid-cols-3 gap-3",
                            for (idx, product) in top_products.iter().enumerate() {
                                {
                                    let rank_badge = match idx {
                                        0 => "border-amber-200/60 bg-amber-50",
                                        1 => "border-slate-200/60 bg-slate-50",
                                        2 => "border-orange-200/40 bg-orange-50/50",
                                        _ => "border-kr-text-nucleus/10",
                                    };
                                    let rank_num_class = match idx {
                                        0 => "text-amber-700 font-black",
                                        1 => "text-slate-600 font-bold",
                                        2 => "text-orange-700 font-bold",
                                        _ => "text-kr-text-matrix",
                                    };
                                    let formatted_price = format!("${:.2}", product.price);
                                    rsx! {
                                        div {
                                            key: "{product.id}",
                                            class: "border {rank_badge} p-3 space-y-1.5",
                                            div {
                                                class: "flex items-center justify-between",
                                                span {
                                                    class: "font-mono text-[10px] {rank_num_class}",
                                                    "#{idx + 1}"
                                                }
                                                span {
                                                    class: "font-mono text-[10px] font-bold text-emerald-600",
                                                    "{formatted_price}"
                                                }
                                            }
                                            p {
                                                class: "text-xs font-bold text-kr-text-nucleus line-clamp-1 font-display",
                                                "{lang().tr(&product.name)}"
                                            }
                                            p {
                                                class: "text-[10px] text-kr-text-matrix line-clamp-1 font-serif italic",
                                                "{lang().tr(&product.description)}"
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
