use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

impl Language {
    pub fn as_code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::French => "fr",
        }
    }
}

impl Language {
    /// Translate a static UI string key. Use this for hard-coded UI labels
    /// that are not stored in the database.
    pub fn t(self, key: &'static str) -> &'static str {
        match self {
            Language::English => translate_en(key),
            Language::French => translate_fr(key),
        }
    }
}

fn translate_en(key: &'static str) -> &'static str {
    key
}

fn translate_fr(key: &'static str) -> &'static str {
    match key {
        "Concepts" => "Concepts",
        "Archive" => "Archives",
        "Workspace" => "Espace de travail",
        "Philosophy" => "Philosophie",
        "Classification Engine" => "Moteur de classification",
        "© 2026 KRYNON. DEVELOPED BY THE HOLOBION ORGANISATION. ALL SYSTEMS INTEGRATED." =>
            "© 2026 KRYNON. DÉVELOPPÉ PAR L'ORGANISATION HOLOBION. TOUS SYSTÈMES INTÉGRÉS.",
        "GitHub" => "GitHub",
        "Origin Story" => "Histoire d'Origine",
        "The Philosophy of Krinein" => "La philosophie de Krinein",
        "The name " => "Le nom ",
        " is derived from the ancient Greek verb " => " est dérivé du verbe grec ancien ",
        " (κρίνειν), meaning " => " (κρίνειν), signifiant ",
        "“to separate,” “to sort,” or “to decide.”" => "« séparer », « trier » ou « décider ».",
        " This verb is also the etymological root of the English word " => " Ce verbe est également la racine étymologique du mot français ",
        "criterion" => "critère",
        "—a standard by which something may be judged or decided." => " — un standard par lequel quelque chose peut être jugé ou décidé.",
        "Krynon is designed to embody this origin. Rather than merging reviews into a single arbitrary score, Krynon helps you separate products into their fundamental components, evaluate them systematically, and make decisions tailored to your exact priorities." =>
            "Krynon est conçu pour incarner cette origine. Plutôt que de fusionner les avis en une seule note arbitraire, Krynon vous aide à diviser les produits en leurs composants fondamentaux, à les évaluer systématiquement et à prendre des décisions adaptées à vos priorités exactes.",
        "Why Generic 5-Star Reviews Fail" => "Pourquoi les avis génériques à 5 étoiles échouent",
        "The Problem with Averages" => "Le problème des moyennes",
        "A standard 5-star rating aggregates everything—shipping speed, build quality, price, customer service—into one number. A phone might get 3 stars because the buyer received a damaged box, which tells you nothing about its actual battery life." =>
            "Une évaluation standard à 5 étoiles agrège tout (vitesse d'expédition, qualité de fabrication, prix, service client) en un seul chiffre. Un téléphone peut obtenir 3 étoiles parce que l'acheteur a reçu une boîte endommagée, ce qui ne vous dit rien sur son autonomie réelle.",
        "Lack of Personalization" => "Absence de personnalisation",
        "A runner who prioritizes eco-sustainability over cushioning shouldn't be forced to buy a shoe rated 4.9 for cushioning if a carbon-neutral option rated 4.2 is available. Standard reviews assume everyone has identical priorities." =>
            "Un coureur qui privilégie l'éco-durabilité sur l'amorti ne devrait pas être contraint d'acheter une chaussure notée 4,9 pour l'amorti si une option neutre en carbone notée 4,2 est disponible. Les avis standard supposent que tout le monde a des priorités identiques.",
        "The Krynon Way: Separation & Inheritance" => "La méthode Krynon : séparation & héritage",
        "Krynon separates products into specific criteria (e.g. camera quality) and inherits global criteria from parent categories (e.g. CO2 footprint). You assign weights to exactly what matters to you." =>
            "Krynon sépare les produits en critères spécifiques (ex. qualité de l'appareil photo) et hérite des critères mondiaux des catégories parentes (ex. empreinte CO2). Vous attribuez des coefficients à ce qui compte vraiment pour vous.",
        "Global Benchmarks" => "Points de référence mondiaux",
        "Compare items across different product types within the same broad category (e.g. coffee vs rice in 'Food & Beverage') using only the shared, global category benchmarks." =>
            "Comparez des articles de différents types de produits au sein d'une même grande catégorie (ex. café vs riz dans « Alimentation & Boisson ») en utilisant uniquement les points de référence partagés de la catégorie globale.",
        "The Mathematical Formula" => "La formule mathématique",
        "The sorting engine calculates the final normalized score of a product using a weighted average. Each product has a fixed score between 0.0 and 10.0 for each criterion, and the user provides the weights:" =>
            "Le moteur de tri calcule le score normalisé final d'un produit à l'aide d'une moyenne pondérée. Chaque produit a un score fixe entre 0,0 et 10,0 pour chaque critère, et l'utilisateur fournit les coefficients :",
        "Weighted Average Formula" => "Formule de moyenne pondérée",
        "Weighted Score = ∑ (Score_i × Weight_i) / ∑ Weight_i" => "Score pondéré = ∑ (Score_i × Coefficient_i) / ∑ Coefficient_i",
        "Where i represents each criterion in the active context (inherited category-level + product type specific)." =>
            "Où i représente chaque critère dans le contexte actif (au niveau de la catégorie héritée + spécifique au type de produit).",
        "Open Source & Collaboration" => "Open Source & Collaboration",
        "Krynon is an open-source project created and maintained by the " => "Krynon est un projet open-source créé et maintenu par l'organisation ",
        " organization. We believe in collaborative curation, transparent mathematical models, and user data ownership." =>
            ". Nous croyons en une conservation collaborative, des modèles mathématiques transparents et l'appropriation des données par les utilisateurs.",
        "Explore our codebase, contribute to the design, or report issues on the official repository hosted under the Holobion organization: " =>
            "Explorez notre code source, contribuez au design ou signalez des problèmes sur le dépôt officiel hébergé sous l'organisation Holobion : ",
        "Interested in how Krynon evolves?" => "Intéressé par l'évolution de Krynon ?",
        "Check our development plan for the PostgreSQL release structures." => "Consultez notre plan de développement pour les structures de base de données PostgreSQL.",
        "Try the Workspace" => "Essayer l'espace de travail",
        "Krynon Scientific Document // Ref. No. 8023-F" => "Document scientifique Krynon // Réf. N° 8023-F",
        "Separate criteria." => "Séparez les critères.",
        "Decide with clarity." => "Décidez avec clarté.",
        "\"Krynon is designed to embody its etymological root—krinein. By separating products into their fundamental components and evaluating them systematically, it empowers users to make objective, data-driven decisions based on their exact priorities.\"" =>
            "\"Krynon est conçu pour incarner sa racine étymologique — krinein. En séparant les produits en leurs composants fondamentaux et en les évaluant systématiquement, il permet aux utilisateurs de prendre des décisions objectives et fondées sur des données en fonction de leurs priorités exactes.\"",
        "Engine Status" => "État du moteur",
        "Sorting Core Active" => "Noyau de tri actif",
        "Active Matrices" => "Matrices actives",
        "Inherited Tree Loaded" => "Arbre hérité chargé",
        "Calculation Mode" => "Mode de calcul",
        "Weighted Average" => "Moyenne pondérée",
        "FIG 01. THE CRITERION // EVALUATION FACET" => "FIG 01. LE CRITÈRE // FACETTE D'ÉVALUATION",
        "Mathematical separation of independent parameter scores." => "Séparation mathématique des scores de paramètres indépendants.",
        "Taxonomy // Core Concepts" => "Taxonomie // Concepts clés",
        "Concept 01 // Criterion" => "Concept 01 // Critère",
        "The fundamental unit of judgment. Separates items into independent, measurable facets rather than merging everything into a single arbitrary score." =>
            "L'unité fondamentale de jugement. Sépare les articles en facettes mesurables indépendantes plutôt que de tout fusionner en une seule note arbitraire.",
        "Concept 02 // Inheritance" => "Concept 02 // Héritage",
        "Hierarchical structure. Criteria are inherited from parent categories down to specific products, allowing unified comparisons at any abstraction level." =>
            "Structure hiérarchique. Les critères sont hérités des catégories parentes jusqu'aux produits spécifiques, permettant des comparaisons unifiées à n'importe quel niveau d'abstraction.",
        "Concept 03 // Weighting" => "Concept 03 // Pondération",
        "Subjective scaling. Users define the priority of each criterion in real-time, allowing the engine's sorting calculations to align with individual needs." =>
            "Échelle subjective. Les utilisateurs définissent la priorité de chaque critère en temps réel, permettant aux calculs de tri du moteur de s'aligner sur les besoins individuels.",
        "Concept 04 // Consensus" => "Concept 04 // Consensus",
        "The synthesized decision. Evaluates the multi-criteria scores using a weighted average model, outputting the optimal match value." =>
            "La décision synthétisée. Évalue les scores multicritères à l'aide d'un modèle de moyenne pondérée, produisant la valeur de correspondance optimale.",
        "Engine Repositories // Open Source Components" => "Dépôts du moteur // Composants open-source",
        "The core rust-based classification engine. Orchestrates sorting algorithms, computes weighted score normalization, and manages criteria registries." =>
            "Le moteur de classification principal basé sur Rust. Orchestre les algorithmes de tri, calcule la normalisation des scores pondérés et gère les registres de critères.",
        "The frontend client interface. Implements the high-performance Swiss grid layout, interactive weight sliders, and criteria creation forms." =>
            "L'interface client frontend. Implémente la disposition de la grille suisse haute performance, les curseurs de coefficients interactifs et les formulaires de création de critères.",
        "The persistent database layer. Manages product schemas, relational category hierarchies, and criteria definitions using PostgreSQL." =>
            "La couche de base de données persistante. Gère les schémas de produits, les hiérarchies de catégories relationnelles et les définitions de critères à l'aide de PostgreSQL.",
        "Analytical Workspace // Criteria Filter" => "Espace d'analyse // Filtre de critères",
        "Execute Criteria-Based Sorting" => "Exécuter le tri par critères",
        "Initialize the comparison workspace to compute weight matrices, evaluate product fitness scores, and discover the optimal choices." =>
            "Initialisez l'espace de comparaison pour calculer les matrices de coefficients, évaluer les scores de pertinence des produits et découvrir les choix optimaux.",
        "Initialize Workspace →" => "Initialiser l'espace →",
        "Classification Workspace" => "Espace de classification",
        "Inherit global criteria, configure custom weights, and perform deep comparative ranking." =>
            "Héritez des critères mondiaux, configurez des coefficients personnalisés et effectuez un classement comparatif approfondi.",
        "Compare by Product Type" => "Comparer par type de produit",
        "Compare by Category" => "Comparer par catégorie",
        "Search product types or categories (e.g. coffee, technology)..." => "Rechercher des types de produits ou catégories (ex. café, technologie)...",
        "Search product types in this category..." => "Rechercher des types de produits dans cette catégorie...",
        "Search Results" => "Résultats de recherche",
        "results found" => "résultats trouvés",
        "No matches found" => "Aucun résultat trouvé",
        "We couldn't find any category or product type matching \"{search_query}\". Try searching for 'coffee', 'rice', 'tech', or 'office'." =>
            "Nous n'avons trouvé aucune catégorie ou type de produit correspondant à « {search_query} ». Essayez de rechercher 'café', 'riz', 'tech' ou 'office'.",
        "No product types match your search." => "Aucun type de produit ne correspond à votre recherche.",
        "Clear Search" => "Effacer la recherche",
        "Category Benchmark" => "Référence de catégorie",
        "Product Type Workspace" => "Espace type de produit",
        "Go to Benchmark ➔" => "Aller à la référence ➔",
        "Go to Workspace ➔" => "Aller à l'espace ➔",
        "Can't find what you need?" => "Vous ne trouvez pas ce qu'il vous faut ?",
        "Define a custom Category or Product Type with your own criteria to evaluate products." =>
            "Définissez une catégorie ou un type de produit personnalisé avec vos propres critères pour évaluer les produits.",
        "Create Category" => "Créer une catégorie",
        "Create Product Type" => "Créer un type de produit",
        "Create New Category" => "Créer une nouvelle catégorie",
        "Define a new evaluation category." => "Définir une nouvelle catégorie d'évaluation.",
        "Select categories and product type, or create them if needed." =>
            "Sélectionnez des catégories et un type de produit, ou créez-les si nécessaire.",
        "Category" => "Catégorie",
        "Product" => "Produit",
        "Search categories..." => "Rechercher des catégories...",
        "Search product types..." => "Rechercher des types de produit...",
        "No categories found" => "Aucune catégorie trouvée",
        "No product types found" => "Aucun type de produit trouvé",
        "Cancel" => "Annuler",
        "Emoji" => "Émoticône",
        "Category Name" => "Nom de la catégorie",
        "Description" => "Description",
        "Explain what this category evaluates..." => "Expliquez ce que cette catégorie évalue...",
        "Inherit Existing Criteria" => "Hériter des critères existants",
        "Add Custom Criteria" => "Ajouter des critères personnalisés",
        "Remove" => "Supprimer",
        "Criterion Name" => "Nom du critère",
        "Criterion Description" => "Description du critère",
        "e.g., Water Conservation" => "ex. Conservation de l'eau",
        "e.g., Gallons of water saved during production..." => "ex. Litres d'eau économisés pendant la production...",
        "+ Add Custom Criterion" => "+ Ajouter un critère personnalisé",
        "Save Category" => "Enregistrer la catégorie",
        "Create New Product Type" => "Créer un nouveau type de produit",
        "Establish a product type inheriting from categories." => "Établir un type de produit héritant des catégories.",
        "Product Type Name" => "Nom du type de produit",
        "Inherit from Categories" => "Hériter des catégories",
        "Save Product Type" => "Enregistrer le type de produit",
        "Select Product Type" => "Sélectionnez un type de produit",
        "Select at least one category." => "Sélectionnez au moins une catégorie.",
        "Weight Presets" => "Préréglages de coefficients",
        "Reset Weights" => "Réinitialiser",
        "Ranking Analysis" => "Analyse du classement",
        "Displaying sorted match scores based on your custom configuration." =>
            "Affichage des scores de correspondance triés selon votre configuration personnalisée.",
        "Add Product" => "Ajouter un produit",
        "Hide Form" => "Masquer le formulaire",
        "Price" => "Prix",
        "Quantity (optional)" => "Quantité (facultatif)",
        "Unit (e.g. kg, L)" => "Unité (ex. kg, L)",
        "Product Name" => "Nom du produit",
        "Product Description" => "Description du produit",
        "Explain key features of this product..." => "Expliquez les principales caractéristiques de ce produit...",
        "Criterion Scores" => "Notes par critère",
        "Assign values from 0.0 (poor) to 10.0 (excellent) for each evaluation dimension." =>
            "Attribuez des valeurs de 0.0 (médiocre) à 10.0 (excellent) pour chaque dimension d'évaluation.",
        "+ Save Product" => "+ Enregistrer le produit",
        "No products found for this configuration." => "Aucun produit trouvé pour cette configuration.",
        "Create or import products to begin ranking analysis." => "Créez ou importez des produits pour commencer l'analyse du classement.",
        "Ignore" => "Ignorer",
        "Critical" => "Critique",
        "Score" => "Note",
        "Detailed Criteria Breakdown" => "Détails par critère",
        "Market Pricing" => "Prix du marché",
        "Weight factor: x" => "Coefficient : x",
        "Adds +" => "Ajoute +",
        " to score" => " à la note",
        "Loading product data..." => "Chargement des données...",
        "Failed to load product data." => "Échec du chargement des données.",
        "Parent Categories (Inherit Criteria)" => "Catégories parentes (Hériter des critères)",
        "Include Specific Criteria" => "Inclure des critères spécifiques",
        "Add Custom Specific Criteria" => "Ajouter des critères spécifiques personnalisés",
        "e.g., Filtration Efficiency" => "ex. Efficacité de la filtration",
        "e.g., HEPA filter capture rate of fine dust particles..." => "ex. Taux de capture du filtre HEPA pour les poussières fines...",
        "Product Type" => "Type de produit",
        "Global Category" => "Catégorie globale",
        "Quick Weight Presets" => "Préréglages rapides de coefficients",
        "Customize Criteria Weight" => "Personnaliser les coefficients des critères",
        "Reset All to 5.0" => "Tout réinitialiser à 5.0",
        "Analytical Ranking" => "Classement analytique",
        " Items Sorted" => " Éléments Triés",
        "+ Add Product" => "+ Ajouter un produit",
        "Add Product to " => "Ajouter un produit à ",
        "e.g., Fairphone 6" => "ex. Fairphone 6",
        "Price ($)" => "Prix ($)",
        "e.g., 699.00" => "ex. 699.00",
        "Qty" => "Qté",
        "e.g., 0.25" => "ex. 0.25",
        "Unit" => "Unité",
        "None (per piece)" => "Aucune (par pièce)",
        "Kilogram (kg)" => "Kilogramme (kg)",
        "Liter (L)" => "Litre (L)",
        "Meter (m)" => "Mètre (m)",
        "Brief description of the product..." => "Brève description du produit...",
        "Evaluate Criteria Scores (0 - 10)" => "Évaluer les notes des critères (0 - 10)",
        "Save Product" => "Enregistrer le produit",
        _ => key,
    }
}
