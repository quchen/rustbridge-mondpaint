#![allow(dead_code)]

use piston_window::types;

mod paint;

pub enum SplitDirection {
    Vertical,
    Horizontal
}

pub enum Pietree {
    Paint {color: types::Color},
    Split { subtree1:        Box<Pietree>
          , subtree2:        Box<Pietree>
          , split_direction: SplitDirection
          , split_fraction:  f64 }
}

#[derive(Clone, Copy)]
pub struct Canvas {
    pub x : f64,
    pub y : f64,
    pub w : f64,
    pub h : f64
}

pub fn draw_tree(tree : Pietree, canvas : Canvas, channel : &paint::SendChannel) -> () {
    match tree {
        Pietree::Paint {color} =>
            paint::paint_rectangle(
                canvas.x,
                canvas.y,
                canvas.w,
                canvas.h,
                color,
                channel),
        Pietree::Split {subtree1, subtree2, split_direction, split_fraction} => {
            let (canvas1, canvas2) = {
                let mut c1 = canvas.clone();
                let mut c2 = canvas.clone();
                match split_direction {
                    SplitDirection::Horizontal => {
                        c1.h = canvas.h * split_fraction;
                        c2.y = canvas.y + c1.h;
                        c2.h = canvas.h - c1.h;
                    },
                    SplitDirection::Vertical => {
                        c1.w = canvas.w * split_fraction;
                        c2.x = canvas.x + c1.w;
                        c2.w = canvas.w - c1.w
                    }
                };
                (c1, c2)
            };
            draw_tree(*subtree1, canvas1, channel);
            draw_tree(*subtree2, canvas2, channel);
        }
    };
}
