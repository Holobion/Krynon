use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use sqlx::{types::Json, PgPool, Row};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewCategoryInput {
    pub name: String,
    pub description: String,
    pub emoji: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewProductTypeInput {
    pub name: String,
    pub description: String,
    pub emoji: String,
    pub category_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewProductInput {
    pub product_type_id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslationInput {
    pub entity_type: String, // 'criterion', 'category', 'product_type', 'product', 'weight_profile'
    pub entity_id: String,
    pub language_code: String,
    pub field_name: String, // 'name' or 'description'
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportedLanguage {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_default: bool,
    pub is_enabled: bool,
}

#[cfg(feature = "server")]
fn unique_id_from_name(name: &str, fallback: &str) -> String {
    let slug = slugify(name);
    let base = if slug.is_empty() {
        fallback.to_string()
    } else {
        slug
    };
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{}-{}", base, timestamp)
}

#[cfg(feature = "server")]
async fn generate_unique_category_id(pool: &PgPool, name: &str) -> Result<String, sqlx::Error> {
    loop {
        let candidate = unique_id_from_name(name, "category");
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1)")
                .bind(&candidate)
                .fetch_one(pool)
                .await?;

        if !exists {
            return Ok(candidate);
        }
    }
}

#[cfg(feature = "server")]
async fn generate_unique_product_type_id(pool: &PgPool, name: &str) -> Result<String, sqlx::Error> {
    loop {
        let candidate = unique_id_from_name(name, "product-type");
        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM product_types WHERE id = $1)")
                .bind(&candidate)
                .fetch_one(pool)
                .await?;

        if !exists {
            return Ok(candidate);
        }
    }
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

// Helper to get translated value, with fallback to base content if translation doesn't exist
#[cfg(feature = "server")]
async fn get_translation(
    pool: &PgPool,
    entity_type: &str,
    entity_id: &str,
    field_name: &str,
    language: &str,
    fallback: &str,
) -> Result<String, sqlx::Error> {
    // Try to get translation for requested language
    if language != "en" {
        let result: Option<String> = sqlx::query_scalar(
            "SELECT value FROM translations WHERE entity_type = $1 AND entity_id = $2 AND language_code = $3 AND field_name = $4"
        )
        .bind(entity_type)
        .bind(entity_id)
        .bind(language)
        .bind(field_name)
        .fetch_optional(pool)
        .await?;

        if let Some(value) = result {
            return Ok(value);
        }
    }
    // Fallback to provided value (usually English)
    Ok(fallback.to_string())
}

// Helper to fetch criteria for a given category with language support
#[cfg(feature = "server")]
pub async fn fetch_criteria_for_category_with_language(
    pool: &PgPool,
    category_id: &str,
    language: &str,
) -> Result<Vec<Criterion>, sqlx::Error> {
    let criteria_rows = sqlx::query(
        r#"
        SELECT c.id, c.name, c.description, c.emoji
        FROM criteria c
        JOIN category_criteria cc ON c.id = cc.criterion_id
        WHERE cc.category_id = $1
        ORDER BY c.name
        "#,
    )
    .bind(category_id)
    .fetch_all(pool)
    .await?;

    let mut criteria = Vec::new();
    for row in criteria_rows {
        let criterion_id: String = row.get("id");
        let name = get_translation(
            pool,
            "criterion",
            &criterion_id,
            "name",
            language,
            &row.get::<String, _>("name"),
        )
        .await?;
        let description = get_translation(
            pool,
            "criterion",
            &criterion_id,
            "description",
            language,
            &row.get::<String, _>("description"),
        )
        .await?;

        criteria.push(Criterion {
            id: criterion_id,
            name,
            description,
            emoji: row.get("emoji"),
        });
    }
    Ok(criteria)
}

// Helper to fetch specific criteria for a product type with language support
#[cfg(feature = "server")]
pub async fn fetch_specific_criteria_for_product_type_with_language(
    pool: &PgPool,
    product_type_id: &str,
    language: &str,
) -> Result<Vec<Criterion>, sqlx::Error> {
    let criteria_rows = sqlx::query(
        r#"
        SELECT c.id, c.name, c.description, c.emoji
        FROM criteria c
        JOIN product_type_specific_criteria pts ON c.id = pts.criterion_id
        WHERE pts.product_type_id = $1
        ORDER BY c.name
        "#,
    )
    .bind(product_type_id)
    .fetch_all(pool)
    .await?;

    let mut criteria = Vec::new();
    for row in criteria_rows {
        let criterion_id: String = row.get("id");
        let name = get_translation(
            pool,
            "criterion",
            &criterion_id,
            "name",
            language,
            &row.get::<String, _>("name"),
        )
        .await?;
        let description = get_translation(
            pool,
            "criterion",
            &criterion_id,
            "description",
            language,
            &row.get::<String, _>("description"),
        )
        .await?;

        criteria.push(Criterion {
            id: criterion_id,
            name,
            description,
            emoji: row.get("emoji"),
        });
    }
    Ok(criteria)
}

// Fetch all categories with translations
#[cfg(feature = "server")]
pub async fn fetch_all_categories_with_language(
    pool: &PgPool,
    language: &str,
) -> Result<Vec<Category>, sqlx::Error> {
    let categories_rows =
        sqlx::query("SELECT id, name, description, emoji FROM categories ORDER BY name")
            .fetch_all(pool)
            .await?;

    let mut categories: Vec<Category> = Vec::new();

    for row in categories_rows {
        let category_id: String = row.get("id");
        let name = get_translation(
            pool,
            "category",
            &category_id,
            "name",
            language,
            &row.get::<String, _>("name"),
        )
        .await?;
        let description = get_translation(
            pool,
            "category",
            &category_id,
            "description",
            language,
            &row.get::<String, _>("description"),
        )
        .await?;

        let category_criteria =
            fetch_criteria_for_category_with_language(pool, &category_id, language).await?;

        categories.push(Category {
            id: category_id,
            name,
            description,
            emoji: row.get("emoji"),
            criteria: category_criteria,
        });
    }

    Ok(categories)
}

// Fetch all product types with translations
#[cfg(feature = "server")]
pub async fn fetch_all_product_types_with_language(
    pool: &PgPool,
    language: &str,
) -> Result<Vec<ProductType>, sqlx::Error> {
    let product_types_rows =
        sqlx::query("SELECT id, name, description, emoji FROM product_types ORDER BY name")
            .fetch_all(pool)
            .await?;

    let mut product_types: Vec<ProductType> = Vec::new();

    for row in product_types_rows {
        let pt_id: String = row.get("id");
        let name = get_translation(
            pool,
            "product_type",
            &pt_id,
            "name",
            language,
            &row.get::<String, _>("name"),
        )
        .await?;
        let description = get_translation(
            pool,
            "product_type",
            &pt_id,
            "description",
            language,
            &row.get::<String, _>("description"),
        )
        .await?;

        // Fetch associated category IDs
        let category_ids_rows = sqlx::query(
            "SELECT category_id FROM product_type_categories WHERE product_type_id = $1",
        )
        .bind(&pt_id)
        .fetch_all(pool)
        .await?;
        let category_ids: Vec<String> = category_ids_rows
            .iter()
            .map(|row| row.get("category_id"))
            .collect();

        // Fetch specific criteria
        let specific_criteria =
            fetch_specific_criteria_for_product_type_with_language(pool, &pt_id, language).await?;

        // Fetch weight profiles with translations
        let presets =
            fetch_weight_profiles_for_product_type_with_language(pool, &pt_id, language).await?;

        product_types.push(ProductType {
            id: pt_id,
            name,
            description,
            emoji: row.get("emoji"),
            category_ids,
            specific_criteria,
            presets,
        });
    }

    Ok(product_types)
}

#[cfg(feature = "server")]
pub async fn fetch_weight_profiles_for_product_type_with_language(
    pool: &PgPool,
    product_type_id: &str,
    language: &str,
) -> Result<Vec<WeightProfile>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, name, weights FROM weight_profiles WHERE product_type_id = $1 ORDER BY name",
    )
    .bind(product_type_id)
    .fetch_all(pool)
    .await?;

    let mut profiles = Vec::new();
    for row in rows {
        let profile_id: String = row.get("id");
        let name = get_translation(
            pool,
            "weight_profile",
            &profile_id,
            "name",
            language,
            &row.get::<String, _>("name"),
        )
        .await?;
        let weights: Json<HashMap<String, f64>> = row.get("weights");

        profiles.push(WeightProfile {
            name,
            weights: weights.0,
        });
    }
    Ok(profiles)
}

// Fetch all products with translations
#[cfg(feature = "server")]
pub async fn fetch_all_products_with_language(
    pool: &PgPool,
    language: &str,
) -> Result<Vec<Product>, sqlx::Error> {
    let products_rows =
        sqlx::query("SELECT id, name, description, price, quantity, unit, product_type_id FROM products ORDER BY name")
            .fetch_all(pool)
            .await?;

    let mut products = Vec::new();
    for row in products_rows {
        let product_id: String = row.get("id");
        let name = get_translation(
            pool,
            "product",
            &product_id,
            "name",
            language,
            &row.get::<String, _>("name"),
        )
        .await?;
        let description = get_translation(
            pool,
            "product",
            &product_id,
            "description",
            language,
            &row.get::<String, _>("description"),
        )
        .await?;
        let scores = fetch_scores_for_product(pool, &product_id).await?;

        products.push(Product {
            id: product_id,
            name,
            description,
            price: row.get("price"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            product_type_id: row.get("product_type_id"),
            scores,
        });
    }

    Ok(products)
}

#[get("/api/app-data/{language}")]
pub async fn load_app_data(language: String) -> Result<AppData, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let lang = if language.is_empty() { "en" } else { &language };

    let categories = fetch_all_categories_with_language(pool, lang)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let product_types = fetch_all_product_types_with_language(pool, lang)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    let products = fetch_all_products_with_language(pool, lang)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(AppData {
        categories,
        product_types,
        products,
    })
}

#[cfg(feature = "server")]
fn slugify(value: &str) -> String {
    let slug = value
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>();

    slug.trim_matches('-').to_string()
}

#[post("/api/categories")]
pub async fn create_category(input: NewCategoryInput) -> Result<Category, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let category_id = generate_unique_category_id(pool, &input.name)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    sqlx::query(
        r#"
        INSERT INTO categories (id, name, description, emoji)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&category_id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.emoji)
    .execute(&mut *tx)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    tx.commit()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(Category {
        id: category_id,
        name: input.name,
        description: input.description,
        emoji: input.emoji,
        criteria: Vec::new(),
    })
}

#[post("/api/product-types")]
pub async fn create_product_type(input: NewProductTypeInput) -> Result<ProductType, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let product_type_id = generate_unique_product_type_id(pool, &input.name)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    sqlx::query(
        r#"
        INSERT INTO product_types (id, name, description, emoji)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&product_type_id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.emoji)
    .execute(&mut *tx)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    for category_id in &input.category_ids {
        sqlx::query(
            r#"
            INSERT INTO product_type_categories (product_type_id, category_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(&product_type_id)
        .bind(category_id)
        .execute(&mut *tx)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(ProductType {
        id: product_type_id,
        name: input.name,
        description: input.description,
        emoji: input.emoji,
        category_ids: input.category_ids,
        specific_criteria: Vec::new(),
        presets: Vec::new(),
    })
}

#[cfg(feature = "server")]
async fn generate_unique_product_id(pool: &PgPool, name: &str) -> Result<String, sqlx::Error> {
    let base_slug = slugify(name);
    let base = if base_slug.is_empty() {
        "product".to_string()
    } else {
        base_slug
    };
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    let mut counter: u32 = 0;
    loop {
        let candidate = if counter == 0 {
            format!("{}-{}", base, timestamp)
        } else {
            format!("{}-{}-{}", base, timestamp, counter)
        };

        let exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM products WHERE id = $1)")
                .bind(&candidate)
                .fetch_one(pool)
                .await?;

        if !exists {
            return Ok(candidate);
        }

        counter = counter.saturating_add(1);
    }
}

#[post("/api/products")]
pub async fn create_product(input: NewProductInput) -> Result<Product, ServerFnError> {
    let pool = crate::db::server::get_pool().await;
    let product_id = generate_unique_product_id(pool, &input.name)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    sqlx::query(
        r#"
        INSERT INTO products (id, name, description, price, quantity, unit, product_type_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&product_id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(input.price)
    .bind(input.quantity)
    .bind(input.unit.as_deref())
    .bind(&input.product_type_id)
    .execute(&mut *tx)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    for (criterion_id, score) in &input.scores {
        sqlx::query(
            r#"
            INSERT INTO product_scores (product_id, criterion_id, score)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&product_id)
        .bind(criterion_id)
        .bind(*score as f32)
        .execute(&mut *tx)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(Product {
        id: product_id,
        name: input.name,
        description: input.description,
        price: input.price,
        quantity: input.quantity,
        unit: input.unit,
        product_type_id: input.product_type_id,
        scores: input.scores,
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

// Server function to add or update a translation
#[cfg(feature = "server")]
async fn upsert_translation(
    pool: &PgPool,
    entity_type: &str,
    entity_id: &str,
    language_code: &str,
    field_name: &str,
    value: &str,
) -> Result<(), sqlx::Error> {
    let trans_id = format!(
        "trans-{}-{}-{}-{}",
        entity_id,
        language_code,
        field_name,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    );

    sqlx::query(
        r#"
        INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (entity_type, entity_id, language_code, field_name) 
        DO UPDATE SET value = EXCLUDED.value, updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(trans_id)
    .bind(entity_type)
    .bind(entity_id)
    .bind(language_code)
    .bind(field_name)
    .bind(value)
    .execute(pool)
    .await?;

    Ok(())
}

#[post("/api/translations")]
pub async fn save_translation(input: TranslationInput) -> Result<(), ServerFnError> {
    let pool = crate::db::server::get_pool().await;

    upsert_translation(
        pool,
        &input.entity_type,
        &input.entity_id,
        &input.language_code,
        &input.field_name,
        &input.value,
    )
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(())
}

#[get("/api/supported-languages")]
pub async fn get_supported_languages() -> Result<Vec<SupportedLanguage>, ServerFnError> {
    let pool = crate::db::server::get_pool().await;

    let rows = sqlx::query(
        "SELECT code, name, native_name, is_default, is_enabled FROM supported_languages ORDER BY is_default DESC, code",
    )
    .fetch_all(pool)
    .await
    .map_err(|err| ServerFnError::new(err.to_string()))?;

    Ok(rows
        .into_iter()
        .map(|row| SupportedLanguage {
            code: row.get("code"),
            name: row.get("name"),
            native_name: row.get("native_name"),
            is_default: row.get("is_default"),
            is_enabled: row.get("is_enabled"),
        })
        .collect())
}
