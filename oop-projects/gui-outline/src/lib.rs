pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Give this a setter, maybe give it an f64 percentage(?), exact number of pixels(?)
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
    // hash map this?
}

impl Draw for Button {
    fn draw(&self) {
        // TODO: code to draw button
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // TODO: code to draw select box
    }
}


