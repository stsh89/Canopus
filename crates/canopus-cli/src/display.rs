pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, data: impl std::fmt::Display) {
        println!("{}", data)
    }
}
