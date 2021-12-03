const fs = require('fs');
const util = require('util');

const OBJECTIVES_MAP = {
    1: 'kill_enemies',
    2: 'destroy_buildings',
    3: 'save_animals',
    4: 'kill_bosses',
};

const PROPS_MAP = {
    0: 'Grass1',
    1: 'Grass2',
    2: 'Grass3',
    3: 'FlowerWhite',
    7: 'FlowerYellow',
    23: 'FlowerRed',
    24: 'FlowerBlack',
    20: 'Gravel1',
    21: 'Gravel2',
    22: 'Gravel3',
    4: 'Mud',
    5: null,
    6: null,
    11: 'Hay',
    12: null,
    13: null,
    14: null,
    15: null,
    16: null,
    17: null,
    18: null,
    19: null,
    25: 'Eggplant',
};

const BUILDINGS_MAP = {
    barn: 'Barn',
    car: 'Car',
    concrete_wall_h: 'ConcreteWallH',
    concrete_wall_v: 'ConcreteWallV',
    down_with_horses: 'DownWithHorses',
    feeding_trough: 'FeedingTrough',
    fence: 'FenceH',
    fence_v: 'FenceV',
    garage: 'Garage',
    hay_bale_v: 'HayBaleH',
    hay_bale_h: 'HayBaleV',
    horse_crossing_sign: 'HorseCrossingSign',
    house_1: 'House1',
    house_2: 'House2',
    oil_barrel: 'OilBarrel',
    outhouse: 'Outhouse',
    portapotty: 'Portapotty',
    stable: 'Stable',
    stable_double: 'StableDouble',
    stable_wide: 'StableWide',
    stop_sign: 'StopSign',
    yield_sign: 'YieldSign',
};

const ANIMALS_MAP = {
    cat: 'Cat',
    dog: 'Dog',
    duck: 'Duck',
    brown_horse: 'Horse',
    kuma: 'Kuma',
    loaf: 'Loaf',
    mouse: 'Mouse',
    poop: 'Poop',
    rabbit: 'Rabbit',
    rubber_ducky: 'RubberDucky',
    snail: 'Snail',
    snake: 'Snake',
    turtle: 'Turtle',
};

const ENEMIES_MAP = {
    demon: 'Demon',
    demon_boss: 'DemonBoss',
    farmer: 'Farmer',
    police: 'Police',
    snowman: 'Snowman',
    soldier: 'Soldier',
};

main();

function index_to_xy(index) {
    const t15 = 2 ** 15;
    const t16 = 2 ** 16;
    const m15 = index % t15;
    const m16 = index % t16;
    const d15 = index / t15;
    const d16 = index / t16;
    const s = index < 0 ? -1 : 1;

    const x = Math.abs(m16) < t15 ? m16 % t15 : (m16 % t15) - s * t15;
    const y = Math.floor(d16);
    return [x, y];
}

function test(input, expected) {
    const actual = index_to_xy(input);
    if (actual[0] === expected[0] && actual[1] === expected[1]) {
        log('pass', input, expected);
    } else {
        log('FAIL', input, 'expected:', expected, 'actual:', actual);
    }
}

function tests() {
    test(0, [0, 0]);
    test(2, [2, 0]);
    test(131072, [0, 2]);
    test(196611, [3, 3]);
    test(983071, [31, 15]);

    test(65535, [-1, 0]);
    test(65533, [-3, 0]);
    test(262143, [-1, 3]);
    test(196604, [-4, 2]);
    test(1245160, [-24, 18]);

    test(-65536, [0, -1]);
    test(-196608, [0, -3]);
    test(-65534, [2, -1]);
    test(-458746, [6, -7]);
    test(-786402, [30, -12]);

    test(-1, [-1, -1]);
    test(-131073, [-1, -3]);
    test(-3, [-3, -1]);
    test(-524278, [10, -8]);
    test(-1507364, [-36, -24]);

    process.exit(1);
}

function main() {
    // tests();

    const args = process.argv.slice(2);

    const filename = args[0];
    const data = parse(filename);

    const level = {
        bgColor: [0.23, 0.39, 0.15, 1],
        props: [],
        buildings: [],
        animals: [],
        enemies: [],
    };

    log('Converting', filename);
    // log(`Updated ${new Date().toISOString()}`);

    {
        const { objective, objective_count } = data.find(
            (datum) => datum.objective != null,
        );
        const fn = OBJECTIVES_MAP[objective];
        level.objective = `${fn}(${objective_count})`;
    }

    /***
     * scenario opts
     */
    {
        const { id } = data.find(
            (datum) => datum.path === 'res://scenarios/base_scenario.tscn',
        );
        const needle = `ExtResource(${id})`;
        const scenario = data.find((datum) => datum.instance === needle);

        if (scenario.background_color) {
            level.bgColor = scenario.background_color
                .replace('Color(', '')
                .replace(')', '')
                .split(', ')
                .map(parseFloat);
        }
    }

    /***
     * props
     */

    const tilemap = data.find((datum) => datum.tile_data != null);
    const tiles = tilemap.tile_data
        .replace('PoolIntArray(', '')
        .replace(')', '')
        .split(',');
    for (let i = 0; i < tiles.length; i += 3) {
        const idx = parseInt(tiles[i].trim());
        const tile_idx = tiles[i + 1].trim();
        const position = index_to_xy(idx);
        const type = PROPS_MAP[tile_idx];
        if (type === undefined) {
            throw new Error(`Unknown prop: "${tile_idx}"`);
        }
        if (type != null) {
            level.props.push({ position, type });
        }
    }
    const { min, max } = Math;
    const minX = level.props.reduce((acc, t) => min(acc, t.position[0]), 0);
    const minY = level.props.reduce((acc, t) => min(acc, t.position[1]), 0);
    const maxX = level.props.reduce((acc, t) => max(acc, t.position[0]), 0);
    const maxY = level.props.reduce((acc, t) => max(acc, t.position[1]), 0);
    level.props = level.props.map((t) => ({
        ...t,
        position: [t.position[0] - minX, t.position[1] - minY],
    }));
    level.bgSize = [maxX - minX + 2, maxY - minY + 2];
    level.bgOffset = [minX * 32, minY * 32];

    /***
     * buildings
     */

    const buildingExts = {};
    data.forEach(({ id, path }) => {
        if (!path?.startsWith('res://entities/buildings/')) return;
        const filename = path
            .replace('res://entities/buildings/128/', '')
            .replace('_128.tscn', '');
        const name = BUILDINGS_MAP[filename];
        if (name == null) {
            throw new Error(`Unknown building: ${path}`);
        }
        buildingExts[`ExtResource(${id})`] = name;
    });
    data.forEach((datum) => {
        const type = buildingExts[datum.instance];
        if (type == null) return;
        const position = parse_vector2(datum.position);
        level.buildings.push({ type, position });
    });

    /***
     * animals
     */

    const animalExts = {};
    data.forEach(({ id, path }) => {
        if (!path?.startsWith('res://entities/player/')) return;
        const filename = path
            .replace('res://entities/player/', '')
            .replace('.tscn', '');
        const name = ANIMALS_MAP[filename];
        if (name == null) {
            throw new Error(`Unknown animal: ${path}`);
        }
        animalExts[`ExtResource(${id})`] = name;
    });
    data.forEach((datum) => {
        const type = animalExts[datum.instance];
        if (type == null) return;
        const position = parse_vector2(datum.position ?? 'Vector2(0, 0)');
        if (position[0] === 0 && position[1] === 0) {
            position[0] = 0.00001;
        }
        level.animals.push({ type, position });
    });

    /***
     * enemies
     */

    const enemyExts = {};
    data.forEach(({ id, path }) => {
        if (!path?.startsWith('res://entities/enemies/')) return;
        const filename = path
            .replace('res://entities/enemies/', '')
            .replace('.tscn', '');
        const name = ENEMIES_MAP[filename];
        if (name == null) {
            throw new Error(`Unknown animal: ${path}`);
        }
        enemyExts[`ExtResource(${id})`] = name;
    });
    data.forEach((datum) => {
        const type = enemyExts[datum.instance];
        if (type == null) return;
        const position = parse_vector2(datum.position ?? 'Vector2(0, 0)');
        level.enemies.push({ type, position });
    });

    console.log(printLevel(level));
}

function parse_vector2(value) {
    const [, x, y] = /Vector2\((-?\d+\.?\d*), ?(-?\d+\.?\d*)\)/.exec(value);
    return [parseFloat(x), parseFloat(y)];
}

function to_vec2([x, y]) {
    x = to_float(x);
    y = to_float(y);
    return `vec2(${x}, ${y})`;
}

function to_float(v) {
    const s = v.toString();
    if (/\./.test(s)) return s;
    return `${s}.0`;
}

function parse(filename) {
    const lines = fs.readFileSync(filename, 'utf8').split('\n');

    const processed = [];
    let curr = null;
    for (line of lines) {
        line = line
            .replace(/\( /g, '(')
            .replace(/ \)/g, ')')
            .replace(' = ', '=')
            .trim();

        if (line === '') continue;

        if (line.startsWith('[')) {
            if (curr != null) processed.push(curr);

            const [TYPE, ...props] = line.replace(/^\[|\]$/g, '').split(' ');
            curr = { TYPE };
            for (prop of props) {
                const [key, val] = prop.split('=');
                curr[key] = val.replace(/^"|"$/g, '');
            }
        } else {
            const [key, val] = line.split('=');
            if (val != null) {
                curr[key] = val.replace(/^"|"$/g, '');
            }
        }
    }

    return processed;
}

function log(...args) {
    process.stderr.write(
        args
            .map((a) => (typeof a === 'string' ? a : util.inspect(a)))
            .join(' '),
    );
    process.stderr.write('\n');
    return args[args.length - 1];
}

function printLevel({
    objective,
    bgColor,
    bgSize,
    bgOffset,
    props,
    animals,
    buildings,
    enemies,
}) {
    return `
use macroquad::prelude::*;

use crate::{
    animals::{ Animal, Variant::* },
    background::{Background, Prop::*},
    buildings::{ Building, Variant::* },
    enemies::{ Enemy, Variant::*},
    entities::Entities,
    levels::LevelData,
    objectives::Objective,
    Resources,
};

pub fn init(res: &mut Resources) -> LevelData {
    let objective = Objective::${objective};

    let background =
        Background::builder(Color::new(${bgColor.map(to_float).join(',')}),
        (${bgSize[0]}, ${bgSize[1]}))
        .offset(${to_vec2(bgOffset)})
        .set_props(&[
    ${props
        .map(({ type, position: [x, y] }) => `((${x}, ${y}), ${type}),`)
        .join('\n    ')}
        ])
            .build(res);

    let mut animals = Entities::new();
    ${animals
        .map(
            ({ type, position }) =>
                `animals.push(|idx| Animal::new(${type}, idx, res, ${to_vec2(
                    position,
                )}));`,
        )
        .join('\n    ')}

    let mut buildings = Entities::new();
    ${buildings
        .map(
            ({ type, position }) =>
                `buildings.push(|idx| Building::new(${type}, idx, res, ${to_vec2(
                    position,
                )}));`,
        )
        .join('\n    ')}

    let mut enemies = Entities::new();
    ${enemies
        .map(
            ({ type, position }) =>
                `enemies.push(|idx| Enemy::new(${type}, idx, res, ${to_vec2(
                    position,
                )}));`,
        )
        .join('\n    ')}

    LevelData {
        objective,
        background,
        animals,
        buildings,
        enemies,
    }
}
    `;
}
