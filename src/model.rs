use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criterion {
    pub id: String,
    pub name: String,
    pub description: String,
    pub emoji: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
    pub emoji: String,
    pub criteria: Vec<Criterion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub emoji: String,
    pub category_ids: Vec<String>,
    pub specific_criteria: Vec<Criterion>,
    pub presets: Vec<WeightProfile>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub product_type_id: String,
    pub scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightProfile {
    pub name: String,
    pub weights: HashMap<String, f64>,
}

/// Calculate the weighted score for a product based on a map of criteria weights.
/// If a product does not have a score for a criterion, it defaults to 5.0.
/// Weights are assumed to be between 0.0 and 10.0.
pub fn calculate_score(product: &Product, weights: &HashMap<String, f64>) -> f64 {
    let mut total_weight = 0.0;
    let mut weighted_sum = 0.0;

    for (crit_id, score) in &product.scores {
        if let Some(&weight) = weights.get(crit_id) {
            weighted_sum += score * weight;
            total_weight += weight;
        }
    }

    if total_weight > 0.0 {
        weighted_sum / total_weight
    } else {
        0.0
    }
}

/// Computes the union of criteria inherited from all categories of a product type + its specific criteria.
pub fn get_combined_criteria(product_type: &ProductType, categories: &[Category]) -> Vec<Criterion> {
    let mut list = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // First inherit category-level criteria
    for cat_id in &product_type.category_ids {
        if let Some(cat) = categories.iter().find(|c| c.id == *cat_id) {
            for crit in &cat.criteria {
                if seen.insert(crit.id.clone()) {
                    list.push(crit.clone());
                }
            }
        }
    }

    // Then add product-type specific criteria
    for crit in &product_type.specific_criteria {
        if seen.insert(crit.id.clone()) {
            list.push(crit.clone());
        }
    }

    list
}

/// Seed the initial mock categories.
pub fn get_mock_categories() -> Vec<Category> {
    vec![
        Category {
            id: "food".to_string(),
            name: "Food & Beverage".to_string(),
            description: "Assess consumption items based on resource usage, farming practices, and footprint.".to_string(),
            emoji: "🍎".to_string(),
            criteria: vec![
                Criterion {
                    id: "carbon_footprint".to_string(),
                    name: "Carbon Footprint".to_string(),
                    description: "CO2 equivalent emissions per kg produced, including transport/packaging.".to_string(),
                    emoji: "🌍".to_string(),
                },
                Criterion {
                    id: "sourcing_ethics".to_string(),
                    name: "Sourcing Ethics".to_string(),
                    description: "Fair-trade guarantees, direct trade links, and organic/eco farming methods.".to_string(),
                    emoji: "🤝".to_string(),
                },
            ],
        },
        Category {
            id: "technology".to_string(),
            name: "Electronics & Tech".to_string(),
            description: "Evaluate electronic devices based on ethical supply chains and hardware recyclability.".to_string(),
            emoji: "⚡".to_string(),
            criteria: vec![
                Criterion {
                    id: "e_waste".to_string(),
                    name: "E-Waste & Recycling".to_string(),
                    description: "Proportion of recycled circular materials and ease of end-of-life recycling.".to_string(),
                    emoji: "♻️".to_string(),
                },
                Criterion {
                    id: "sourcing_ethics".to_string(),
                    name: "Supply Chain Ethics".to_string(),
                    description: "Fair mineral sourcing, sweatshop-free labor standards, and supplier audits.".to_string(),
                    emoji: "🤝".to_string(),
                },
            ],
        },
        Category {
            id: "furniture".to_string(),
            name: "Home & Lifestyle".to_string(),
            description: "Focus on ergonomics, material quality, and expected longevity of furniture/decor.".to_string(),
            emoji: "🏡".to_string(),
            criteria: vec![
                Criterion {
                    id: "durability".to_string(),
                    name: "Longevity & Durability".to_string(),
                    description: "Expected lifetime under normal use and warranty coverage.".to_string(),
                    emoji: "🛡️".to_string(),
                },
            ],
        },
    ]
}

/// Seed the initial mock product types.
pub fn get_mock_product_types() -> Vec<ProductType> {
    vec![
        ProductType {
            id: "smartphones".to_string(),
            name: "Smartphones".to_string(),
            description: "Compare features like photo output, performance speed, and repair modularity.".to_string(),
            emoji: "📱".to_string(),
            category_ids: vec!["technology".to_string()],
            specific_criteria: vec![
                Criterion {
                    id: "camera".to_string(),
                    name: "Camera Quality".to_string(),
                    description: "Image details, zoom options, low-light processing, and video stabilization.".to_string(),
                    emoji: "📷".to_string(),
                },
                Criterion {
                    id: "battery".to_string(),
                    name: "Battery & Charging".to_string(),
                    description: "Screen-on time under load and fast-charging capabilities.".to_string(),
                    emoji: "🔋".to_string(),
                },
                Criterion {
                    id: "reparability".to_string(),
                    name: "Reparability Index".to_string(),
                    description: "Availability of replacement screens/batteries and ease of disassembly.".to_string(),
                    emoji: "🔧".to_string(),
                },
                Criterion {
                    id: "performance".to_string(),
                    name: "Performance & Gaming".to_string(),
                    description: "App launching speed, multitasking smoothness, and thermal throttling.".to_string(),
                    emoji: "⚡".to_string(),
                },
            ],
            presets: vec![
                WeightProfile {
                    name: "Balanced Default".to_string(),
                    weights: [
                        ("e_waste".to_string(), 5.0),
                        ("sourcing_ethics".to_string(), 5.0),
                        ("camera".to_string(), 5.0),
                        ("battery".to_string(), 5.0),
                        ("reparability".to_string(), 5.0),
                        ("performance".to_string(), 5.0),
                    ].into_iter().collect(),
                },
                WeightProfile {
                    name: "Eco & Repair Advocate".to_string(),
                    weights: [
                        ("e_waste".to_string(), 8.0),
                        ("sourcing_ethics".to_string(), 9.0),
                        ("camera".to_string(), 3.0),
                        ("battery".to_string(), 5.0),
                        ("reparability".to_string(), 10.0),
                        ("performance".to_string(), 3.0),
                    ].into_iter().collect(),
                },
                WeightProfile {
                    name: "Power Gamer / Geek".to_string(),
                    weights: [
                        ("e_waste".to_string(), 2.0),
                        ("sourcing_ethics".to_string(), 3.0),
                        ("camera".to_string(), 7.0),
                        ("battery".to_string(), 8.0),
                        ("reparability".to_string(), 2.0),
                        ("performance".to_string(), 10.0),
                    ].into_iter().collect(),
                },
            ],
        },
        ProductType {
            id: "coffee".to_string(),
            name: "Specialty Coffee".to_string(),
            description: "Grade specialty coffee beans on flavor notes, acidity, body, and aroma complexity.".to_string(),
            emoji: "☕".to_string(),
            category_ids: vec!["food".to_string()],
            specific_criteria: vec![
                Criterion {
                    id: "aroma".to_string(),
                    name: "Fragrance & Aroma".to_string(),
                    description: "Complexity and intensity of the dry grounds and wet brew scent.".to_string(),
                    emoji: "👃".to_string(),
                },
                Criterion {
                    id: "acidity".to_string(),
                    name: "Crisp Acidity".to_string(),
                    description: "Bright fruit notes and clean, sparkling sensation on the tongue.".to_string(),
                    emoji: "🍋".to_string(),
                },
                Criterion {
                    id: "body".to_string(),
                    name: "Mouthfeel & Body".to_string(),
                    description: "Texture, weight, and creaminess on the palate.".to_string(),
                    emoji: "🥛".to_string(),
                },
                Criterion {
                    id: "sweetness".to_string(),
                    name: "Natural Sweetness".to_string(),
                    description: "Caramel, chocolate, or ripe berry sugars without adding sweeteners.".to_string(),
                    emoji: "🍯".to_string(),
                },
            ],
            presets: vec![
                WeightProfile {
                    name: "Balanced Filter Roast".to_string(),
                    weights: [
                        ("carbon_footprint".to_string(), 5.0),
                        ("sourcing_ethics".to_string(), 5.0),
                        ("aroma".to_string(), 6.0),
                        ("acidity".to_string(), 6.0),
                        ("body".to_string(), 4.0),
                        ("sweetness".to_string(), 6.0),
                    ].into_iter().collect(),
                },
                WeightProfile {
                    name: "Bright & Fruity".to_string(),
                    weights: [
                        ("carbon_footprint".to_string(), 4.0),
                        ("sourcing_ethics".to_string(), 6.0),
                        ("aroma".to_string(), 9.0),
                        ("acidity".to_string(), 10.0),
                        ("body".to_string(), 2.0),
                        ("sweetness".to_string(), 8.0),
                    ].into_iter().collect(),
                },
                WeightProfile {
                    name: "Rich & Heavy Espresso".to_string(),
                    weights: [
                        ("carbon_footprint".to_string(), 4.0),
                        ("sourcing_ethics".to_string(), 6.0),
                        ("aroma".to_string(), 8.0),
                        ("acidity".to_string(), 2.0),
                        ("body".to_string(), 10.0),
                        ("sweetness".to_string(), 8.0),
                    ].into_iter().collect(),
                },
            ],
        },
        ProductType {
            id: "rice".to_string(),
            name: "Premium Rice".to_string(),
            description: "Evaluate gourmet rice grains on scent, texture, grain length, and fluffiness.".to_string(),
            emoji: "🌾".to_string(),
            category_ids: vec!["food".to_string()],
            specific_criteria: vec![
                Criterion {
                    id: "texture".to_string(),
                    name: "Texture & Grain".to_string(),
                    description: "Softness, fluffiness, and length of grain when cooked correctly.".to_string(),
                    emoji: "🌾".to_string(),
                },
                Criterion {
                    id: "fragrance".to_string(),
                    name: "Natural Aroma".to_string(),
                    description: "Strength of jasmine/pandan floral scents or basmati nutty aromas.".to_string(),
                    emoji: "🌸".to_string(),
                },
            ],
            presets: vec![
                WeightProfile {
                    name: "Balanced Cooking".to_string(),
                    weights: [
                        ("carbon_footprint".to_string(), 5.0),
                        ("sourcing_ethics".to_string(), 5.0),
                        ("texture".to_string(), 6.0),
                        ("fragrance".to_string(), 6.0),
                    ].into_iter().collect(),
                },
                WeightProfile {
                    name: "Aromatic & Fluffy First".to_string(),
                    weights: [
                        ("carbon_footprint".to_string(), 4.0),
                        ("sourcing_ethics".to_string(), 7.0),
                        ("texture".to_string(), 7.0),
                        ("fragrance".to_string(), 10.0),
                    ].into_iter().collect(),
                },
            ],
        },
        ProductType {
            id: "office_chairs".to_string(),
            name: "Office Chairs".to_string(),
            description: "Grade ergonomic desk seating on posture adjustments and lumbar support quality.".to_string(),
            emoji: "💺".to_string(),
            category_ids: vec!["furniture".to_string()],
            specific_criteria: vec![
                Criterion {
                    id: "ergonomics".to_string(),
                    name: "Lumbar Support".to_string(),
                    description: "Spinal support alignment, posture correction, and mesh breathability.".to_string(),
                    emoji: "💺".to_string(),
                },
                Criterion {
                    id: "adjustability".to_string(),
                    name: "Custom Adjustments".to_string(),
                    description: "Armrest, tilt lock, seat depth, and height customization ranges.".to_string(),
                    emoji: "⚙️".to_string(),
                },
            ],
            presets: vec![
                WeightProfile {
                    name: "Ergonomic Office Worker".to_string(),
                    weights: [
                        ("durability".to_string(), 8.0),
                        ("ergonomics".to_string(), 10.0),
                        ("adjustability".to_string(), 9.0),
                    ].into_iter().collect(),
                },
                WeightProfile {
                    name: "Minimalist Durable".to_string(),
                    weights: [
                        ("durability".to_string(), 10.0),
                        ("ergonomics".to_string(), 6.0),
                        ("adjustability".to_string(), 5.0),
                    ].into_iter().collect(),
                },
            ],
        },
    ]
}

/// Seed the initial mock products.
pub fn get_mock_products() -> Vec<Product> {
    vec![
        // Smartphones
        Product {
            id: "iphone-15-pro".to_string(),
            name: "iPhone 15 Pro".to_string(),
            description: "Premium titanium flagship with class-leading video and processor performance, but locked down design.".to_string(),
            product_type_id: "smartphones".to_string(),
            scores: [
                ("e_waste".to_string(), 4.5),
                ("sourcing_ethics".to_string(), 5.0),
                ("camera".to_string(), 9.4),
                ("battery".to_string(), 8.0),
                ("reparability".to_string(), 4.2),
                ("performance".to_string(), 9.7),
            ].into_iter().collect(),
        },
        Product {
            id: "galaxy-s24-ultra".to_string(),
            name: "Galaxy S24 Ultra".to_string(),
            description: "Large display, versatile cameras, and styling pen. High performance and price tag.".to_string(),
            product_type_id: "smartphones".to_string(),
            scores: [
                ("e_waste".to_string(), 5.0),
                ("sourcing_ethics".to_string(), 4.8),
                ("camera".to_string(), 9.5),
                ("battery".to_string(), 8.8),
                ("reparability".to_string(), 5.0),
                ("performance".to_string(), 9.6),
            ].into_iter().collect(),
        },
        Product {
            id: "fairphone-5".to_string(),
            name: "Fairphone 5".to_string(),
            description: "Modular, highly sustainable phone designed for self-repair with an industry-best 5-year warranty.".to_string(),
            product_type_id: "smartphones".to_string(),
            scores: [
                ("e_waste".to_string(), 9.5),
                ("sourcing_ethics".to_string(), 9.8),
                ("camera".to_string(), 6.5),
                ("battery".to_string(), 7.5),
                ("reparability".to_string(), 10.0),
                ("performance".to_string(), 6.8),
            ].into_iter().collect(),
        },
        Product {
            id: "pixel-8a".to_string(),
            name: "Google Pixel 8a".to_string(),
            description: "Incredible price-to-performance value, delivering flagship-grade photos and Google AI features.".to_string(),
            product_type_id: "smartphones".to_string(),
            scores: [
                ("e_waste".to_string(), 5.5),
                ("sourcing_ethics".to_string(), 5.8),
                ("camera".to_string(), 8.8),
                ("battery".to_string(), 7.8),
                ("reparability".to_string(), 5.5),
                ("performance".to_string(), 8.0),
            ].into_iter().collect(),
        },
        // Specialty Coffee
        Product {
            id: "ethiopian-yirgacheffe".to_string(),
            name: "Ethiopian Yirgacheffe".to_string(),
            description: "Renowned for its bright citrus acidity, elegant floral aroma, and tea-like light body.".to_string(),
            product_type_id: "coffee".to_string(),
            scores: [
                ("carbon_footprint".to_string(), 7.5),
                ("sourcing_ethics".to_string(), 8.2),
                ("aroma".to_string(), 9.6),
                ("acidity".to_string(), 9.2),
                ("body".to_string(), 4.5),
                ("sweetness".to_string(), 8.8),
            ].into_iter().collect(),
        },
        Product {
            id: "colombian-supremo".to_string(),
            name: "Colombian Supremo".to_string(),
            description: "A classic crowd-pleaser. Extremely balanced with rich caramel sweetness and medium body.".to_string(),
            product_type_id: "coffee".to_string(),
            scores: [
                ("carbon_footprint".to_string(), 6.8),
                ("sourcing_ethics".to_string(), 8.5),
                ("aroma".to_string(), 8.2),
                ("acidity".to_string(), 6.5),
                ("body".to_string(), 7.6),
                ("sweetness".to_string(), 8.5),
            ].into_iter().collect(),
        },
        Product {
            id: "sumatran-mandheling".to_string(),
            name: "Sumatran Mandheling".to_string(),
            description: "Deeply complex, earthy, low acid, and full-bodied with notes of dark chocolate and cedarwood.".to_string(),
            product_type_id: "coffee".to_string(),
            scores: [
                ("carbon_footprint".to_string(), 7.2),
                ("sourcing_ethics".to_string(), 7.0),
                ("aroma".to_string(), 7.8),
                ("acidity".to_string(), 3.2),
                ("body".to_string(), 9.5),
                ("sweetness".to_string(), 6.0),
            ].into_iter().collect(),
        },
        Product {
            id: "mass-market-roast".to_string(),
            name: "Supermarket Blend".to_string(),
            description: "Generic commercial dark roast, bitter and flat, sourced through industrial farm channels.".to_string(),
            product_type_id: "coffee".to_string(),
            scores: [
                ("carbon_footprint".to_string(), 3.8),
                ("sourcing_ethics".to_string(), 2.5),
                ("aroma".to_string(), 3.5),
                ("acidity".to_string(), 4.0),
                ("body".to_string(), 5.5),
                ("sweetness".to_string(), 3.0),
            ].into_iter().collect(),
        },
        // Premium Rice
        Product {
            id: "jasmine-rice".to_string(),
            name: "Premium Jasmine Rice".to_string(),
            description: "Fragrant, soft, and slightly sticky, excellent with Asian culinary dishes.".to_string(),
            product_type_id: "rice".to_string(),
            scores: [
                ("carbon_footprint".to_string(), 7.5),
                ("sourcing_ethics".to_string(), 8.0),
                ("texture".to_string(), 9.0),
                ("fragrance".to_string(), 9.5),
            ].into_iter().collect(),
        },
        Product {
            id: "basmati-rice".to_string(),
            name: "Gourmet Basmati Rice".to_string(),
            description: "Long, slender, aromatic grain that remains fluffy and separate after cooking.".to_string(),
            product_type_id: "rice".to_string(),
            scores: [
                ("carbon_footprint".to_string(), 7.0),
                ("sourcing_ethics".to_string(), 7.8),
                ("texture".to_string(), 9.2),
                ("fragrance".to_string(), 9.0),
            ].into_iter().collect(),
        },
        Product {
            id: "cheap-white-rice".to_string(),
            name: "Bulk White Rice".to_string(),
            description: "Generic commercial white rice, standard processing, high yield, low trace sourcing ethics.".to_string(),
            product_type_id: "rice".to_string(),
            scores: [
                ("carbon_footprint".to_string(), 5.0),
                ("sourcing_ethics".to_string(), 3.0),
                ("texture".to_string(), 5.5),
                ("fragrance".to_string(), 3.0),
            ].into_iter().collect(),
        },
        // Office Chairs
        Product {
            id: "herman-miller-aeron".to_string(),
            name: "Herman Miller Aeron".to_string(),
            description: "The gold standard of ergonomic mesh chairs, built with high recyclability and a 12-year warranty.".to_string(),
            product_type_id: "office_chairs".to_string(),
            scores: [
                ("durability".to_string(), 9.8),
                ("ergonomics".to_string(), 9.6),
                ("adjustability".to_string(), 9.2),
            ].into_iter().collect(),
        },
        Product {
            id: "steelcase-gesture".to_string(),
            name: "Steelcase Gesture".to_string(),
            description: "Premium fabric chair designed to support diverse posture styles and continuous movement.".to_string(),
            product_type_id: "office_chairs".to_string(),
            scores: [
                ("durability".to_string(), 9.5),
                ("ergonomics".to_string(), 9.5),
                ("adjustability".to_string(), 9.8),
            ].into_iter().collect(),
        },
        Product {
            id: "budget-mesh-chair".to_string(),
            name: "Basic Task Chair".to_string(),
            description: "Standard plastic office chair with simple height adjust and thin padding, short lifespan.".to_string(),
            product_type_id: "office_chairs".to_string(),
            scores: [
                ("durability".to_string(), 4.0),
                ("ergonomics".to_string(), 5.0),
                ("adjustability".to_string(), 4.5),
            ].into_iter().collect(),
        },
    ]
}
