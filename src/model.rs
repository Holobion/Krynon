use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::{types::Json, PgPool, Row};
use std::collections::HashMap;

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
    pub criteria: Vec<Criterion>, // This will be populated by joining with criteria table
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub emoji: String,
    pub category_ids: Vec<String>, // This will be populated by joining with product_type_categories
    pub specific_criteria: Vec<Criterion>, // This will be populated by joining with product_type_specific_criteria
    pub presets: Vec<WeightProfile>,       // This will need separate handling
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub product_type_id: String,
    pub scores: HashMap<String, f64>, // This will be populated by fetching from product_scores
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeightProfile {
    pub name: String,
    pub weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppData {
    pub categories: Vec<Category>,
    pub product_types: Vec<ProductType>,
    pub products: Vec<Product>,
}

// Helper to fetch criteria for a given category
#[cfg(feature = "server")]
pub async fn fetch_criteria_for_category(
    pool: &PgPool,
    category_id: &str,
) -> Result<Vec<Criterion>, sqlx::Error> {
    let criteria = sqlx::query_as!(
        Criterion,
        r#"
        SELECT c.id, c.name, c.description, c.emoji
        FROM criteria c
        JOIN category_criteria cc ON c.id = cc.criterion_id
        WHERE cc.category_id = $1
        ORDER BY c.name
        "#,
        category_id
    )
    .fetch_all(pool)
    .await?;
    Ok(criteria)
}

// Helper to fetch criteria for a given product type
#[cfg(feature = "server")]
pub async fn fetch_specific_criteria_for_product_type(
    pool: &PgPool,
    product_type_id: &str,
) -> Result<Vec<Criterion>, sqlx::Error> {
    let criteria = sqlx::query_as!(
        Criterion,
        r#"
        SELECT c.id, c.name, c.description, c.emoji
        FROM criteria c
        JOIN product_type_specific_criteria pts ON c.id = pts.criterion_id
        WHERE pts.product_type_id = $1
        ORDER BY c.name
        "#,
        product_type_id
    )
    .fetch_all(pool)
    .await?;
    Ok(criteria)
}

// Fetch all categories, including their associated criteria
#[cfg(feature = "server")]
pub async fn fetch_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    let categories_rows =
        sqlx::query("SELECT id, name, description, emoji FROM categories ORDER BY name")
            .fetch_all(pool)
            .await?;

    let mut categories: Vec<Category> = categories_rows
        .into_iter()
        .map(|row| Category {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            emoji: row.get("emoji"),
            criteria: Vec::new(), // Initialize empty, will be populated below
        })
        .collect();

    // Fetch all criteria once to avoid multiple queries inside the loop if possible,
    // but for category-specific criteria, we still need to query per category.
    // A more optimized approach might fetch all category_criteria mappings first.
    // For now, we rely on fetch_criteria_for_category.
    // let all_criteria = fetch_all_criteria(pool).await?; // This fetches all criteria, not specific to category

    for category in &mut categories {
        // Fetch criteria specifically for this category using the helper
        let category_criteria = fetch_criteria_for_category(pool, &category.id).await?;
        category.criteria = category_criteria;
    }

    Ok(categories)
}

// Fetch all product types, including their associated category IDs and specific criteria
#[cfg(feature = "server")]
pub async fn fetch_all_product_types(pool: &PgPool) -> Result<Vec<ProductType>, sqlx::Error> {
    let product_types_rows =
        sqlx::query("SELECT id, name, description, emoji FROM product_types ORDER BY name")
            .fetch_all(pool)
            .await?;

    let mut product_types: Vec<ProductType> = product_types_rows
        .into_iter()
        .map(|row| ProductType {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            emoji: row.get("emoji"),
            category_ids: Vec::new(),      // Initialize empty
            specific_criteria: Vec::new(), // Initialize empty
            presets: Vec::new(),           // Initialize empty
        })
        .collect();

    for pt in &mut product_types {
        // Fetch associated category IDs
        let category_ids_rows = sqlx::query(
            "SELECT category_id FROM product_type_categories WHERE product_type_id = $1",
        )
        .bind(&pt.id)
        .fetch_all(pool)
        .await?;
        pt.category_ids = category_ids_rows
            .iter()
            .map(|row| row.get("category_id"))
            .collect();

        // Fetch specific criteria and presets for this product type
        pt.specific_criteria = fetch_specific_criteria_for_product_type(pool, &pt.id).await?;
        pt.presets = fetch_weight_profiles_for_product_type(pool, &pt.id).await?;
    }

    Ok(product_types)
}

#[cfg(feature = "server")]
pub async fn fetch_weight_profiles_for_product_type(
    pool: &PgPool,
    product_type_id: &str,
) -> Result<Vec<WeightProfile>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT name, weights FROM weight_profiles WHERE product_type_id = $1 ORDER BY name",
    )
    .bind(product_type_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let weights: Json<HashMap<String, f64>> = row.get("weights");
            WeightProfile {
                name: row.get("name"),
                weights: weights.0,
            }
        })
        .collect())
}

// Fetch all products, including their scores.
#[cfg(feature = "server")]
pub async fn fetch_all_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    let products_rows =
        sqlx::query("SELECT id, name, description, price, quantity, unit, product_type_id FROM products ORDER BY name")
            .fetch_all(pool)
            .await?;

    let mut products = Vec::new();
    for row in products_rows {
        let product_id: String = row.get("id");
        let scores = fetch_scores_for_product(pool, &product_id).await?;
        products.push(Product {
            id: product_id,
            name: row.get("name"),
            description: row.get("description"),
            price: row.get("price"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            product_type_id: row.get("product_type_id"),
            scores,
        });
    }

    Ok(products)
}

// Fetch scores for a specific product
#[cfg(feature = "server")]
pub async fn fetch_scores_for_product(
    pool: &PgPool,
    product_id: &str,
) -> Result<HashMap<String, f64>, sqlx::Error> {
    let scores_rows =
        sqlx::query("SELECT criterion_id, score FROM product_scores WHERE product_id = $1")
            .bind(product_id)
            .fetch_all(pool)
            .await?;

    let mut scores = HashMap::new();
    for row in scores_rows {
        let criterion_id: String = row.get("criterion_id");
        let score: f32 = row.get("score");
        scores.insert(criterion_id, f64::from(score));
    }
    Ok(scores)
}

#[get("/api/app-data")]
pub async fn load_app_data() -> Result<AppData, ServerFnError> {
    let pool = crate::db::server::get_pool().await;

    let categories = fetch_all_categories(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let product_types = fetch_all_product_types(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let products = fetch_all_products(pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(AppData {
        categories,
        product_types,
        products,
    })
}

pub fn calculate_score(product: &Product, weights: &HashMap<String, f64>) -> f64 {
    let mut total_score = 0.0;
    let mut total_weight = 0.0;

    for (criterion_id, score) in &product.scores {
        let weight = weights.get(criterion_id).copied().unwrap_or(5.0);
        total_score += score * weight;
        total_weight += weight;
    }

    if total_weight > 0.0 {
        total_score / total_weight
    } else {
        0.0
    }
}

pub fn get_combined_criteria(
    product_type: &ProductType,
    categories: &[Category],
) -> Vec<Criterion> {
    let mut list = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Add specific criteria for the product type
    for criterion in &product_type.specific_criteria {
        if seen.insert(criterion.id.clone()) {
            list.push(criterion.clone());
        }
    }

    // Add criteria from associated categories
    for category_id in &product_type.category_ids {
        if let Some(category) = categories.iter().find(|c| c.id == *category_id) {
            for criterion in &category.criteria {
                if seen.insert(criterion.id.clone()) {
                    list.push(criterion.clone());
                }
            }
        }
    }

    // Sort criteria by name for consistent ordering
    list.sort_by(|a, b| a.name.cmp(&b.name));
    list
}
