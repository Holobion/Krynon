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
    pub fn t(self, key: &'static str) -> &'static str {
        match self {
            Language::English => translate_en(key),
            Language::French => translate_fr(key),
        }
    }

    pub fn tr(self, text: &str) -> String {
        if self == Language::English {
            return text.to_string();
        }

        match text {
            // Categories
            "Food & Beverage" => "Alimentation & Boisson",
            "Assess consumption items based on resource usage, farming practices, and footprint." => 
                "Évaluez les produits de consommation en fonction de l'utilisation des ressources, des pratiques agricoles et de l'empreinte carbone.",
            "Electronics & Tech" => "Électronique & Tech",
            "Evaluate electronic devices based on ethical supply chains and hardware recyclability." => 
                "Évaluez les appareils électroniques sur la base de chaînes d'approvisionnement éthiques et de la recyclabilité du matériel.",
            "Home & Lifestyle" => "Maison & Style de vie",
            "Focus on ergonomics, material quality, and expected longevity of furniture/decor." => 
                "Concentrez-vous sur l'ergonomie, la qualité des matériaux et la longévité attendue des meubles/décorations.",

            // Product Types
            "Smartphones" => "Smartphones",
            "Compare features like photo output, performance speed, and repair modularity." => 
                "Comparez des fonctionnalités telles que le rendu photo, la vitesse de performance et la modularité des réparations.",
            "Specialty Coffee" => "Café de spécialité",
            "Grade specialty coffee beans on flavor notes, acidity, body, and aroma complexity." => 
                "Évaluez les grains de café de spécialité sur les notes aromatiques, l'acidité, le corps et la complexité des arômes.",
            "Premium Rice" => "Riz de qualité supérieure",
            "Evaluate gourmet rice grains on scent, texture, grain length, and fluffiness." => 
                "Évaluez les grains de riz gastronomiques sur l'arôme, la texture, la longueur du grain et le moelleux.",
            "Office Chairs" => "Chaises de bureau",
            "Grade ergonomic desk seating on posture adjustments and lumbar support quality." => 
                "Évaluez les sièges de bureau ergonomiques sur les ajustements de posture et la qualité du soutien lombaire.",

            // Criteria
            "Carbon Footprint" => "Empreinte carbone",
            "CO2 equivalent emissions per kg produced, including transport/packaging." => 
                "Émissions d'équivalent CO2 par kg produit, y compris le transport/l'emballage.",
            "Sourcing Ethics" => "Éthique d'approvisionnement",
            "Fair-trade guarantees, direct trade links, and organic/eco farming methods." => 
                "Garanties de commerce équitable, liens commerciaux directs et méthodes d'agriculture biologique/écologique.",
            "E-Waste & Recycling" => "Déchets électroniques & Recyclage",
            "Proportion of recycled circular materials and ease of end-of-life recycling." => 
                "Proportion de matériaux circulaires recyclés et facilité de recyclage en fin de vie.",
            "Supply Chain Ethics" => "Éthique de la chaîne d'approvisionnement",
            "Fair mineral sourcing, sweatshop-free labor standards, and supplier audits." => 
                "Approvisionnement équitable en minéraux, normes de travail sans ateliers de misère et audits des fournisseurs.",
            "Longevity & Durability" => "Longévité & Durabilité",
            "Expected lifetime under normal use and warranty coverage." => 
                "Durée de vie attendue dans des conditions normales d'utilisation et couverture de garantie.",
            "Camera Quality" => "Qualité de la caméra",
            "Image details, zoom options, low-light processing, and video stabilization." => 
                "Détails de l'image, options de zoom, traitement en basse lumière et stabilisation vidéo.",
            "Battery & Charging" => "Batterie & Charge",
            "Screen-on time under load and fast-charging capabilities." => 
                "Temps d'écran allumé sous charge et capacités de charge rapide.",
            "Reparability Index" => "Indice de réparabilité",
            "Availability of replacement screens/batteries and ease of disassembly." => 
                "Disponibilité des écrans/batteries de rechange et facilité de démontage.",
            "Performance & Gaming" => "Performance & Jeu",
            "App launching speed, multitasking smoothness, and thermal throttling." => 
                "Vitesse de lancement de l'application, fluidité du multitâche et étranglement thermique.",
            "Fragrance & Aroma" => "Parfum & Arôme",
            "Complexity and intensity of the dry grounds and wet brew scent." => 
                "Complexité et intensité de l'odeur du café moulu et infusé.",
            "Crisp Acidity" => "Acidité vive",
            "Bright fruit notes and clean, sparkling sensation on the tongue." => 
                "Notes de fruits éclatantes et sensation de propreté et pétillante sur la langue.",
            "Mouthfeel & Body" => "Texture & Corps",
            "Texture, weight, and creaminess on the palate." => 
                "Texture, poids et onctuosité en bouche.",
            "Natural Sweetness" => "Douceur naturelle",
            "Caramel, chocolate, or ripe berry sugars without adding sweeteners." => 
                "Sucres de caramel, de chocolat ou de baies mûres sans ajout d'édulcorants.",
            "Texture & Grain" => "Texture & Grain",
            "Softness, fluffiness, and length of grain when cooked correctly." => 
                "Moelleux, légèreté et longueur du grain lorsqu'il est cuit correctement.",
            "Natural Aroma" => "Arôme naturel",
            "Strength of jasmine/pandan floral scents or basmati nutty aromas." => 
                "Intensité des parfums floraux de jasmin/pandan ou des arômes de noisette du basmati.",
            "Lumbar Support" => "Soutien lombaire",
            "Spinal support alignment, posture correction, and mesh breathability." => 
                "Alignement du soutien de la colonne vertébrale, correction de la posture et respirabilité de la maille.",
            "Custom Adjustments" => "Ajustements personnalisés",
            "Armrest, tilt lock, seat depth, and height customization ranges." => 
                "Plages de personnalisation des accoudoirs, du verrouillage de l'inclinaison, de la profondeur du siège et de la hauteur.",

            // Products
            "iPhone 15 Pro" => "iPhone 15 Pro",
            "Premium titanium flagship with class-leading video and processor performance, but locked down design." => 
                "Fleuron en titane haut de gamme avec des performances vidéo et de processeur de premier plan, mais un design fermé.",
            "Galaxy S24 Ultra" => "Galaxy S24 Ultra",
            "Large display, versatile cameras, and styling pen. High performance and price tag." => 
                "Grand écran, caméras polyvalentes et stylet. Haute performance et prix élevé.",
            "Fairphone 5" => "Fairphone 5",
            "Modular, highly sustainable phone designed for self-repair with an industry-best 5-year warranty." => 
                "Téléphone modulaire hautement durable conçu pour l'auto-réparation avec une garantie de 5 ans unique dans l'industrie.",
            "Google Pixel 8a" => "Google Pixel 8a",
            "Incredible price-to-performance value, delivering flagship-grade photos and Google AI features." => 
                "Rapport qualité-prix incroyable, offrant des photos de qualité professionnelle et les fonctionnalités IA de Google.",
            "Ethiopian Yirgacheffe" => "Ethiopian Yirgacheffe",
            "Renowned for its bright citrus acidity, elegant floral aroma, and tea-like light body." => 
                "Renommé pour son acidité vive d'agrumes, son arôme floral élégant et son corps léger semblable à du thé.",
            "Colombian Supremo" => "Colombian Supremo",
            "A classic crowd-pleaser. Extremely balanced with rich caramel sweetness and medium body." => 
                "Un classique apprécié de tous. Extrêmement équilibré avec une riche douceur de caramel et un corps moyen.",
            "Sumatran Mandheling" => "Sumatran Mandheling",
            "Deeply complex, earthy, low acid, and full-bodied with notes of dark chocolate and cedarwood." => 
                "Profondément complexe, terreux, peu acide et corsé avec des notes de chocolat noir et de bois de cèdre.",
            "Supermarket Blend" => "Mélange de supermarché",
            "Generic commercial dark roast, bitter and flat, sourced through industrial farm channels." => 
                "Café torréfié foncé commercial générique, amer et plat, provenant de filières agricoles industrielles.",
            "Premium Jasmine Rice" => "Riz jasmin de qualité supérieure",
            "Fragrant, soft, and slightly sticky, excellent with Asian culinary dishes." => 
                "Parfumé, doux et légèrement collant, excellent avec les plats culinaires asiatiques.",
            "Gourmet Basmati Rice" => "Riz basmati gastronomique",
            "Long, slender, aromatic grain that remains fluffy and separate after cooking." => 
                "Grain long, fin et aromatique qui reste moelleux et séparé après la cuisson.",
            "Bulk White Rice" => "Riz blanc en vrac",
            "Generic commercial white rice, standard processing, high yield, low trace sourcing ethics." => 
                "Riz blanc commercial générique, traitement standard, rendement élevé, faible traçabilité éthique.",
            "Herman Miller Aeron" => "Herman Miller Aeron",
            "The gold standard of ergonomic mesh chairs, built with high recyclability and a 12-year warranty." => 
                "La référence absolue des chaises de bureau ergonomiques en maille, construite avec une haute recyclabilité et une garantie de 12 ans.",
            "Steelcase Gesture" => "Steelcase Gesture",
            "Premium fabric chair designed to support diverse posture styles and continuous movement." => 
                "Chaise en tissu haut de gamme conçue pour soutenir divers styles de posture et un mouvement continu.",
            "Basic Task Chair" => "Chaise de travail de base",
            "Standard plastic office chair with simple height adjust and thin padding, short lifespan." => 
                "Chaise de bureau standard en plastique avec réglage simple de la hauteur et rembourrage mince, courte durée de vie.",

            // Presets
            "Balanced Default" => "Défaut équilibré",
            "Eco & Repair Advocate" => "Défenseur de l'éco-réparation",
            "Power Gamer / Geek" => "Joueur exigeant / Geek",
            "Balanced Filter Roast" => "Torréfaction filtre équilibrée",
            "Bright & Fruity" => "Éclatant & Fruité",
            "Rich & Heavy Espresso" => "Expresso riche & corsé",
            "Balanced Cooking" => "Cuisson équilibrée",
            "Aromatic & Fluffy First" => "Arôme & Moelleux d'abord",
            "Ergonomic Office Worker" => "Travailleur de bureau ergonomique",
            "Minimalist Durable" => "Durable minimaliste",
            "Balanced Benchmark" => "Référence équilibrée",
            "Climate First" => "Climat en priorité",
            "Ethics First" => "Éthique en priorité",

            _ => text,
        }.to_string()
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
        "Search Results" => "Résultats de recherche",
        "results found" => "résultats trouvés",
        "No matches found" => "Aucun résultat trouvé",
        "We couldn't find any category or product type matching \"{search_query}\". Try searching for 'coffee', 'rice', 'tech', or 'office'." =>
            "Nous n'avons trouvé aucune catégorie ou type de produit correspondant à « {search_query} ». Essayez de rechercher 'café', 'riz', 'tech' ou 'office'.",
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
