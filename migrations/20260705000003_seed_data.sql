-- Seed criteria
INSERT INTO criteria (id, name, description, emoji) VALUES
('carbon_footprint', 'Carbon Footprint', 'CO2 equivalent emissions per kg produced, including transport/packaging.', '🌍'),
('sourcing_ethics', 'Sourcing Ethics', 'Fair-trade guarantees, direct trade links, and organic/eco farming methods.', '🤝'),
('e_waste', 'E-Waste & Recycling', 'Proportion of recycled circular materials and ease of end-of-life recycling.', '♻️'),
('supply_chain_ethics', 'Supply Chain Ethics', 'Fair mineral sourcing, sweatshop-free labor standards, and supplier audits.', '🤝'),
('durability', 'Longevity & Durability', 'Expected lifetime under normal use and warranty coverage.', '🛡️'),
('camera', 'Camera Quality', 'Image details, zoom options, low-light processing, and video stabilization.', '📷'),
('battery', 'Battery & Charging', 'Screen-on time under load and fast-charging capabilities.', '🔋'),
('reparability', 'Reparability Index', 'Availability of replacement screens/batteries and ease of disassembly.', '🔧'),
('performance', 'Performance & Gaming', 'App launching speed, multitasking smoothness, and thermal throttling.', '⚡'),
('aroma', 'Fragrance & Aroma', 'Complexity and intensity of the dry grounds and wet brew scent.', '👃'),
('acidity', 'Crisp Acidity', 'Bright fruit notes and clean, sparkling sensation on the tongue.', '🍋'),
('body', 'Mouthfeel & Body', 'Texture, weight, and creaminess on the palate.', '🥛'),
('sweetness', 'Natural Sweetness', 'Caramel, chocolate, or ripe berry sugars without adding sweeteners.', '🍯'),
('texture', 'Texture & Grain', 'Softness, fluffiness, and length of grain when cooked correctly.', '🌾'),
('fragrance', 'Natural Aroma', 'Strength of jasmine/pandan floral scents or basmati nutty aromas.', '🌸'),
('ergonomics', 'Lumbar Support', 'Spinal support alignment, posture correction, and mesh breathability.', '💺'),
('adjustability', 'Custom Adjustments', 'Armrest, tilt lock, seat depth, and height customization ranges.', '⚙️')
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    emoji = EXCLUDED.emoji;

-- Seed categories
INSERT INTO categories (id, name, description, emoji) VALUES
('food', 'Food & Beverage', 'Assess consumption items based on resource usage, farming practices, and footprint.', '🍎'),
('technology', 'Electronics & Tech', 'Evaluate electronic devices based on ethical supply chains and hardware recyclability.', '⚡'),
('furniture', 'Home & Lifestyle', 'Focus on ergonomics, material quality, and expected longevity of furniture/decor.', '🏡')
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    emoji = EXCLUDED.emoji;

-- Seed category_criteria (linking categories to criteria)
INSERT INTO category_criteria (category_id, criterion_id) VALUES
('food', 'carbon_footprint'),
('food', 'sourcing_ethics'),
('technology', 'e_waste'),
('technology', 'supply_chain_ethics'), -- Changed from 'sourcing_ethics' to match model
('furniture', 'durability')
ON CONFLICT (category_id, criterion_id) DO NOTHING;

-- Seed product_types
INSERT INTO product_types (id, name, description, emoji) VALUES
('smartphones', 'Smartphones', 'Compare features like photo output, performance speed, and repair modularity.', '📱'),
('coffee', 'Specialty Coffee', 'Grade specialty coffee beans on flavor notes, acidity, body, and aroma complexity.', '☕'),
('rice', 'Premium Rice', 'Evaluate gourmet rice grains on scent, texture, grain length, and fluffiness.', '🌾'),
('office_chairs', 'Office Chairs', 'Grade ergonomic desk seating on posture adjustments and lumbar support quality.', '💺')
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    emoji = EXCLUDED.emoji;

-- Seed product_type_categories (linking product types to categories)
INSERT INTO product_type_categories (product_type_id, category_id) VALUES
('smartphones', 'technology'),
('coffee', 'food'),
('rice', 'food'),
('office_chairs', 'furniture')
ON CONFLICT (product_type_id, category_id) DO NOTHING;

-- Seed product_type_specific_criteria (linking product types to specific criteria)
INSERT INTO product_type_specific_criteria (product_type_id, criterion_id) VALUES
('smartphones', 'camera'),
('smartphones', 'battery'),
('smartphones', 'reparability'),
('smartphones', 'performance'),
('coffee', 'aroma'),
('coffee', 'acidity'),
('coffee', 'body'),
('coffee', 'sweetness'),
('rice', 'texture'),
('rice', 'fragrance'),
('office_chairs', 'ergonomics'),
('office_chairs', 'adjustability')
ON CONFLICT (product_type_id, criterion_id) DO NOTHING;

-- Seed products
INSERT INTO products (id, name, description, product_type_id) VALUES
('iphone-15-pro', 'iPhone 15 Pro', 'Premium titanium flagship with class-leading video and processor performance, but locked down design.', 'smartphones'),
('galaxy-s24-ultra', 'Galaxy S24 Ultra', 'Large display, versatile cameras, and styling pen. High performance and price tag.', 'smartphones'),
('fairphone-5', 'Fairphone 5', 'Modular, highly sustainable phone designed for self-repair with an industry-best 5-year warranty.', 'smartphones'),
('pixel-8a', 'Google Pixel 8a', 'Incredible price-to-performance value, delivering flagship-grade photos and Google AI features.', 'smartphones'),
('ethiopian-yirgacheffe', 'Ethiopian Yirgacheffe', 'Renowned for its bright citrus acidity, elegant floral aroma, and tea-like light body.', 'coffee'),
('colombian-supremo', 'Colombian Supremo', 'A classic crowd-pleaser. Extremely balanced with rich caramel sweetness and medium body.', 'coffee'),
('sumatran-mandheling', 'Sumatran Mandheling', 'Deeply complex, earthy, low acid, and full-bodied with notes of dark chocolate and cedarwood.', 'coffee'),
('mass-market-roast', 'Supermarket Blend', 'Generic commercial dark roast, bitter and flat, sourced through industrial farm channels.', 'coffee'),
('jasmine-rice', 'Premium Jasmine Rice', 'Fragrant, soft, and slightly sticky, excellent with Asian culinary dishes.', 'rice'),
('basmati-rice', 'Gourmet Basmati Rice', 'Long, slender, aromatic grain that remains fluffy and separate after cooking.', 'rice'),
('cheap-white-rice', 'Bulk White Rice', 'Generic commercial white rice, standard processing, high yield, low trace sourcing ethics.', 'rice'),
('herman-miller-aeron', 'Herman Miller Aeron', 'The gold standard of ergonomic mesh chairs, built with high recyclability and a 12-year warranty.', 'office_chairs'),
('steelcase-gesture', 'Steelcase Gesture', 'Premium fabric chair designed to support diverse posture styles and continuous movement.', 'office_chairs'),
('budget-mesh-chair', 'Basic Task Chair', 'Standard plastic office chair with simple height adjust and thin padding, short lifespan.', 'office_chairs')
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    product_type_id = EXCLUDED.product_type_id;

-- Seed product_scores
INSERT INTO product_scores (product_id, criterion_id, score) VALUES
-- iPhone 15 Pro
('iphone-15-pro', 'e_waste', 4.5),
('iphone-15-pro', 'sourcing_ethics', 5.0),
('iphone-15-pro', 'camera', 9.4),
('iphone-15-pro', 'battery', 8.0),
('iphone-15-pro', 'reparability', 4.2),
('iphone-15-pro', 'performance', 9.7),
-- Galaxy S24 Ultra
('galaxy-s24-ultra', 'e_waste', 5.0),
('galaxy-s24-ultra', 'sourcing_ethics', 4.8),
('galaxy-s24-ultra', 'camera', 9.5),
('galaxy-s24-ultra', 'battery', 8.8),
('galaxy-s24-ultra', 'reparability', 5.0),
('galaxy-s24-ultra', 'performance', 9.6),
-- Fairphone 5
('fairphone-5', 'e_waste', 9.5),
('fairphone-5', 'sourcing_ethics', 9.8),
('fairphone-5', 'camera', 6.5),
('fairphone-5', 'battery', 7.5),
('fairphone-5', 'reparability', 10.0),
('fairphone-5', 'performance', 6.8),
-- Google Pixel 8a
('pixel-8a', 'e_waste', 5.5),
('pixel-8a', 'sourcing_ethics', 5.8),
('pixel-8a', 'camera', 8.8),
('pixel-8a', 'battery', 7.8),
('pixel-8a', 'reparability', 5.5),
('pixel-8a', 'performance', 8.0),
-- Ethiopian Yirgacheffe
('ethiopian-yirgacheffe', 'carbon_footprint', 7.5),
('ethiopian-yirgacheffe', 'sourcing_ethics', 8.2),
('ethiopian-yirgacheffe', 'aroma', 9.6),
('ethiopian-yirgacheffe', 'acidity', 9.2),
('ethiopian-yirgacheffe', 'body', 4.5),
('ethiopian-yirgacheffe', 'sweetness', 8.8),
-- Colombian Supremo
('colombian-supremo', 'carbon_footprint', 6.8),
('colombian-supremo', 'sourcing_ethics', 8.5),
('colombian-supremo', 'aroma', 8.2),
('colombian-supremo', 'acidity', 6.5),
('colombian-supremo', 'body', 7.6),
('colombian-supremo', 'sweetness', 8.5),
-- Sumatran Mandheling
('sumatran-mandheling', 'carbon_footprint', 7.2),
('sumatran-mandheling', 'sourcing_ethics', 7.0),
('sumatran-mandheling', 'aroma', 7.8),
('sumatran-mandheling', 'acidity', 3.2),
('sumatran-mandheling', 'body', 9.5),
('sumatran-mandheling', 'sweetness', 6.0),
-- Supermarket Blend
('mass-market-roast', 'carbon_footprint', 3.8),
('mass-market-roast', 'sourcing_ethics', 2.5),
('mass-market-roast', 'aroma', 3.5),
('mass-market-roast', 'acidity', 4.0),
('mass-market-roast', 'body', 5.5),
('mass-market-roast', 'sweetness', 3.0),
-- Premium Jasmine Rice
('jasmine-rice', 'carbon_footprint', 7.5),
('jasmine-rice', 'sourcing_ethics', 8.0),
('jasmine-rice', 'texture', 9.0),
('jasmine-rice', 'fragrance', 9.5),
-- Gourmet Basmati Rice
('basmati-rice', 'carbon_footprint', 7.0),
('basmati-rice', 'sourcing_ethics', 7.8),
('basmati-rice', 'texture', 9.2),
('basmati-rice', 'fragrance', 9.0),
-- Bulk White Rice
('cheap-white-rice', 'carbon_footprint', 5.0),
('cheap-white-rice', 'sourcing_ethics', 3.0),
('cheap-white-rice', 'texture', 5.5),
('cheap-white-rice', 'fragrance', 3.0),
-- Herman Miller Aeron
('herman-miller-aeron', 'durability', 9.8),
('herman-miller-aeron', 'ergonomics', 9.6),
('herman-miller-aeron', 'adjustability', 9.2),
-- Steelcase Gesture
('steelcase-gesture', 'durability', 9.5),
('steelcase-gesture', 'ergonomics', 9.5),
('steelcase-gesture', 'adjustability', 9.8),
-- Basic Task Chair
('budget-mesh-chair', 'durability', 4.0),
('budget-mesh-chair', 'ergonomics', 5.0),
('budget-mesh-chair', 'adjustability', 4.5)
ON CONFLICT (product_id, criterion_id) DO UPDATE SET
    score = EXCLUDED.score;

-- Seed System Weight Profiles (Presets)
INSERT INTO weight_profiles (id, product_type_id, name, weights, is_system) VALUES
-- Smartphones presets
('phone-balanced', 'smartphones', 'Balanced Default', '{"e_waste": 5.0, "sourcing_ethics": 5.0, "camera": 5.0, "battery": 5.0, "reparability": 5.0, "performance": 5.0}', TRUE),
('phone-eco', 'smartphones', 'Eco & Repair Advocate', '{"e_waste": 8.0, "sourcing_ethics": 9.0, "camera": 3.0, "battery": 5.0, "reparability": 10.0, "performance": 3.0}', TRUE),
('phone-power', 'smartphones', 'Power Gamer / Geek', '{"e_waste": 2.0, "sourcing_ethics": 3.0, "camera": 7.0, "battery": 8.0, "reparability": 2.0, "performance": 10.0}', TRUE),
-- Coffee presets
('coffee-balanced', 'coffee', 'Balanced Filter Roast', '{"carbon_footprint": 5.0, "sourcing_ethics": 5.0, "aroma": 6.0, "acidity": 6.0, "body": 4.0, "sweetness": 6.0}', TRUE),
('coffee-bright', 'coffee', 'Bright & Fruity', '{"carbon_footprint": 4.0, "sourcing_ethics": 6.0, "aroma": 9.0, "acidity": 10.0, "body": 2.0, "sweetness": 8.0}', TRUE),
('coffee-rich', 'coffee', 'Rich & Heavy Espresso', '{"carbon_footprint": 4.0, "sourcing_ethics": 6.0, "aroma": 8.0, "acidity": 2.0, "body": 10.0, "sweetness": 8.0}', TRUE),
-- Rice presets
('rice-balanced', 'rice', 'Balanced Cooking', '{"carbon_footprint": 5.0, "sourcing_ethics": 5.0, "texture": 6.0, "fragrance": 6.0}', TRUE),
('rice-aromatic', 'rice', 'Aromatic & Fluffy First', '{"carbon_footprint": 4.0, "sourcing_ethics": 7.0, "texture": 7.0, "fragrance": 10.0}', TRUE),
-- Office chairs presets
('chair-ergo', 'office_chairs', 'Ergonomic Office Worker', '{"durability": 8.0, "ergonomics": 10.0, "adjustability": 9.0}', TRUE),
('chair-minimalist', 'office_chairs', 'Minimalist Durable', '{"durability": 10.0, "ergonomics": 6.0, "adjustability": 5.0}', TRUE)
ON CONFLICT (id) DO UPDATE SET
    product_type_id = EXCLUDED.product_type_id,
    name = EXCLUDED.name,
    weights = EXCLUDED.weights,
    is_system = EXCLUDED.is_system;
