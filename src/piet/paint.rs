extern crate graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;

use std::sync::mpsc;

pub type SendChannel = mpsc::Sender<(types::Rectangle, types::Color)>;

pub fn paint_rectangle(x :f64, y :f64, width :f64, height :f64, c : types::Color, chn: &SendChannel)
{
    chn.send( ([x, y, width, height], c) ).unwrap();
}
