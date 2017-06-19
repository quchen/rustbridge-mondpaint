extern crate graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;

use rand::Rng;

use std::thread;
use std::sync::mpsc;

mod piet;
use piet::*;

const WHITE:  [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK:  [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const GREEN1: [f32; 4] = [0.20, 0.53, 0.18, 1.0];
const GREEN2: [f32; 4] = [0.25, 0.68, 0.22, 1.0];
const GREEN3: [f32; 4] = [0.35, 0.78, 0.32, 1.0];

const RED1:   [f32; 4] = [0.47, 0.17, 0.16, 1.0];
const RED2:   [f32; 4] = [0.62, 0.23, 0.21, 1.0];
const RED3:   [f32; 4] = [0.76, 0.29, 0.27, 1.0];

const BLUE1:  [f32; 4] = [0.16, 0.44, 0.43, 1.0];
const BLUE2:  [f32; 4] = [0.20, 0.59, 0.57, 1.0];
const BLUE3:  [f32; 4] = [0.25, 0.74, 0.73, 1.0];

fn main() {
    let (paintsend, paintrecv) = mpsc::channel();

    let serverthread = thread::spawn(move || {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Construct the window.
        let mut window: PistonWindow =
            WindowSettings::new("RustBridge / Mondrian Pattern Generator", [500, 500])
                .opengl(opengl).samples(4).exit_on_esc(true).build().unwrap();
        window.set_ups(60);

        let mut canvas: Vec<(types::Rectangle, types::Color)> = Vec::new();
        while let Some(e) = window.next() {
            if let Ok( tobepainted ) = paintrecv.try_recv() {
                canvas.push( tobepainted );
            }
            window.draw_2d(&e, |c, gl| {
                clear(WHITE, gl);
                let redrect = graphics::Rectangle::new(WHITE).border(graphics::rectangle::Border{color :BLACK, radius :1.0});
                for item in (&canvas).iter() {
                    let (rct, col) = *item;
                    redrect.color(col).draw(rct, &c.draw_state, c.transform, gl);
                }
            });
        }
    });
    let chn = paintsend.clone();
    let painterthread = thread::spawn(move || {
        let canvas = piet::Canvas
            { x: 20.0
            , y: 20.0
            , h: 500.0 - 40.0
            , w: 500.0 - 40.0 };
        println!("Generating tree");
        let tree = random_piet_tree();
        println!("Painting tree");
        draw_tree(tree, canvas, &chn);
        println!("Done", );
        }
    );
    painterthread.join().unwrap();
    serverthread.join().unwrap();
}

fn random_piet_tree() -> Pietree {
    random_piet_tree_rng(1.0, 0.95, &mut rand::thread_rng())
}

fn random_piet_tree_rng( split_likelihood: f64, depth_cutoff: f64, rng : &mut rand::ThreadRng ) -> Pietree {
    if random_bool(split_likelihood, rng) {
        let direction = if random_bool(0.5, rng) {
            piet::SplitDirection::Vertical
        } else {
            piet::SplitDirection::Horizontal
        };
        let split_fraction = rng.gen_range(0.2, 0.8);
        let recurse = |rng : &mut rand::ThreadRng| {random_piet_tree_rng(split_likelihood * depth_cutoff, split_likelihood, rng)};
        Pietree::Split {
            subtree1:        Box::new(recurse(rng)),
            subtree2:        Box::new(recurse(rng)),
            split_direction: direction,
            split_fraction:  split_fraction
        }
    } else {
        let mut color = WHITE;
        if random_bool(0.4, rng) {
            let colours = [ GREEN1, GREEN2, GREEN3
                          , RED1, RED2, RED3
                          , BLUE1, BLUE2, BLUE3 ];
            color = *rng.choose(&colours).unwrap();
        }
        Pietree::Paint { color: color }
    }
}

fn random_bool(likelihood: f64, rng: &mut rand::ThreadRng) -> bool {
    rng.gen_range(0.0, 1.0) <= likelihood
}
