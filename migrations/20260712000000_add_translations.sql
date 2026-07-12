-- Add language column to base tables to track the default language
ALTER TABLE criteria ADD COLUMN language_code TEXT DEFAULT 'en' NOT NULL;
ALTER TABLE categories ADD COLUMN language_code TEXT DEFAULT 'en' NOT NULL;
ALTER TABLE product_types ADD COLUMN language_code TEXT DEFAULT 'en' NOT NULL;
ALTER TABLE products ADD COLUMN language_code TEXT DEFAULT 'en' NOT NULL;

-- Create the translations table to store all language-specific content
CREATE TABLE IF NOT EXISTS translations (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,  -- 'criterion', 'category', 'product_type', 'product', 'weight_profile'
    entity_id TEXT NOT NULL,
    language_code TEXT NOT NULL,  -- 'en', 'fr', 'es', 'de', 'ja', etc.
    field_name TEXT NOT NULL,     -- 'name', 'description'
    value TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(entity_type, entity_id, language_code, field_name),
    CHECK (entity_type IN ('criterion', 'category', 'product_type', 'product', 'weight_profile'))
);

CREATE INDEX idx_translations_entity ON translations(entity_type, entity_id, language_code);
CREATE INDEX idx_translations_language ON translations(language_code);

-- Create a supported languages reference table
CREATE TABLE IF NOT EXISTS supported_languages (
    code TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    native_name TEXT NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    is_enabled BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert default supported languages
INSERT INTO supported_languages (code, name, native_name, is_default, is_enabled) VALUES
    ('en', 'English', 'English', TRUE, TRUE),
    ('fr', 'French', 'Français', FALSE, TRUE),
    ('es', 'Spanish', 'Español', FALSE, FALSE),
    ('de', 'German', 'Deutsch', FALSE, FALSE),
    ('it', 'Italian', 'Italiano', FALSE, FALSE),
    ('pt', 'Portuguese', 'Português', FALSE, FALSE),
    ('ja', 'Japanese', '日本語', FALSE, FALSE),
    ('zh', 'Chinese', '中文', FALSE, FALSE)
ON CONFLICT (code) DO NOTHING;

-- Backfill translations table with existing English content from criteria
INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || c.id || '-en-name-' || gen_random_uuid()::text,
    'criterion',
    c.id,
    'en',
    'name',
    c.name
FROM criteria c
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || c.id || '-en-desc-' || gen_random_uuid()::text,
    'criterion',
    c.id,
    'en',
    'description',
    c.description
FROM criteria c
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

-- Backfill translations table with existing English content from categories
INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || c.id || '-en-name-' || gen_random_uuid()::text,
    'category',
    c.id,
    'en',
    'name',
    c.name
FROM categories c
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || c.id || '-en-desc-' || gen_random_uuid()::text,
    'category',
    c.id,
    'en',
    'description',
    c.description
FROM categories c
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

-- Backfill translations table with existing English content from product_types
INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || p.id || '-en-name-' || gen_random_uuid()::text,
    'product_type',
    p.id,
    'en',
    'name',
    p.name
FROM product_types p
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || p.id || '-en-desc-' || gen_random_uuid()::text,
    'product_type',
    p.id,
    'en',
    'description',
    p.description
FROM product_types p
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

-- Backfill translations table with existing English content from products
INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || p.id || '-en-name-' || gen_random_uuid()::text,
    'product',
    p.id,
    'en',
    'name',
    p.name
FROM products p
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || p.id || '-en-desc-' || gen_random_uuid()::text,
    'product',
    p.id,
    'en',
    'description',
    p.description
FROM products p
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

-- Backfill translations table with existing English content from weight_profiles
INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value)
SELECT 
    'trans-' || w.id || '-en-name-' || gen_random_uuid()::text,
    'weight_profile',
    w.id,
    'en',
    'name',
    w.name
FROM weight_profiles w
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;

-- Add French translations (backfilled from the i18n.rs hardcoded translations)
INSERT INTO translations (id, entity_type, entity_id, language_code, field_name, value) VALUES
-- Criteria translations to French
('trans-carbon_footprint-fr-name', 'criterion', 'carbon_footprint', 'fr', 'name', 'Empreinte carbone'),
('trans-carbon_footprint-fr-desc', 'criterion', 'carbon_footprint', 'fr', 'description', 'Émissions d''équivalent CO2 par kg produit, y compris le transport/l''emballage.'),
('trans-sourcing_ethics-fr-name', 'criterion', 'sourcing_ethics', 'fr', 'name', 'Éthique d''approvisionnement'),
('trans-sourcing_ethics-fr-desc', 'criterion', 'sourcing_ethics', 'fr', 'description', 'Garanties de commerce équitable, liens commerciaux directs et méthodes d''agriculture biologique/écologique.'),
('trans-e_waste-fr-name', 'criterion', 'e_waste', 'fr', 'name', 'Déchets électroniques & Recyclage'),
('trans-e_waste-fr-desc', 'criterion', 'e_waste', 'fr', 'description', 'Proportion de matériaux circulaires recyclés et facilité de recyclage en fin de vie.'),
('trans-supply_chain_ethics-fr-name', 'criterion', 'supply_chain_ethics', 'fr', 'name', 'Éthique de la chaîne d''approvisionnement'),
('trans-supply_chain_ethics-fr-desc', 'criterion', 'supply_chain_ethics', 'fr', 'description', 'Approvisionnement équitable en minéraux, normes de travail sans ateliers de misère et audits des fournisseurs.'),
('trans-durability-fr-name', 'criterion', 'durability', 'fr', 'name', 'Longévité & Durabilité'),
('trans-durability-fr-desc', 'criterion', 'durability', 'fr', 'description', 'Durée de vie attendue dans des conditions normales d''utilisation et couverture de garantie.'),
('trans-camera-fr-name', 'criterion', 'camera', 'fr', 'name', 'Qualité de la caméra'),
('trans-camera-fr-desc', 'criterion', 'camera', 'fr', 'description', 'Détails de l''image, options de zoom, traitement en basse lumière et stabilisation vidéo.'),
('trans-battery-fr-name', 'criterion', 'battery', 'fr', 'name', 'Batterie & Charge'),
('trans-battery-fr-desc', 'criterion', 'battery', 'fr', 'description', 'Temps d''écran allumé sous charge et capacités de charge rapide.'),
('trans-reparability-fr-name', 'criterion', 'reparability', 'fr', 'name', 'Indice de réparabilité'),
('trans-reparability-fr-desc', 'criterion', 'reparability', 'fr', 'description', 'Disponibilité des écrans/batteries de rechange et facilité de démontage.'),
('trans-performance-fr-name', 'criterion', 'performance', 'fr', 'name', 'Performance & Jeu'),
('trans-performance-fr-desc', 'criterion', 'performance', 'fr', 'description', 'Vitesse de lancement de l''application, fluidité du multitâche et étranglement thermique.'),
('trans-aroma-fr-name', 'criterion', 'aroma', 'fr', 'name', 'Parfum & Arôme'),
('trans-aroma-fr-desc', 'criterion', 'aroma', 'fr', 'description', 'Complexité et intensité de l''odeur du café moulu et infusé.'),
('trans-acidity-fr-name', 'criterion', 'acidity', 'fr', 'name', 'Acidité vive'),
('trans-acidity-fr-desc', 'criterion', 'acidity', 'fr', 'description', 'Notes de fruits éclatantes et sensation de propreté et pétillante sur la langue.'),
('trans-body-fr-name', 'criterion', 'body', 'fr', 'name', 'Texture & Corps'),
('trans-body-fr-desc', 'criterion', 'body', 'fr', 'description', 'Texture, poids et onctuosité en bouche.'),
('trans-sweetness-fr-name', 'criterion', 'sweetness', 'fr', 'name', 'Douceur naturelle'),
('trans-sweetness-fr-desc', 'criterion', 'sweetness', 'fr', 'description', 'Sucres de caramel, de chocolat ou de baies mûres sans ajout d''édulcorants.'),
('trans-texture-fr-name', 'criterion', 'texture', 'fr', 'name', 'Texture & Grain'),
('trans-texture-fr-desc', 'criterion', 'texture', 'fr', 'description', 'Moelleux, légèreté et longueur du grain lorsqu''il est cuit correctement.'),
('trans-fragrance-fr-name', 'criterion', 'fragrance', 'fr', 'name', 'Arôme naturel'),
('trans-fragrance-fr-desc', 'criterion', 'fragrance', 'fr', 'description', 'Intensité des parfums floraux de jasmin/pandan ou des arômes de noisette du basmati.'),
('trans-ergonomics-fr-name', 'criterion', 'ergonomics', 'fr', 'name', 'Soutien lombaire'),
('trans-ergonomics-fr-desc', 'criterion', 'ergonomics', 'fr', 'description', 'Soutien spinal, correction de la posture et respirabilité du maillage.'),
('trans-adjustability-fr-name', 'criterion', 'adjustability', 'fr', 'name', 'Ajustements personnalisés'),
('trans-adjustability-fr-desc', 'criterion', 'adjustability', 'fr', 'description', 'Plage d''ajustement des accoudoirs, du verrouillage d''inclinaison, de la profondeur d''assise et de la hauteur.'),

-- Categories translations to French
('trans-food-fr-name', 'category', 'food', 'fr', 'name', 'Alimentation & Boisson'),
('trans-food-fr-desc', 'category', 'food', 'fr', 'description', 'Évaluez les produits de consommation en fonction de l''utilisation des ressources, des pratiques agricoles et de l''empreinte carbone.'),
('trans-technology-fr-name', 'category', 'technology', 'fr', 'name', 'Électronique & Tech'),
('trans-technology-fr-desc', 'category', 'technology', 'fr', 'description', 'Évaluez les appareils électroniques sur la base de chaînes d''approvisionnement éthiques et de la recyclabilité du matériel.'),
('trans-furniture-fr-name', 'category', 'furniture', 'fr', 'name', 'Maison & Style de vie'),
('trans-furniture-fr-desc', 'category', 'furniture', 'fr', 'description', 'Concentrez-vous sur l''ergonomie, la qualité des matériaux et la longévité attendue des meubles/décorations.'),

-- Product types translations to French
('trans-smartphones-fr-name', 'product_type', 'smartphones', 'fr', 'name', 'Smartphones'),
('trans-smartphones-fr-desc', 'product_type', 'smartphones', 'fr', 'description', 'Comparez des fonctionnalités telles que le rendu photo, la vitesse de performance et la modularité des réparations.'),
('trans-coffee-fr-name', 'product_type', 'coffee', 'fr', 'name', 'Café de spécialité'),
('trans-coffee-fr-desc', 'product_type', 'coffee', 'fr', 'description', 'Évaluez les grains de café de spécialité sur les notes aromatiques, l''acidité, le corps et la complexité des arômes.'),
('trans-rice-fr-name', 'product_type', 'rice', 'fr', 'name', 'Riz de qualité supérieure'),
('trans-rice-fr-desc', 'product_type', 'rice', 'fr', 'description', 'Évaluez les grains de riz gastronomiques sur l''arôme, la texture, la longueur du grain et le moelleux.'),
('trans-office_chairs-fr-name', 'product_type', 'office_chairs', 'fr', 'name', 'Chaises de bureau'),
('trans-office_chairs-fr-desc', 'product_type', 'office_chairs', 'fr', 'description', 'Évaluez les sièges de bureau ergonomiques sur les ajustements de posture et la qualité du soutien lombaire.'),

-- Products translations to French
('trans-iphone-15-pro-fr-name', 'product', 'iphone-15-pro', 'fr', 'name', 'iPhone 15 Pro'),
('trans-iphone-15-pro-fr-desc', 'product', 'iphone-15-pro', 'fr', 'description', 'Flagship premium en titane avec vidéo et performance de processeur de classe mondiale, mais conception verrouillée.'),
('trans-galaxy-s24-ultra-fr-name', 'product', 'galaxy-s24-ultra', 'fr', 'name', 'Galaxy S24 Ultra'),
('trans-galaxy-s24-ultra-fr-desc', 'product', 'galaxy-s24-ultra', 'fr', 'description', 'Grand écran, appareil photo polyvalent et stylet de conception. Performance élevée et prix élevé.'),
('trans-fairphone-5-fr-name', 'product', 'fairphone-5', 'fr', 'name', 'Fairphone 5'),
('trans-fairphone-5-fr-desc', 'product', 'fairphone-5', 'fr', 'description', 'Téléphone modulaire et hautement durable conçu pour l''auto-réparation avec une garantie de 5 ans de classe mondiale.'),
('trans-pixel-8a-fr-name', 'product', 'pixel-8a', 'fr', 'name', 'Google Pixel 8a'),
('trans-pixel-8a-fr-desc', 'product', 'pixel-8a', 'fr', 'description', 'Incroyable valeur rapport qualité-prix, fournissant des photos de classe mondiale et des fonctionnalités Google IA.'),
('trans-ethiopian-yirgacheffe-fr-name', 'product', 'ethiopian-yirgacheffe', 'fr', 'name', 'Ethiopian Yirgacheffe'),
('trans-ethiopian-yirgacheffe-fr-desc', 'product', 'ethiopian-yirgacheffe', 'fr', 'description', 'Renommé pour son acidité citronnée brillante, son arôme floral élégant et son corps léger semblable au thé.'),
('trans-colombian-supremo-fr-name', 'product', 'colombian-supremo', 'fr', 'name', 'Colombian Supremo'),
('trans-colombian-supremo-fr-desc', 'product', 'colombian-supremo', 'fr', 'description', 'Un classique très apprécié. Extrêmement équilibré avec une douceur riche en caramel et un corps moyen.'),
('trans-sumatran-mandheling-fr-name', 'product', 'sumatran-mandheling', 'fr', 'name', 'Sumatran Mandheling'),
('trans-sumatran-mandheling-fr-desc', 'product', 'sumatran-mandheling', 'fr', 'description', 'Complexe profondément, terreux, faible acidité et corps complet avec des notes de chocolat noir et de cèdre.'),
('trans-mass-market-roast-fr-name', 'product', 'mass-market-roast', 'fr', 'name', 'Supermarket Blend'),
('trans-mass-market-roast-fr-desc', 'product', 'mass-market-roast', 'fr', 'description', 'Torréfaction noire commerciale générique, amère et plate, provenant de canaux de ferme industrielle.'),
('trans-jasmine-rice-fr-name', 'product', 'jasmine-rice', 'fr', 'name', 'Premium Jasmine Rice'),
('trans-jasmine-rice-fr-desc', 'product', 'jasmine-rice', 'fr', 'description', 'Parfumé, tendre et légèrement collant, excellent avec les plats culinaires asiatiques.'),
('trans-basmati-rice-fr-name', 'product', 'basmati-rice', 'fr', 'name', 'Gourmet Basmati Rice'),
('trans-basmati-rice-fr-desc', 'product', 'basmati-rice', 'fr', 'description', 'Grain long, mince et aromatique qui reste léger et séparé après la cuisson.'),
('trans-cheap-white-rice-fr-name', 'product', 'cheap-white-rice', 'fr', 'name', 'Bulk White Rice'),
('trans-cheap-white-rice-fr-desc', 'product', 'cheap-white-rice', 'fr', 'description', 'Riz blanc commercial générique, traitement standard, rendement élevé, éthique d''approvisionnement faible.'),
('trans-herman-miller-aeron-fr-name', 'product', 'herman-miller-aeron', 'fr', 'name', 'Herman Miller Aeron'),
('trans-herman-miller-aeron-fr-desc', 'product', 'herman-miller-aeron', 'fr', 'description', 'L''étalon-or des chaises ergonomiques en maille, construit avec une recyclabilité élevée et une garantie de 12 ans.'),
('trans-steelcase-gesture-fr-name', 'product', 'steelcase-gesture', 'fr', 'name', 'Steelcase Gesture'),
('trans-steelcase-gesture-fr-desc', 'product', 'steelcase-gesture', 'fr', 'description', 'Chaise en tissu haut de gamme conçue pour soutenir divers styles de posture et le mouvement continu.'),
('trans-budget-mesh-chair-fr-name', 'product', 'budget-mesh-chair', 'fr', 'name', 'Basic Task Chair'),
('trans-budget-mesh-chair-fr-desc', 'product', 'budget-mesh-chair', 'fr', 'description', 'Chaise de bureau en plastique standard avec ajustement de hauteur simple et rembourrage fin, durée de vie courte.')
ON CONFLICT (entity_type, entity_id, language_code, field_name) DO NOTHING;
