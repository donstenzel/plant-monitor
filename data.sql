INSERT INTO [PlantTypes] (
  name,
  scientific,
  description,
  ml_per_day,
  ideal_ph,
  ideal_temp,
  ideal_hum,
  ideal_ec,
  ideal_lux,
  image
)
VALUES
  ( 'Dandelion',
    'Taraxacum officinale',
    'A common yellow flower.',
    10, 7.1, 16.9, 51, 1.4, 20000,
    'https://minecraft.wiki/images/Dandelion_JE7_BE4.png?2d25a'),

  ( 'Poppy',
    'Papaver somniferum',
    'A red flower whose seeds can be used for food',
    11, 6.7, 17.5, 44, 1.0, 24000,
    'https://minecraft.wiki/images/Poppy_JE8_BE2.png?39ac9'),

  ( 'Blue Orchid',
    'Vanda coerulea',
    'A blue orchid is a rare flower that grows naturally in wetlands. It can be used to produce light blue dye and to create certain special herbal mixtures.',
    45, 5.8, 24, 75, 1.2, 15000,
    'https://minecraft.wiki/images/Blue_Orchid_JE7_BE2.png?f8a0f&format=original'),

  ( 'Allium',
    'Allium giganteum',
    'An allium is a vibrant purple flower that can be used to create magenta dye and enhance certain special recipes.',
    25, 6.5, 18, 55, 1.1, 22000,
    'https://minecraft.wiki/images/Allium_JE7_BE2.png?44096'),

  ( 'Red Tulip',
    'Tulipa gesneriana',
    'Tulips are colorful flowers found in open meadows and forests. They come in red, orange, white, and pink varieties and can be used to produce matching dyes.',
    18, 6.4, 16, 50, 1.0, 21000,
    'https://minecraft.wiki/images/Red_Tulip_JE7_BE2.png?3301b'),

  ( 'Cornflower',
    'Centaurea cyanus',
    'A cornflower is a bright blue flower used to make blue dye and other crafted mixtures.',
    20, 6.3, 17, 52, 1.05, 20000,
    'https://minecraft.wiki/images/Cornflower_JE1_BE1.png?d9019'),

  ( 'Cactus',
    'Carnegiea gigantea',
    'A spiky green plant that prefers arid conditions like deserts.',
    100, 6.8, 29, 30, 1.2, 34000,
    'https://minecraft.wiki/images/Cactus_JE4.png?12482');

INSERT INTO [PotTypes] (name, drainage, volume, image)
VALUES ('Clay Pot', 10, 19, 'https://minecraft.wiki/images/Flower_Pot_JE3.png?e781b'),
       ('Cloth Pot', 18, 30, 'file://images/cloth_pot.png'),
       ('Marble Tub', 5, 50, 'file://images/marble_tub.png'),
       ('Plastic Pot', 14, 24, 'file://images/plastic_pot.png'),
       ('Wicker Basket', 25, 22, 'file://images/wicker_basket.png');

INSERT INTO [Usages] (plant, pot, planted)
VALUES (1, 1, datetime('2026-01-09 00:00:00')),
       (3, 1, datetime('2026-01-09 00:00:00')),
       (4, 1, datetime('2026-01-09 00:00:00')),
       (6, 1, datetime('2026-01-09 00:00:00')),

       (1, 2, datetime('2026-01-09 00:00:00')),
       (2, 2, datetime('2026-01-09 00:00:00')),
       (4, 2, datetime('2026-01-09 00:00:00')),
       (5, 2, datetime('2026-01-09 00:00:00')),
       (6, 2, datetime('2026-01-09 00:00:00')),

       (7, 3, datetime('2026-01-09 00:00:00')),
       (7, 3, datetime('2026-01-09 00:00:00')),

       (1, 4, datetime('2026-01-09 00:00:00')),
       (2, 4, datetime('2026-01-09 00:00:00')),
       (3, 4, datetime('2026-01-09 00:00:00')),
       (4, 4, datetime('2026-01-09 00:00:00')),
       (5, 4, datetime('2026-01-09 00:00:00'));
