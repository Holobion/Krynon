use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "max-w-4xl mx-auto px-6 py-12 space-y-12",

            // Section 1: The Etymology of Krinein
            div {
                class: "space-y-4",
                div {
                    class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-kr-primary/10 border border-kr-primary/20 text-kr-primary text-xs font-semibold uppercase tracking-wider",
                    span { "🧬 Origin Story" }
                }
                h1 { class: "text-4xl font-black text-kr-nucleus tracking-tight sm:text-5xl font-display", "The Philosophy of Krinein" }
                p {
                    class: "text-kr-nucleus/90 text-base sm:text-lg leading-relaxed",
                    "The name "
                    span { class: "font-black text-kr-primary", "Krynon" }
                    " is derived from the ancient Greek verb "
                    span { class: "italic text-kr-primary font-semibold", "krinein" }
                    " (κρίνειν), meaning "
                    span { class: "font-semibold text-kr-nucleus", "“to separate,” “to sort,” or “to decide.”" }
                    " This verb is also the etymological root of the English word "
                    span { class: "italic font-semibold text-kr-nucleus", "criterion" }
                    "—a standard by which something may be judged or decided."
                }
                p {
                    class: "text-kr-matrix text-sm sm:text-base leading-relaxed",
                    "Krynon is designed to embody this origin. Rather than merging reviews into a single arbitrary score, Krynon helps you separate products into their fundamental components, evaluate them systematically, and make decisions tailored to your exact priorities."
                }
            }

            // Section 2: Why 5-Star Reviews Fail
            div {
                class: "bg-kr-cytoplasm border border-kr-matrix/10 kr-squarcle p-6 sm:p-8 space-y-6 shadow-sm",
                h2 { class: "text-2xl font-bold text-kr-nucleus tracking-tight font-display", "Why Generic 5-Star Reviews Fail" }

                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                    div {
                        class: "space-y-2 border-l-2 border-rose-500/40 pl-4",
                        h3 { class: "text-sm font-bold text-rose-600", "The Problem with Averages" }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "A standard 5-star rating aggregates everything—shipping speed, build quality, price, customer service—into one number. A phone might get 3 stars because the buyer received a damaged box, which tells you nothing about its actual battery life." }
                    }

                    div {
                        class: "space-y-2 border-l-2 border-rose-500/40 pl-4",
                        h3 { class: "text-sm font-bold text-rose-600", "Lack of Personalization" }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "A runner who prioritizes eco-sustainability over cushioning shouldn't be forced to buy a shoe rated 4.9 for cushioning if a carbon-neutral option rated 4.2 is available. Standard reviews assume everyone has identical priorities." }
                    }

                    div {
                        class: "space-y-2 border-l-2 border-kr-primary/40 pl-4",
                        h3 { class: "text-sm font-bold text-kr-primary", "The Krynon Way: Separation & Inheritance" }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "Krynon separates products into specific criteria (e.g. camera quality) and inherits global criteria from parent categories (e.g. CO2 footprint). You assign weights to exactly what matters to you." }
                    }

                    div {
                        class: "space-y-2 border-l-2 border-kr-primary/40 pl-4",
                        h3 { class: "text-sm font-bold text-kr-primary", "Global Benchmarks" }
                        p { class: "text-kr-matrix text-xs leading-relaxed", "Compare items across different product types within the same broad category (e.g. coffee vs rice in 'Food & Beverage') using only the shared, global category benchmarks." }
                    }
                }
            }

            // Section 3: The Mathematical Model
            div {
                class: "space-y-4",
                h2 { class: "text-2xl font-bold text-kr-nucleus tracking-tight font-display", "The Mathematical Formula" }
                p { class: "text-kr-matrix text-sm leading-relaxed", "The sorting engine calculates the final normalized score of a product using a weighted average. Each product has a fixed score between 0.0 and 10.0 for each criterion, and the user provides the weights:" }

                div {
                    class: "bg-kr-cytoplasm border border-kr-matrix/10 p-6 kr-squarcle flex flex-col items-center justify-center text-center shadow-sm",
                    span { class: "text-xs font-bold text-kr-matrix uppercase tracking-widest mb-3", "Weighted Average Formula" }
                    div {
                        class: "font-mono text-sm sm:text-base text-kr-primary bg-kr-membrane px-4 py-3 rounded-xl border border-kr-matrix/10 shadow-xs",
                        "Weighted Score = ∑ (Score_i × Weight_i) / ∑ Weight_i"
                    }
                    span { class: "text-[10px] text-kr-matrix mt-3", "Where i represents each criterion in the active context (inherited category-level + product type specific)." }
                }
            }

            // Section 4: Open Source & Collaboration
            div {
                class: "bg-kr-cytoplasm border border-kr-matrix/10 kr-squarcle p-6 sm:p-8 space-y-4 shadow-sm",
                h2 { class: "text-2xl font-bold text-kr-nucleus tracking-tight font-display", "Open Source & Collaboration" }
                p {
                    class: "text-kr-matrix text-sm leading-relaxed",
                    "Krynon is an open-source project created and maintained by the "
                    a {
                        href: "https://github.com/Holobion",
                        class: "text-kr-primary font-bold hover:underline",
                        "Holobion"
                    }
                    " organization. We believe in collaborative curation, transparent mathematical models, and user data ownership."
                }
                p {
                    class: "text-kr-matrix text-sm leading-relaxed",
                    "Explore our codebase, contribute to the design, or report issues on the official repository hosted under the Holobion organization: "
                    a {
                        href: "https://github.com/Holobion/Krynon",
                        class: "text-kr-primary font-bold hover:underline",
                        "Holobion/Krynon"
                    }
                    "."
                }
            }

            // Section 5: What's Next? (Roadmap Hint)
            div {
                class: "border-t border-kr-matrix/10 pt-8 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-6",
                div {
                    h3 { class: "font-bold text-kr-nucleus text-base font-display", "Interested in how Krynon evolves?" }
                    p { class: "text-kr-matrix text-xs mt-1", "Check our development plan for the PostgreSQL release structures." }
                }
                Link {
                    to: Route::Compare {},
                    class: "kr-btn-pill px-6 py-2.5 text-white text-xs font-bold active:scale-95 shadow-md shadow-kr-primary/10",
                    "Try the Workspace"
                }
            }
        }
    }
}
