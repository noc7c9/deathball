use macroquad::prelude::*;

use crate::Resources;

pub struct Background {
    clear_color: Color,
    offset: Vec2,
    props: Vec<(Vec2, PropSprite)>,
}

impl Background {
    pub fn new(clear_color: Color, offset: Vec2, props: Vec<((u32, u32), Prop)>) -> Self {
        Self {
            clear_color,
            offset,
            props: props
                .into_iter()
                .map(|((x, y), prop)| (vec2(x as f32, y as f32), prop.to_data()))
                .collect(),
        }
    }

    pub fn draw(&self, res: &Resources) {
        let tile_size = res.assets.props.cell_size;

        clear_background(self.clear_color);

        for (pos, prop) in &self.props {
            res.assets
                .props
                .multisprite(prop.position, prop.size)
                .draw_top_right(*pos * tile_size + self.offset);
        }
    }
}

#[derive(Clone, Copy)]
pub enum Prop {
    Grass1,
    Grass2,
    Grass3,
    FlowerWhite,
    FlowerYellow,
    FlowerRed,
    FlowerBlack,
    Gravel1,
    Gravel2,
    Gravel3,
    Mud,
    Hay,
    Eggplant,
}

struct PropSprite {
    position: Vec2,
    size: Vec2,
}

impl Prop {
    fn to_data(self) -> PropSprite {
        match self {
            Prop::Grass1 => PropSprite::sprite(0., 0.),
            Prop::Grass2 => PropSprite::sprite(1., 0.),
            Prop::Grass3 => PropSprite::sprite(2., 0.),
            Prop::FlowerWhite => PropSprite::sprite(3., 0.),
            Prop::FlowerYellow => PropSprite::sprite(7., 0.),
            Prop::FlowerRed => PropSprite::sprite(6., 1.),
            Prop::FlowerBlack => PropSprite::sprite(7., 1.),
            Prop::Gravel1 => PropSprite::sprite(3., 1.),
            Prop::Gravel2 => PropSprite::sprite(4., 1.),
            Prop::Gravel3 => PropSprite::sprite(5., 1.),
            Prop::Mud => PropSprite::multisprite(4., 0., 3., 1.),
            Prop::Hay => PropSprite::multisprite(0., 1., 3., 3.),
            Prop::Eggplant => PropSprite::sprite(3., 2.),
        }
    }
}

impl PropSprite {
    fn sprite(x: f32, y: f32) -> Self {
        PropSprite {
            position: vec2(x, y),
            size: vec2(1., 1.),
        }
    }

    fn multisprite(x: f32, y: f32, w: f32, h: f32) -> Self {
        PropSprite {
            position: vec2(x, y),
            size: vec2(w, h),
        }
    }
}
