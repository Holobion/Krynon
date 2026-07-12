use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let lang = use_context::<Signal<crate::i18n::Language>>();

    rsx! {
        div {
            class: "max-w-4xl mx-auto px-6 py-12 space-y-12",

            // Section 1: The Etymology of Krinein
            div {
                class: "space-y-4",
                div {
                    class: "inline-flex items-center gap-2 px-3 py-1 rounded-none bg-kr-turquoise/15 border border-kr-turquoise/40 text-kr-nucleus text-xs font-bold uppercase tracking-wider",
                    span { "🧬 {lang().t(\"Origin Story\")}" }
                }
                h1 { class: "text-4xl font-black text-kr-nucleus tracking-tight sm:text-5xl font-display", "{lang().t(\"The Philosophy of Krinein\")}" }
                p {
                    class: "text-kr-nucleus/90 text-base sm:text-lg leading-relaxed",
                    "{lang().t(\"The name \")}"
                    span { class: "font-black border-b-2 border-kr-turquoise text-kr-nucleus pb-0.5", "Krynon" }
                    "{lang().t(\" is derived from the ancient Greek verb \")}"
                    span { class: "italic font-semibold border-b-2 border-kr-turquoise text-kr-nucleus pb-0.5", "krinein" }
                    "{lang().t(\" (κρίνειν), meaning \")}"
                    span { class: "font-semibold text-kr-nucleus", "{lang().t(\"“to separate,” “to sort,” or “to decide.”\")}" }
                    "{lang().t(\" This verb is also the etymological root of the English word \")}"
                    span { class: "italic font-semibold text-kr-nucleus", "{lang().t(\"criterion\")}" }
                    "{lang().t(\"—a standard by which something may be judged or decided.\")}"
                }
                p {
                    class: "text-kr-matrix text-sm sm:text-base leading-relaxed",
                    "{lang().t(\"Krynon is designed to embody this origin. Rather than merging reviews into a single arbitrary score, Krynon helps you separate products into their fundamental components, evaluate them systematically, and make decisions tailored to your exact priorities.\")}"
                }
            }

            // Section 2: Why 5-Star Reviews Fail
            div {
                class: "bg-kr-cytoplasm border border-kr-matrix/10 kr-squarcle p-6 sm:p-8 space-y-6 shadow-sm bg-dot-pattern-dense",
                h2 { class: "text-2xl font-bold text-kr-nucleus tracking-tight font-display", "{lang().t(\"Why Generic 5-Star Reviews Fail\")}" }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                    div {
                        class: "space-y-2 border-l-2 border-kr-clay pl-4",
                        h3 {
                            class: "text-sm font-bold text-kr-nucleus flex items-center gap-2",
                            div { class: "w-2 h-2 bg-kr-clay border border-kr-nucleus" }
                            "{lang().t(\"The Problem with Averages\")}"
                        }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "{lang().t(\"A standard 5-star rating aggregates everything—shipping speed, build quality, price, customer service—into one number. A phone might get 3 stars because the buyer received a damaged box, which tells you nothing about its actual battery life.\")}" }
                    }

                    div {
                        class: "space-y-2 border-l-2 border-kr-clay pl-4",
                        h3 {
                            class: "text-sm font-bold text-kr-nucleus flex items-center gap-2",
                            div { class: "w-2 h-2 bg-kr-clay border border-kr-nucleus" }
                            "{lang().t(\"Lack of Personalization\")}"
                        }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "{lang().t(\"A runner who prioritizes eco-sustainability over cushioning shouldn't be forced to buy a shoe rated 4.9 for cushioning if a carbon-neutral option rated 4.2 is available. Standard reviews assume everyone has identical priorities.\")}" }
                    }

                    div {
                        class: "space-y-2 border-l-2 border-kr-turquoise pl-4",
                        h3 {
                            class: "text-sm font-bold text-kr-nucleus flex items-center gap-2",
                            div { class: "w-2 h-2 bg-kr-turquoise border border-kr-nucleus" }
                            "The Krynon Way: Separation & Inheritance"
                        }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "{lang().t(\"Krynon separates products into specific criteria (e.g. camera quality) and inherits global criteria from parent categories (e.g. CO2 footprint). You assign weights to exactly what matters to you.\")}" }
                    }

                    div {
                        class: "space-y-2 border-l-2 border-kr-turquoise pl-4",
                        h3 {
                            class: "text-sm font-bold text-kr-nucleus flex items-center gap-2",
                            div { class: "w-2 h-2 bg-kr-turquoise border border-kr-nucleus" }
                            "{lang().t(\"Global Benchmarks\")}"
                        }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "{lang().t(\"Compare items across different product types within the same broad category (e.g. coffee vs rice in 'Food & Beverage') using only the shared, global category benchmarks.\")}" }
                    }
                }
            }

            // Section 3: The Mathematical Model
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-bold text-kr-nucleus tracking-tight font-display", "{lang().t(\"The Mathematical Formula\")}" }
                {
                    let formula_desc = lang().t("The sorting engine calculates the final normalized score of a product using a weighted average. Each product has a fixed score between 0.0 and 10.0 for each criterion, and the user provides the weights:");
                    rsx! {
                        p { class: "text-kr-matrix text-sm leading-relaxed", "{formula_desc}" }
                    }
                }

                div {
                    class: "bg-kr-cytoplasm border border-kr-matrix/10 p-6 kr-squarcle flex flex-col items-center justify-center text-center shadow-sm bg-grid-pattern",
                    span { class: "text-xs font-bold text-kr-matrix uppercase tracking-widest mb-3", "{lang().t(\"Weighted Average Formula\")}" }
                    div {
                        class: "font-mono text-sm sm:text-base text-kr-nucleus bg-kr-turquoise/15 px-4 py-3 rounded-none border border-kr-turquoise/35 shadow-xs",
                        "{lang().t(\"Weighted Score = ∑ (Score_i × Weight_i) / ∑ Weight_i\")}"
                    }
                    span { class: "text-[10px] text-kr-matrix mt-3", "{lang().t(\"Where i represents each criterion in the active context (inherited category-level + product type specific).\")}" }
                }
            }

            // Section 4: Open Source & Collaboration
            div {
                class: "bg-kr-cytoplasm border border-kr-matrix/10 kr-squarcle p-6 sm:p-8 space-y-4 shadow-sm bg-dot-pattern",
                h2 { class: "text-2xl font-bold text-kr-nucleus tracking-tight font-display", "{lang().t(\"Open Source & Collaboration\")}" }
                p {
                    class: "text-kr-matrix text-sm leading-relaxed",
                    "{lang().t(\"Krynon is an open-source project created and maintained by the \")}"
                    a {
                        href: "https://github.com/Holobion",
                        class: "text-kr-nucleus font-bold border-b border-kr-turquoise hover:bg-kr-turquoise/10 transition-colors px-0.5",
                        "Holobion"
                    }
                    "{lang().t(\" organization. We believe in collaborative curation, transparent mathematical models, and user data ownership.\")}"
                }
                {
                    let repo_desc = lang().t("Explore our codebase, contribute to the design, or report issues on the official repository hosted under the Holobion organization: ");
                    rsx! {
                        p {
                            class: "text-kr-matrix text-sm leading-relaxed",
                            "{repo_desc}"
                            a {
                                href: "https://github.com/Holobion/Krynon",
                                class: "text-kr-nucleus font-bold border-b border-kr-turquoise hover:bg-kr-turquoise/10 transition-colors px-0.5",
                                "Holobion/Krynon"
                            }
                            "."
                        }
                    }
                }
            }

            // Section 5: What's Next? (Roadmap Hint)
            div {
                class: "border-t border-kr-matrix/10 pt-8 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-6",
                div {
                    h3 { class: "font-bold text-kr-nucleus text-base font-display", "{lang().t(\"Interested in how Krynon evolves?\")}" }
                    p { class: "text-kr-matrix text-xs mt-1", "{lang().t(\"Check our development plan for the PostgreSQL release structures.\")}" }
                }
                Link {
                    to: Route::Compare {},
                    style: "background-color: var(--color-kr-turquoise) !important; color: white !important;",
                    class: "kr-btn-pill px-6 py-2.5 text-xs transition-all active:scale-95 shadow-md shadow-kr-turquoise/10",
                    "{lang().t(\"Try the Workspace\")}"
                }
            }
        }
    }
}
