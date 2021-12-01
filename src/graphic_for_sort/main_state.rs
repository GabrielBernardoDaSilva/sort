use ggez::{
    event::{self, KeyCode, KeyMods},
    graphics::{self, Color, Rect},
    Context, GameResult,
};
use glam::Vec2;
use rand::Rng;

use crate::sort::{
    bubble_sort::{bubble_sort, bubble_sort_visualizer},
    selection_sort::{selection_sort, selection_sort_bar, selection_sort_visualizer},
};

use super::{
    bars::{self, Bar},
    util::{self, ControlIndex, Shuffle},
};

pub struct MainState {
    pub control_index: ControlIndex,
    pub bars: Vec<Bar>,
    pub trigger_shuffle: bool,
    pub trigger_sort: bool,
    pub center_height: f32,
    pub center_width: f32,
    pub min: usize
}

impl MainState {
    pub fn new(bars: Vec<Bar>) -> GameResult<MainState> {
        Ok(MainState {
            control_index: ControlIndex::default(),
            trigger_shuffle: false,
            trigger_sort: false,
            bars,
            center_width: 1920.0 / 2.0,
            center_height: 1080.0 / 2.0,
            min: 0
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.trigger_shuffle {
            self.bars.shuffle();
            self.trigger_shuffle = false;
            self.trigger_sort = false;
            self.control_index.j = 0;
            self.control_index.i = 0;
        }
        if self.trigger_sort {
            selection_sort_visualizer(self);           
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for bar in &self.bars {
            let rectangle_mesh = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0.0, 0.0, bar.width, bar.height),
                bar.color,
            )?;

            graphics::draw(ctx, &rectangle_mesh, (Vec2::new(bar.pos_x, bar.pos_y),))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Q {
            self.trigger_shuffle = true;
        }

        if keycode == KeyCode::E {
            self.trigger_sort = true;
        }

        if keycode == KeyCode::Escape{
            event::quit(ctx);
        }
    }
}
