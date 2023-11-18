--RECEIPS
insert into recipes values (1, 'Mushroom-Risotto', 'A creamy italian dish', 2, 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse2.mm.bing.net%2Fth%3Fid%3DOIP.bgYmIo8ata1o2ifT385NnAHaHa%26pid%3DApi&f=1&ipt=08604a185782e4e904ac3a12e369b6ac1534c02811b607f62a91c927552764d3&ipo=images');
insert into recipes values (2, 'Dumplings', 'A Bavarian receipe', 2, 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse4.mm.bing.net%2Fth%3Fid%3DOIP.YWvT_e0FJJ4S50KCIqhUPgHaFj%26pid%3DApi&f=1&ipt=aa0eaa922ea65abaf5e583b615c077b3320c8156ce382ba43994824c7fb3c01d&ipo=images');
insert into recipes values (3, 'Leberkaese Burger', 'A Bavarian receipe', 4, 'https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse1.mm.bing.net%2Fth%3Fid%3DOIP.9MbDLrIfGBcyPvLKY_bnjwHaHa%26pid%3DApi&f=1&ipt=29f655a42c7228a68020ba659bdd24b89e9a64f471786572285925fb19e1290a&ipo=images');

--INGREDIENTS
--Ingredients Mushroom Risotto
insert into ingredients values (1, 'Mushrooms', 'Stone','mushrooms');
insert into ingredients values (2, 'Mushrooms', 'Normal', 'mushrooms');
insert into ingredients values (3, 'Water', 'Tap','x');
insert into ingredients values (4, 'Onions', '','onions');
insert into ingredients values (5, 'Garlic', '', 'garlic');
insert into ingredients values (6, 'Rice', 'For risotto', 'rice');
insert into ingredients values (7, 'Rice', 'Basmati','rice');
insert into ingredients values (8, 'Frozen peas', '', 'legume');
insert into ingredients values (9, 'Salt', '','salt');
insert into ingredients values (10, 'Pepper', '', 'pepper');
insert into ingredients values (11, 'Parmesan', '', 'parmesan');
insert into ingredients values (12, 'Butter', 'Animal based', 'butter');
insert into ingredients values (13, 'Plant Oil', '', 'oil');
insert into ingredients values (14, 'Basil', '', 'basil');

--Ingredients mushroom risotto vegan
insert into ingredients values (15, 'Vegan white wine', '', 'wine');
insert into ingredients values (16, 'Vegan butter', '', 'butter');
insert into ingredients values (17, 'Vegan Parmesan', '','parmesan');

-- More ingredients
insert into ingredients values (18, 'Milk', '', 'milk');
insert into ingredients values (19, 'Egg', '', 'egg');
insert into ingredients values (20, 'Shellfish', '', 'fish');
insert into ingredients values (21, 'Soy', '', 'meat');
insert into ingredients values (22, 'Flour', '', 'flour');

--Alternative legumes
insert into ingredients values (23, 'Kidney beans', '', 'legumes');
insert into ingredients values (24, 'Chickpeas', '', 'legumes');
insert into ingredients values (25, 'Lentils', '', 'legumes');
insert into ingredients values (26, 'Testparmesan2', '', 'parmesan');

--Disliked ingridients
insert into ingredients values (27, 'Coriander', '', 'coriander');
insert into ingredients values (28, 'Olives', '', 'olives');
insert into ingredients values (29, 'Blue Cheese', '', 'cheese');
insert into ingredients values (30, 'Coffee', '', 'coffee');
insert into ingredients values (31, 'Peanuts', '', 'nuts');
insert into ingredients values (32, 'Tree nuts', '', 'nuts');

--Leberkaese
insert into ingredients values (33, 'Senf', '', 'senf');
insert into ingredients values (34, 'Coleslaw', '', 'salat');
insert into ingredients values (35, 'Radishes', '', 'radishes');
insert into ingredients values (36, 'Leberkaese', '', 'leberkaese');
insert into ingredients values (37, 'Laugenbroetchen', '', 'roll');

--Leberkaese Ersatz 
insert into ingredients values (38, 'Gluten free roll', '', 'roll');
-- + soy

--Coclate Brownie
insert into ingredients values (39, 'Coconut oil', '', 'oil'); 
insert into ingredients values (40, 'Sugar', '', 'salat'); 
insert into ingredients values (41, 'Cocoa Powder', '', 'cocoa');  
insert into ingredients values (42, 'Banana', '', 'egg'); 
insert into ingredients values (43, 'flour', '', 'roll'); 
insert into ingredients values (44, 'Chocolate', '', 'choc'); 
insert into ingredients values (44, 'Baking powder', '', 'bakpow'); 

--Dumplings
insert into ingredients values (45, 'roll', '', 'roll'); 
insert into ingredients values (46, 'Petersilie', '', 'herbs'); 
insert into ingredients values (47, 'Semmelbrösel', '', 'roll');


--_______________________________________________________


-- AMOUNT AND ADD INGREDIENTS TO RECIPE
-- Unique, receipe id, ingridient id, amount, unit  
--Mushroom Risotto
insert into recipe_ingredients values (1, 1, 1, 500, 'g');
insert into recipe_ingredients values (2, 1, 2, 100, 'g');
insert into recipe_ingredients values (3, 1, 3, 1000, 'ml');
insert into recipe_ingredients values (4, 1, 4, 1, 'pcs');
insert into recipe_ingredients values (5, 1, 5, 2, 'pcs');
insert into recipe_ingredients values (6, 1, 6, 200, 'g');
insert into recipe_ingredients values (7, 1, 7, 200, 'g');
insert into recipe_ingredients values (8, 1, 8, 100, 'g');
insert into recipe_ingredients values (9, 1, 9, 5, 'g');
insert into recipe_ingredients values (10, 1, 10, 8, 'g');
insert into recipe_ingredients values (11, 1, 11, 100, 'g');
insert into recipe_ingredients values (12, 1, 12, 50, 'g');
insert into recipe_ingredients values (13, 1, 13, 50, 'ml');
insert into recipe_ingredients values (14, 1, 14, 20, 'g');

insert into recipe_ingredients values (15, 3, 33, 7, 'g');
insert into recipe_ingredients values (16, 3, 34, 50, 'g');
insert into recipe_ingredients values (17, 3, 35, 2, 'pcs');
insert into recipe_ingredients values (18, 3, 36, 2, 'pcs');
insert into recipe_ingredients values (19, 3, 37, 1, 'pcs');

-- FOOD GROUPS id, id ingredient, food group id (1, 'Lactose',
-- 2, 'Gluten' 3, 'Histamines', 4, 'Seafood')
insert into ingredient_food_groups values(1, 4, 3);
insert into ingredient_food_groups values(2, 5, 3);
insert into ingredient_food_groups values(3, 6, 3);
insert into ingredient_food_groups values(4, 8, 3);
insert into ingredient_food_groups values(5, 11, 3);
insert into ingredient_food_groups values(6, 11, 1);
insert into ingredient_food_groups values(7, 12, 1);
insert into ingredient_food_groups values(8, 15, 3);
insert into ingredient_food_groups values(9, 18, 1);
insert into ingredient_food_groups values(10, 19, 1);
insert into ingredient_food_groups values(11, 20, 4);
insert into ingredient_food_groups values(12, 21, 3);
insert into ingredient_food_groups values(13, 22, 1);
insert into ingredient_food_groups values(14, 26, 1);

--special diet
--id, ingredient_id INTEGER NOT NULL, special_diet_id
--(1, 'Low Carb'),(2, 'High Carb'),(3, 'High Fat'),(4,'Clean Eating');
insert into ingredient_special_diets values(1, 34, 1);
insert into ingredient_special_diets values(2, 37, 2);
insert into ingredient_special_diets values(3, 36, 3);
insert into ingredient_special_diets values(4, 11, 1);
insert into ingredient_special_diets values(5, 35, 1);
insert into ingredient_special_diets values(6, 38, 2);
insert into ingredient_special_diets values(7, 1, 4);
insert into ingredient_special_diets values(8, 33, 4);
insert into ingredient_special_diets values(10, 16, 3);



-- diet gols id, ingredient_id INTEGER NOT NULL, diet_goal_id INT
-- (1, 'Vegan'),(2, 'Vegetarian'),(3, 'Pescetarian'),(4, 'Keto');
insert into ingredient_diet_goals values(1, 33, 1);
insert into ingredient_diet_goals values(2, 34, 1);
insert into ingredient_diet_goals values(3, 35, 1);
insert into ingredient_diet_goals values(4, 36, 4);
insert into ingredient_diet_goals values(5, 37, 2);

--hated foods
