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