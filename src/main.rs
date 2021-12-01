#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use std::thread;
use std::time;

use ggez::{conf, event, GameResult};
use graphic_for_sort::{bars::Bar, main_state::MainState};
use rand::{prelude::SliceRandom, Rng};

use crate::sort::{
    bubble_sort::bubble_sort, quick_sort::quick_sort, selection_sort::selection_sort,
};

mod graphic_for_sort;
mod sort;

static mut BARS: Vec<Bar> = Vec::new();

fn create_bars() -> Vec<Bar> {
    let mut bars = Vec::new();
    let width_screen: f32 = 1920.0;
    let height_screen: f32 = 1080.0;
    let step_x: f32 = 1920.0 / 60.0;

    let mut pos_x: f32 = 0.0;
    let mut height: f32 = -90.0;

    let mut r: f32 = 0.2;
    let mut g: f32 = 0.0;
    let mut b: f32 = 0.0;

    for i in 0..65 {
        if i < 13 {
            r += 0.061;
        } else if i == 13 {
            r = 1.0;
        } else if i > 13 && i < 26 {
            r -= 0.076;
            g += 0.076;
        } else if i == 26 {
            r = 0.0;
            g = 1.0;
        } else if i > 26 && i < 39 {
            g -= 0.076;
            b += 0.076;
        } else if i == 39 {
            g = 0.0;
            b = 1.0;
        } else if i > 39 && i < 52 {
            r += 0.076;
        } else if i == 52 {
            r = 1.0;
        } else if i > 52 && i < 65 {
            g += 0.076;
        } else if i == 65 {
            g = 1.0;
        }

        bars.push(Bar::new(
            i,
            pos_x,
            height_screen,
            height,
            ggez::graphics::Color::new(r, g, b, 1.0),
        ));
        height -= 15.5;
        pos_x += 30.0;
    }

    bars
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("sort_2d_visualiazer", "gabriel")
        .window_setup(ggez::conf::WindowSetup::default().title("Sort Visualizer"))
        .window_mode(conf::WindowMode {
            width: 1920.0,
            height: 1080.0,
            maximized: true,
            fullscreen_type: conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 720.0,
            min_height: 460.0,
            max_width: 1920.0,
            max_height: 1080.0,
            resizable: true,
            visible: true,
            resize_on_scale_factor_change: true,
        });
    let (ctx, event_loop) = cb.build()?;
    unsafe {
        BARS = create_bars();
    }
    let state = MainState::new(create_bars())?;

    event::run(ctx, event_loop, state)
}

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(time::Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(time::Duration::from_millis(1));
//     }

// }
