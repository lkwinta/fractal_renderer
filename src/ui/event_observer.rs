use std::cell::RefCell;
use std::rc::Rc;

/**
    File containing the Observer pattern traits and definitions
*/

pub enum FractalType {
    Julia([f32; 2]),
    Mandelbrot
}

/// All possible events that can be observed in the application
pub enum ObserverEvent {
    Zoom(f32),
    UnZoom(f32),
    Translate{xrel: i32, yrel: i32},
    WindowSizeChanged{width: i32, height: i32},
    FractalIterations(i32),
    FractalChoice(FractalType),
    FractalAxisRange{x: [f32; 2], y: [f32; 2]},
    FractalHSVScaleChange{h: f32, s: f32, v: f32},
    FractalTerminalColorChange{r: f32, g: f32, b: f32},
}

/// Observer trait for observer pattern
/// Observers are notified of events
pub trait Observer {
    fn notify(&mut self, event: &ObserverEvent);
}

/// Observable trait for observer pattern
/// Observables can register observers and notify them of events
pub trait Observable<'a> {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&mut self, event: ObserverEvent);
}

