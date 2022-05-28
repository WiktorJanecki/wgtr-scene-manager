pub mod wgtr {
    pub enum Transition {
        Next(Box<dyn Scene>),
        Skip,
        Exit,
    }

    pub trait Scene {
        fn init(&mut self) {}
        fn render(&mut self) {}
        fn update(&mut self) -> Transition {
            return Transition::Skip;
        }
    }
    pub fn simple_loop(scene:Box<dyn Scene>) {
        let mut actual = scene;

        'scene_loop: loop {
            actual.init();
            loop {
                // Game loop
                match actual.update() {
                    Transition::Next(next) => {
                        actual = next;
                        break;
                    }
                    Transition::Exit => break 'scene_loop,
                    Transition::Skip => {}
                }
                actual.render();
            }
        }
    }
}
