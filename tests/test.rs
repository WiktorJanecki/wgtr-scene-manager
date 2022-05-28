#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn scene_init(){
        use wgtr_scene_manager::wgtr;
        struct First<'a>{
            x: &'a mut i32,
        }    
        impl <'a> wgtr::Scene for First <'a>{
            fn init(&mut self){
                *self.x = 0;
            }
            fn render(&mut self){
        
            }
            fn update(&mut self) -> wgtr::Transition{
                assert_eq!(*self.x,0);
                return wgtr::Transition::Exit;
            }
        }
        let mut x = 20;
        {
            let mut actual: Box<dyn wgtr::Scene> = Box::new(First{x:&mut x});
            'scene_loop: loop {
                actual.init();
                loop {
                    // Game loop
                    match actual.update() {
                        wgtr::Transition::Next(next) => {
                            actual = next;
                            break;
                        }
                        wgtr::Transition::Exit => break 'scene_loop,
                        wgtr::Transition::Skip => {}
                    }
                    actual.render();
                }
            }
        }
        assert_eq!(x,0);
    }
    #[test]
    fn scene_edit_variable() {
        use wgtr_scene_manager::wgtr;
        struct First<'a>{
            x: &'a mut i32,
        }
        
        struct Second{
        
        }
        impl wgtr::Scene for Second{
            fn init(&mut self){}
            fn update(&mut self) -> wgtr::Transition{
                return wgtr::Transition::Exit;
            }
        }
        
        impl <'a> wgtr::Scene for First <'a>{
            fn init(&mut self){
                *self.x = 0;
            }
            fn render(&mut self){
        
            }
            fn update(&mut self) -> wgtr::Transition{
                if *self.x == 10 {
                    return wgtr::Transition::Next(Box::new(Second{}))
                }
                *self.x += 1;
                return wgtr::Transition::Skip;
            }
        }
        let mut x = 11;
        {
            let mut actual: Box<dyn wgtr::Scene> = Box::new(First{x:&mut x});
            'scene_loop: loop {
                actual.init();
                loop {
                    // Game loop
                    match actual.update() {
                        wgtr::Transition::Next(next) => {
                            actual = next;
                            break;
                        }
                        wgtr::Transition::Exit => break 'scene_loop,
                        wgtr::Transition::Skip => {}
                    }
                    actual.render();
                }
            }
        }
        assert_eq!(x,10);
    }
}
