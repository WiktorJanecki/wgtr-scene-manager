use wgtr_scene_manager::wgtr;

struct First<'a>{
    x: &'a mut i32,
}

struct Second{

}
impl wgtr::Scene for Second{
    fn init(&mut self){
        println!("Second scene has started!");
    }
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
        println!("The value of x in first scene is {}", self.x);
        if *self.x == 10 {
            return wgtr::Transition::Next(Box::new(Second{}))
        }
        *self.x += 1;
        return wgtr::Transition::Skip;
    }
}
fn main() {
    let mut x = 5;
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
        println!("Gameloop has ended!");
    }
    println!("after gameloop x is equal to: {}", x);
    assert_eq!(x,10);
}