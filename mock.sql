--RECIPES
insert into recipes values 
    (1, 'Mushroom-Risotto', 'A creamy italian dish', 2, 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse2.mm.bing.net%2Fth%3Fid%3DOIP.bgYmIo8ata1o2ifT385NnAHaHa%26pid%3DApi&f=1&ipt=08604a185782e4e904ac3a12e369b6ac1534c02811b607f62a91c927552764d3&ipo=images'),
    (2, 'Dumplings', 'A Bavarian receipe', 2, 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse4.mm.bing.net%2Fth%3Fid%3DOIP.YWvT_e0FJJ4S50KCIqhUPgHaFj%26pid%3DApi&f=1&ipt=aa0eaa922ea65abaf5e583b615c077b3320c8156ce382ba43994824c7fb3c01d&ipo=images'),
    (3, 'Leberkaese Burger', 'A Bavarian receipe', 4, 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.9MbDLrIfGBcyPvLKY_bnjwHaHa%26pid%3DApi&f=1&ipt=29f655a42c7228a68020ba659bdd24b89e9a64f471786572285925fb19e1290a&ipo=images'),
    (4, 'Chocolate Brownies', 'A sweet deal! (for 2)', 2, 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.-fLP1Y5PBw8v-NTF5RF7lwHaHa%26pid%3DApi&f=1&ipt=2a1a718df363a5243d99f36fda4e2dce6156d9578058768f9e38bde9aeaba17d&ipo=images');

--INGREDIENTS
--Ingredients Mushroom Risotto
insert into ingredients values
    (1, 'Mushrooms', 'Stone','mushrooms'),
    (2, 'Mushrooms', 'Normal', 'mushrooms'),
    (3, 'Water', 'Tap','water'),
    (4, 'Onions', '','onions'),
    (5, 'Garlic', '', 'garlic'),
    (6, 'Rice', 'For risotto', 'rice'),
    (7, 'Rice', 'Basmati','rice'),
    (8, 'Frozen peas', '', 'legume'),
    (9, 'Salt', '','salt'),
    (10, 'Pepper', '', 'pepper'),
    (11, 'Parmesan', '', 'parmesan'),
    (12, 'Butter', 'Animal based', 'butter'),
    (13, 'Plant Oil', '', 'oil'),
    (14, 'Basil', '', 'basil');

--Ingredients mushroom risotto vegan
insert into ingredients values
    (15, 'Vegan white wine', '', 'wine'),
    (16, 'Vegan butter', '', 'butter'),
    (17, 'Vegan Parmesan', '','parmesan');

-- More ingredients
insert into ingredients values
    (18, 'Milk', '', 'milk'),
    (19, 'Egg', '', 'egg'),
    (20, 'Shellfish', '', 'fish'),
    (21, 'Soy', '', 'meat'),
    (22, 'Flour', '', 'flour');

--Alternative legumes
insert into ingredients values
    (23, 'Kidney beans', '', 'legumes'),
    (24, 'Chickpeas', '', 'legumes'),
    (25, 'Lentils', '', 'legumes'),
    (26, 'Testparmesan2', '', 'parmesan');

--Disliked ingridients
insert into ingredients values
    (27, 'Coriander', '', 'coriander'),
    (28, 'Olives', '', 'olives'),
    (29, 'Blue Cheese', '', 'cheese'),
    (30, 'Coffee', '', 'coffee'),
    (31, 'Peanuts', '', 'nuts'),
    (32, 'Tree nuts', '', 'nuts');

--Leberkaese
insert into ingredients values
    (33, 'Senf', '', 'senf'),
    (34, 'Coleslaw', '', 'salat'),
    (35, 'Radishes', '', 'radishes'),
    (36, 'Leberkaese', '', 'leberkaese'),
    (37, 'Laugenbroetchen', '', 'roll');

--Leberkaese Ersatz 
insert into ingredients values
    (38, 'Gluten free roll', '', 'roll');
-- + soy

--Chocolate Brownie
insert into ingredients values 
    (39, 'Coconut oil', '', 'oil'),
    (40, 'Sugar', '', 'sugar'),
    (41, 'Cocoa Powder', '', 'cocoa'),
    (42, 'Banana', '', 'egg'),
    (43, 'Chocolate', '', 'choc'),
    (44, 'Baking powder', '', 'bakpow');

--Dumplings
insert into ingredients values
    (45, 'Roll', '', 'roll'),
    (46, 'Parsely', '', 'herbs'),
    (47, 'Breadcrumbs', '', 'roll');


--_______________________________________________________


-- AMOUNT AND ADD INGREDIENTS TO RECIPE
-- Unique, receipe id, ingridient id, amount, unit, optional

--Mushroom Risotto
insert into recipe_ingredients values
    (1, 1, 1, 500, 'g', TRUE),
    (2, 1, 2, 100, 'g', FALSE),
    (3, 1, 3, 1000, 'ml', FALSE),
    (4, 1, 4, 1, 'pcs', TRUE),
    (5, 1, 5, 2, 'pcs', TRUE),
    (6, 1, 6, 200, 'g', FALSE),
    (8, 1, 8, 100, 'g', TRUE),
    (9, 1, 9, 5, 'g', TRUE),
    (10, 1, 10, 8, 'g', TRUE),
    (11, 1, 11, 100, 'g', TRUE),
    (12, 1, 12, 50, 'g', TRUE),
    (13, 1, 13, 50, 'ml', TRUE),
    (14, 1, 14, 20, 'g', TRUE),
    (29, 1, 27, 10, 'g', TRUE);

-- Leberkasburger
insert into recipe_ingredients values
    (15, 3, 33, 7, 'g', TRUE),
    (16, 3, 34, 50, 'g', TRUE),
    (17, 3, 35, 2, 'pcs', TRUE),
    (18, 3, 36, 2, 'pcs', FALSE),
    (19, 3, 37, 1, 'pcs', FALSE);

-- Dumplings
insert into recipe_ingredients values
    (20, 2, 45, 3, 'pcs', FALSE),
    (21, 2, 46, 20, 'g', TRUE),
    (22, 2, 47, 50, 'g', TRUE),
    (30, 2, 19, 4, 'pcs', FALSE);

-- Chocolate Brownies
insert into recipe_ingredients values
    (23, 4, 39, 100, 'ml', TRUE),
    (24, 4, 40, 200, 'g', FALSE),
    (25, 4, 41, 50, 'g', TRUE),
    (26, 4, 42, 1, 'pcs', TRUE),
    (27, 4, 43, 100, 'g', FALSE),
    (28, 4, 44, 3.5, 'g', FALSE);

-- FOOD GROUPS 
-- id, id ingredient, food group id (1, 'Lactose',
-- 2, 'Gluten' 3, 'Histamines', 4, 'Seafood')
insert into ingredient_food_groups values
    (1, 4, 3),
    (2, 5, 3),
    (3, 6, 3),
    (4, 8, 3),
    (5, 11, 3),
    (6, 11, 1),
    (7, 12, 1),
    (8, 15, 3),
    (9, 18, 1),
    (10, 19, 1),
    (11, 20, 4),
    (12, 21, 3),
    (13, 22, 1),
    (14, 26, 1),
    (15, 39, 3),
    (16, 40, 3),
    (17, 41, 3),
    (18, 42, 3),
    (19, 43, 3),
    (20, 44, 3),
    (21, 45, 2),
    (22, 46, 3),
    (23, 47, 2),
    (24, 1, 3);

-- DIET GOALS
--id, ingredient_id INTEGER NOT NULL, special_diet_id
--(1, 'Low Carb'),(2, 'High Carb'),(3, 'High Fat'),(4,'Clean Eating');
insert into ingredient_diet_goals values
    (1, 34, 1),
    (2, 37, 2),
    (3, 36, 3),
    (4, 11, 1),
    (5, 35, 1),
    (6, 38, 2),
    (7, 1, 4),
    (8, 33, 4),
    (10, 16, 3),
    (11, 39, 3),
    (12, 40, 2),
    (13, 41, 1),
    (14, 42, 4),
    (15, 43, 2),
    (16, 44, 2),
    (17, 45, 2),
    (18, 46, 4),
    (19, 47, 2),
    (20, 2, 4),
    (21, 3, 4),
    (22, 4, 1),
    (23, 5, 1),
    (24, 6, 2),
    (25, 7, 2),
    (26, 8, 1),
    (27, 11, 3),
    (28, 12, 3),
    (29, 13, 3),
    (30, 14, 4);

-- SPECIAL DIETS
-- diet gols id, ingredient_id INTEGER NOT NULL, diet_goal_id INT
-- (1, 'Vegan'),(2, 'Vegetarian'),(3, 'Pescetarian'),(4, 'Keto');
insert into ingredient_special_diets values
    (1, 33, 1),
    (2, 34, 1),
    (3, 35, 1),
    (4, 36, 4),
    (5, 37, 2),
    (6, 39, 1),
    (7, 40, 1),
    (8, 41, 1),
    (9, 42, 1),
    (10, 43, 2),
    (11, 44, 1),
    (12, 45, 1),
    (13, 46, 1),
    (14, 47, 1),
    (15, 1, 1),
    (16, 2, 1),
    (17, 3, 1),
    (18, 4, 1),
    (19, 5, 1),
    (20, 6, 1),
    (21, 7, 1),
    (22, 8, 1),
    (23, 9, 1),
    (24, 10, 1),
    (25, 11, 2),
    (26, 12, 2),
    (27, 13, 1),
    (28, 14, 1);
