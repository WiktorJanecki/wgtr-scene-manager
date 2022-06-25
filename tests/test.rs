#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn scene_init(){
        use wgtr_scene_manager::*;
        struct First<'a>{
            x: &'a mut i32,
        }    
        impl <'a> Scene for First <'a>{
            fn init(&mut self){
                *self.x = 0;
            }
            fn render(&mut self){
        
            }
            fn update(&mut self) -> Transition{
                assert_eq!(*self.x,0);
                return Transition::Exit;
            }
        }
        let mut x = 20;
        {
            let mut actual: Box<dyn Scene> = Box::new(First{x:&mut x});
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
        assert_eq!(x,0);
    }
    #[test]
    fn scene_edit_variable() {
        use wgtr_scene_manager::*;
        struct First<'a>{
            x: &'a mut i32,
        }
        
        struct Second{
        
        }
        impl Scene for Second{
            fn init(&mut self){}
            fn update(&mut self) -> Transition{
                return Transition::Exit;
            }
        }
        
        impl <'a> Scene for First <'a>{
            fn init(&mut self){
                *self.x = 0;
            }
            fn render(&mut self){
        
            }
            fn update(&mut self) -> Transition{
                if *self.x == 10 {
                    return Transition::Next(Box::new(Second{}))
                }
                *self.x += 1;
                return Transition::Skip;
            }
        }
        let mut x = 11;
        {
            let mut actual: Box<dyn Scene> = Box::new(First{x:&mut x});
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
        assert_eq!(x,10);
    }
}
