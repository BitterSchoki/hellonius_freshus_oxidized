--RECEIPS
insert into recipes values (1, 'Mushroom-Risotto', 'A creamy italian dish', 2);
insert into recipes values (2, 'Noodles with salmon cream sauce', 'A healthy noode receipe', 2);

--INGREDIENTS
--Ingridients Mushroom Risotto
insert into ingredients values (1, 'Mushrooms', 'Stone');
insert into ingredients values (2, 'Mushrooms', 'Normal');
insert into ingredients values (3, 'Water', 'Tap');
insert into ingredients values (4, 'Onions', '');
insert into ingredients values (5, 'Garlic', '');
insert into ingredients values (7, 'Rice', 'For risotto');
insert into ingredients values (8, 'Rice', 'Basmati');
insert into ingredients values (9, 'Freezed peas', '');
insert into ingredients values (10, 'Salt', '');
insert into ingredients values (11, 'Pepper', '');
insert into ingredients values (12, 'Parmesan', '');
insert into ingredients values (13, 'Butter', '');
insert into ingredients values (14, 'Plant Oil', '');
insert into ingredients values (15, 'Basil', '');

--Ingridients Mushroom Risotto Vegan
insert into ingredients values (16, 'Vegan white wine', '');
insert into ingredients values (17, 'Vegan butter', '');
insert into ingredients values (18, 'Vegan Pramesan', '');

--Uniqueid, receipe id, ingridient id, anzahl,einheit 
insert into recipe_ingredients values (1, 1, 3, 500, 'ml');
insert into recipe_ingredients values (2, 2, 1, 200, 'g');
insert into recipe_ingredients values (3, 2, 2, 1.5, 'pcs');

-- FOOD GROUPS
insert into ingredient_food_groups values()
