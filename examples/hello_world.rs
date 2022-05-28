use wgtr_scene_manager::wgtr::*;
struct MyScene;
impl Scene for MyScene {
    fn init(&mut self) {
        println!("Hello from Scene initialization function! This function will be called only once at the begining of the scene");
    }
    fn render(&mut self) {
        // here we would render our scene
    }
    fn update(&mut self) -> Transition {
        println!("Hello from Scene update function! This function will be called every game frame (but we will call it only once by exiting from game loop here)");
        return Transition::Exit;
    }
}
fn main(){
    let scene: Box<dyn Scene> = Box::new(MyScene{});
    simple_loop(scene);
    println!("Game loop has ended");
}
