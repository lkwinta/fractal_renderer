use std::cell::RefCell;
use std::rc::Weak;

pub enum FractalType {
    Julia([f32; 2]),
    Mandelbrot
}

pub enum StateInstruction {
    Close,
    Zoom(f32),
    UnZoom(f32),
    Translate{xrel: i32, yrel: i32},
}

pub enum FractalInstruction {
    FractalIterations(i32),
    FractalChoice(FractalType),
    FractalAxisRange{x: [f32; 2], y: [f32; 2]},
}

pub enum ObserverEvent {
    Close,
    Zoom(f32),
    UnZoom(f32),
    Translate{xrel: i32, yrel: i32},
    FractalIterations(i32),
    FractalChoice(FractalType),
    FractalAxisRange{x: [f32; 2], y: [f32; 2]},
}

pub trait Observer {
    fn notify(&mut self, event: &ObserverEvent);
}

pub trait Observable<'a> {
    fn register(&mut self, observer: &'a RefCell<dyn Observer>);
}

