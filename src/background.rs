use macroquad::prelude::*;

use crate::Resources;

// magic value that makes the sizes match
const CAMERA_ZOOM_FACTOR: f32 = 2.0;

pub struct Background {
    clear_color: Color,
    offset: Vec2,
    texture: Texture2D,
}

impl Background {
    pub fn builder(clear_color: Color, size: (u32, u32)) -> BackgroundBuilder {
        BackgroundBuilder::new(clear_color, size)
    }

    pub fn draw(&self) {
        clear_background(self.clear_color);
        draw_texture(self.texture, self.offset.x, self.offset.y, WHITE);
    }
}

pub struct BackgroundBuilder {
    clear_color: Color,
    props: Vec<Option<Prop>>,
    offset: Option<Vec2>,
    size: (u32, u32),
}

impl BackgroundBuilder {
    fn new(clear_color: Color, size: (u32, u32)) -> Self {
        BackgroundBuilder {
            clear_color,
            props: vec![None; (size.0 * size.1) as usize],
            offset: None,
            size,
        }
    }

    pub fn set_props(mut self, props: &[((u32, u32), Prop)]) -> Self {
        for &(xy, prop) in props {
            self = self.set_prop(xy, prop);
        }
        self
    }

    pub fn set_prop(mut self, (x, y): (u32, u32), prop: Prop) -> Self {
        let idx = y * self.size.0 + x;
        self.props[idx as usize] = Some(prop);
        self
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(self, res: &Resources) -> Background {
        let tile_size = res.assets.props.cell_size;

        let w = self.size.0 * tile_size as u32;
        let h = self.size.1 * tile_size as u32;
        let render_target = render_target(w, h);

        let w = w as f32;
        let h = h as f32;

        let zoom = CAMERA_ZOOM_FACTOR / w;
        set_camera(&Camera2D {
            render_target: Some(render_target),
            zoom: vec2(zoom, zoom * w / h),
            ..Default::default()
        });

        clear_background(self.clear_color);

        for (idx, prop) in self.props.iter().enumerate() {
            if let Some(prop) = prop.map(Prop::to_data) {
                let w = self.size.0 as usize;
                let h = self.size.1 as usize;
                let x = (idx % w) as f32 - (w as f32 / 2.);
                let y = (idx / w) as f32 - (h as f32 / 2.);
                res.assets
                    .props
                    .multisprite(prop.position, prop.size)
                    .draw_top_right(vec2(x, y) * tile_size);
            }
        }

        set_default_camera();

        render_target.texture.set_filter(FilterMode::Nearest);

        Background {
            clear_color: self.clear_color,
            offset: self.offset.unwrap_or_else(|| vec2(w / -2., h / -2.)),
            texture: render_target.texture,
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
