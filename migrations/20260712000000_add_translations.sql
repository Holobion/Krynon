-- ============================================================================
-- 20260712000000_add_translations.sql
--
-- Standardizes multilingual support using BCP 47 language tags.
-- - Uses a `locales` reference table for known/standardized language codes.
-- - Replaces the legacy `translations` table with typed translation tables
--   per entity (category, criterion, product_type, product, weight_profile).
-- - Stores ONLY the languages that actually have human-authored content.
--   Missing translations are never silently filled with another language.
-- - Initially exposes only English ('en') and French ('fr').
-- ============================================================================

-- 1) Locales reference table
CREATE TABLE IF NOT EXISTS locales (
    code          TEXT PRIMARY KEY,        -- BCP 47 tag, e.g. 'en', 'fr', 'fr-CA'
    english_name  TEXT NOT NULL,
    native_name   TEXT NOT NULL,
    is_default    BOOLEAN NOT NULL DEFAULT FALSE,
    is_enabled    BOOLEAN NOT NULL DEFAULT TRUE,
    created_at    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Ensure at most one default locale
CREATE UNIQUE INDEX IF NOT EXISTS idx_locales_default ON locales ((1)) WHERE is_default;

-- Seed standardized locales (English and French enabled initially)
INSERT INTO locales (code, english_name, native_name, is_default, is_enabled) VALUES
    ('en', 'English',    'English',    TRUE,  TRUE),
    ('fr', 'French',     'Français',   FALSE, TRUE),
    ('es', 'Spanish',    'Español',    FALSE, FALSE),
    ('de', 'German',     'Deutsch',    FALSE, FALSE),
    ('it', 'Italian',    'Italiano',   FALSE, FALSE),
    ('pt', 'Portuguese', 'Português',  FALSE, FALSE),
    ('ja', 'Japanese',   '日本語',     FALSE, FALSE),
    ('zh', 'Chinese',    '中文',       FALSE, FALSE)
ON CONFLICT (code) DO NOTHING;

-- 2) Typed translation tables
-- A row exists ONLY for a locale that has been human-authored. There is no
-- implicit fallback row; the frontend performs the fallback at render time.

CREATE TABLE IF NOT EXISTS category_translations (
    category_id  TEXT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    locale       TEXT NOT NULL REFERENCES locales(code) ON UPDATE CASCADE,
    name         TEXT NOT NULL,
    description  TEXT,
    PRIMARY KEY (category_id, locale)
);

CREATE TABLE IF NOT EXISTS criterion_translations (
    criterion_id TEXT NOT NULL REFERENCES criteria(id) ON DELETE CASCADE,
    locale       TEXT NOT NULL REFERENCES locales(code) ON UPDATE CASCADE,
    name         TEXT NOT NULL,
    description  TEXT,
    PRIMARY KEY (criterion_id, locale)
);

CREATE TABLE IF NOT EXISTS product_type_translations (
    product_type_id TEXT NOT NULL REFERENCES product_types(id) ON DELETE CASCADE,
    locale          TEXT NOT NULL REFERENCES locales(code) ON UPDATE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    PRIMARY KEY (product_type_id, locale)
);

CREATE TABLE IF NOT EXISTS product_translations (
    product_id   TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    locale       TEXT NOT NULL REFERENCES locales(code) ON UPDATE CASCADE,
    name         TEXT NOT NULL,
    description  TEXT,
    PRIMARY KEY (product_id, locale)
);

CREATE TABLE IF NOT EXISTS weight_profile_translations (
    weight_profile_id TEXT NOT NULL REFERENCES weight_profiles(id) ON DELETE CASCADE,
    locale            TEXT NOT NULL REFERENCES locales(code) ON UPDATE CASCADE,
    name              TEXT NOT NULL,
    PRIMARY KEY (weight_profile_id, locale)
);

CREATE INDEX IF NOT EXISTS idx_category_translations_locale       ON category_translations(locale);
CREATE INDEX IF NOT EXISTS idx_criterion_translations_locale      ON criterion_translations(locale);
CREATE INDEX IF NOT EXISTS idx_product_type_translations_locale  ON product_type_translations(locale);
CREATE INDEX IF NOT EXISTS idx_product_translations_locale        ON product_translations(locale);
CREATE INDEX IF NOT EXISTS idx_weight_profile_translations_locale ON weight_profile_translations(locale);

-- 3) Backfill: every existing entity already has English source content in
--    the legacy base tables. Persist those rows as canonical 'en' translations
--    so the new schema has a single source of truth. The legacy `name` and
--    `description` columns are no longer the canonical read source after this
--    migration; the typed translation tables are.
--
--    No fallback row is inserted for any other language.

INSERT INTO category_translations (category_id, locale, name, description)
SELECT id, 'en', name, description FROM categories
ON CONFLICT (category_id, locale) DO NOTHING;

INSERT INTO criterion_translations (criterion_id, locale, name, description)
SELECT id, 'en', name, description FROM criteria
ON CONFLICT (criterion_id, locale) DO NOTHING;

INSERT INTO product_type_translations (product_type_id, locale, name, description)
SELECT id, 'en', name, description FROM product_types
ON CONFLICT (product_type_id, locale) DO NOTHING;

INSERT INTO product_translations (product_id, locale, name, description)
SELECT id, 'en', name, description FROM products
ON CONFLICT (product_id, locale) DO NOTHING;

INSERT INTO weight_profile_translations (weight_profile_id, locale, name)
SELECT id, 'en', name FROM weight_profiles
ON CONFLICT (weight_profile_id, locale) DO NOTHING;

-- 4) French (BCP 47 'fr') translations.
--    Only entities that have a real French translation are inserted. Any
--    entity missing from this block will fall back to English client-side
--    (the DB never invents a French row from the English content).

-- Criteria
INSERT INTO criterion_translations (criterion_id, locale, name, description) VALUES
    ('carbon_footprint',      'fr', 'Empreinte carbone',          'Émissions d''équivalent CO2 par kg produit, y compris le transport et l''emballage.'),
    ('sourcing_ethics',       'fr', 'Éthique d''approvisionnement','Garanties de commerce équitable, liens commerciaux directs et méthodes d''agriculture biologique ou écologique.'),
    ('e_waste',               'fr', 'Déchets électroniques & Recyclage', 'Proportion de matériaux circulaires recyclés et facilité de recyclage en fin de vie.'),
    ('supply_chain_ethics',   'fr', 'Éthique de la chaîne d''approvisionnement', 'Approvisionnement équitable en minéraux, normes de travail sans ateliers de misère et audits des fournisseurs.'),
    ('durability',            'fr', 'Longévité & Durabilité',     'Durée de vie attendue dans des conditions normales d''utilisation et couverture de garantie.'),
    ('camera',                'fr', 'Qualité de la caméra',       'Détails de l''image, options de zoom, traitement en basse lumière et stabilisation vidéo.'),
    ('battery',               'fr', 'Batterie & Charge',          'Temps d''écran allumé sous charge et capacités de charge rapide.'),
    ('reparability',          'fr', 'Indice de réparabilité',     'Disponibilité des écrans et batteries de rechange et facilité de démontage.'),
    ('performance',           'fr', 'Performance & Jeu',          'Vitesse de lancement de l''application, fluidité du multitâche et étranglement thermique.'),
    ('aroma',                 'fr', 'Parfum & Arôme',             'Complexité et intensité de l''odeur du café moulu et infusé.'),
    ('acidity',               'fr', 'Acidité vive',               'Notes de fruits éclatantes et sensation de propreté et pétillante sur la langue.'),
    ('body',                  'fr', 'Texture & Corps',            'Texture, poids et onctuosité en bouche.'),
    ('sweetness',             'fr', 'Douceur naturelle',          'Sucres de caramel, de chocolat ou de baies mûres sans ajout d''édulcorants.'),
    ('texture',               'fr', 'Texture & Grain',            'Moelleux, légèreté et longueur du grain lorsqu''il est cuit correctement.'),
    ('fragrance',             'fr', 'Arôme naturel',              'Intensité des parfums floraux de jasmin ou pandan ou des arômes de noisette du basmati.'),
    ('ergonomics',            'fr', 'Soutien lombaire',           'Soutien spinal, correction de la posture et respirabilité du maillage.'),
    ('adjustability',         'fr', 'Ajustements personnalisés',  'Plage d''ajustement des accoudoirs, du verrouillage d''inclinaison, de la profondeur d''assise et de la hauteur.')
ON CONFLICT (criterion_id, locale) DO NOTHING;

-- Categories
INSERT INTO category_translations (category_id, locale, name, description) VALUES
    ('food',        'fr', 'Alimentation & Boisson', 'Évaluez les produits de consommation en fonction de l''utilisation des ressources, des pratiques agricoles et de l''empreinte carbone.'),
    ('technology',  'fr', 'Électronique & Tech',   'Évaluez les appareils électroniques sur la base de chaînes d''approvisionnement éthiques et de la recyclabilité du matériel.'),
    ('furniture',   'fr', 'Maison & Style de vie', 'Concentrez-vous sur l''ergonomie, la qualité des matériaux et la longévité attendue des meubles et décorations.')
ON CONFLICT (category_id, locale) DO NOTHING;

-- Product types
INSERT INTO product_type_translations (product_type_id, locale, name, description) VALUES
    ('smartphones',   'fr', 'Smartphones',         'Comparez des fonctionnalités telles que le rendu photo, la vitesse de performance et la modularité des réparations.'),
    ('coffee',        'fr', 'Café de spécialité',  'Évaluez les grains de café de spécialité sur les notes aromatiques, l''acidité, le corps et la complexité des arômes.'),
    ('rice',          'fr', 'Riz de qualité supérieure', 'Évaluez les grains de riz gastronomiques sur l''arôme, la texture, la longueur du grain et le moelleux.'),
    ('office_chairs', 'fr', 'Chaises de bureau',    'Évaluez les sièges de bureau ergonomiques sur les ajustements de posture et la qualité du soutien lombaire.')
ON CONFLICT (product_type_id, locale) DO NOTHING;

-- Products
INSERT INTO product_translations (product_id, locale, name, description) VALUES
    ('iphone-15-pro',          'fr', 'iPhone 15 Pro',          'Fleuron en titane haut de gamme avec vidéo et processeur de classe mondiale, mais conception verrouillée.'),
    ('galaxy-s24-ultra',       'fr', 'Galaxy S24 Ultra',       'Grand écran, appareil photo polyvalent et stylet. Haute performance et prix élevé.'),
    ('fairphone-5',            'fr', 'Fairphone 5',            'Téléphone modulaire et hautement durable conçu pour l''auto-réparation avec une garantie de 5 ans de classe mondiale.'),
    ('pixel-8a',               'fr', 'Google Pixel 8a',        'Incroyable valeur rapport qualité-prix, fournissant des photos de classe mondiale et des fonctionnalités Google IA.'),
    ('ethiopian-yirgacheffe',  'fr', 'Ethiopian Yirgacheffe',  'Renommé pour son acidité citronnée brillante, son arôme floral élégant et son corps léger semblable au thé.'),
    ('colombian-supremo',      'fr', 'Colombian Supremo',      'Un classique très apprécié. Extrêmement équilibré avec une douceur riche en caramel et un corps moyen.'),
    ('sumatran-mandheling',    'fr', 'Sumatran Mandheling',    'Complexe profondément, terreux, faible acidité et corps complet avec des notes de chocolat noir et de cèdre.'),
    ('mass-market-roast',      'fr', 'Mélange de supermarché', 'Torréfaction noire commerciale générique, amère et plate, provenant de canaux de ferme industrielle.'),
    ('jasmine-rice',           'fr', 'Riz jasmin premium',     'Parfumé, tendre et légèrement collant, excellent avec les plats culinaires asiatiques.'),
    ('basmati-rice',           'fr', 'Riz basmati gastronomique', 'Grain long, mince et aromatique qui reste léger et séparé après la cuisson.'),
    ('cheap-white-rice',       'fr', 'Riz blanc en vrac',      'Riz blanc commercial générique, traitement standard, rendement élevé, faible éthique d''approvisionnement.'),
    ('herman-miller-aeron',    'fr', 'Herman Miller Aeron',    'L''étalon-or des chaises ergonomiques en maille, construit avec une recyclabilité élevée et une garantie de 12 ans.'),
    ('steelcase-gesture',      'fr', 'Steelcase Gesture',      'Chaise en tissu haut de gamme conçue pour soutenir divers styles de posture et le mouvement continu.'),
    ('budget-mesh-chair',      'fr', 'Chaise de travail basique', 'Chaise de bureau en plastique standard avec ajustement de hauteur simple et rembourrage fin, durée de vie courte.')
ON CONFLICT (product_id, locale) DO NOTHING;

-- Weight profiles (presets)
INSERT INTO weight_profile_translations (weight_profile_id, locale, name) VALUES
    ('phone-balanced',    'fr', 'Défaut équilibré'),
    ('phone-eco',         'fr', 'Défenseur de l''éco-réparation'),
    ('phone-power',       'fr', 'Joueur exigeant / Geek'),
    ('coffee-balanced',   'fr', 'Torréfaction filtre équilibrée'),
    ('coffee-bright',     'fr', 'Éclatant & Fruité'),
    ('coffee-rich',       'fr', 'Expresso riche & corsé'),
    ('rice-balanced',     'fr', 'Cuisson équilibrée'),
    ('rice-aromatic',     'fr', 'Arôme & Moelleux d''abord'),
    ('chair-ergo',        'fr', 'Travailleur de bureau ergonomique'),
    ('chair-minimalist',  'fr', 'Durable minimaliste')
ON CONFLICT (weight_profile_id, locale) DO NOTHING;

-- 5) Cleanup of the previous polymorphic translation store. It is replaced by
--    the typed translation tables above; the legacy table is dropped so the
--    schema has a single, standardized source of truth.
DROP TABLE IF EXISTS translations;
DROP TABLE IF EXISTS supported_languages;
