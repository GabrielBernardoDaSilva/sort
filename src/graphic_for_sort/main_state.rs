use std::thread;

use ggez::{
    event::{self, KeyCode, KeyMods},
    graphics::{self, Color, Rect},
    Context, GameResult,
};
use glam::Vec2;
use rand::Rng;

use crate::{
    sort::{bubble_sort::bubble_sort, merge_sort, selection_sort::selection_sort},
    BARS,
};

use super::{
    bars::{self, Bar},
    sort_bar::{
        bubble_sort_bar::bubble_sort_bar, heapsort_bar, merge_sort_bar,
        quick_sort_bar::quick_sort_bar, radix_sort_bar::radix_sort_bar,
        selection_sort_bar::selection_sort_bar, insertion_sort_bar::insertion_sort_bar, cocktail_shaker_sort_bar::cocktail_shaker_sort_bar, gravity_sort_bar,
    },
    util::{self, ControlIndex, Shuffle},
};

pub struct MainState {
    pub control_index: ControlIndex,
    pub bars: Vec<Bar>,
    pub trigger_shuffle: bool,
    pub trigger_sort: bool,
    pub min: usize,
}

impl MainState {
    pub fn new(bars: Vec<Bar>) -> GameResult<MainState> {
        Ok(MainState {
            control_index: ControlIndex::default(),
            trigger_shuffle: false,
            trigger_sort: false,
            bars,
            min: 0,
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

            unsafe {
                BARS.shuffle();
            }
        }
        if self.trigger_sort {
            unsafe {
                thread::spawn(move || {
                    gravity_sort_bar::gravity_sort_bar(&mut BARS);
                });
                self.trigger_sort = false;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        unsafe {
            for bar in &BARS {
                let rectangle_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(0.0, 0.0, bar.width, bar.height),
                    bar.color,
                )?;

                graphics::draw(ctx, &rectangle_mesh, (Vec2::new(bar.pos_x, bar.pos_y),))?;
            }

            graphics::present(ctx)?;
        }
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

        if keycode == KeyCode::Escape {
            event::quit(ctx);
        }
    }
}
