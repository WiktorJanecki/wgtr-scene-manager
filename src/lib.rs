pub mod wgtr {

    /// The [Transition] enum is used to move between states in the update function.
    pub enum Transition {
        /// by returning Transition::Next we are able to switch to the next scene while aborting the current one
        Next(Box<dyn Scene>),
        /// returning skip will just skip the update function (can be helpful at the end of the update function)
        Skip,
        /// by returning exit we will be able to break the game loop
        Exit,
    }

    /// All scenes that we want to create must implement this [Scene] struct.
    /// # Examples:
    /// ```
    /// use wgtr_scene_manager::wgtr::*;
    /// struct GameMenu {
    ///     status: u32,
    /// }
    /// type AnotherScene = GameMenu;
    /// impl Scene for GameMenu {
    ///     fn init(&mut self) {
    ///         self.status = 1 // 1 stands for another menu
    ///     }
    ///     fn render(&mut self) {
    ///         // render our menu
    ///     }
    ///     fn update(&mut self) -> Transition {
    ///         if self.status == 0 { // 0 stands for exit menu
    ///             return Transition::Exit; 
    ///         }
    ///         else if self.status == 1 {
    ///             return Transition::Next(Box::new(AnotherScene{status:0}));
    ///         }
    ///         else{
    ///             return Transition::Skip;
    ///         }
    ///     }
    /// }
    /// fn main(){
    ///     let mut gm: Box<dyn Scene> = Box::new(GameMenu{status : 5});
    ///     gm.init();
    ///     loop{
    ///         // Game loop
    ///         match gm.update() {
    ///             Transition::Next(next) => {
    ///                 gm = next;
    ///                 break;
    ///             }
    ///             Transition::Skip => {}
    ///             Transition::Exit => {break;}
    ///         }
    ///         gm.render();
    ///     }
    /// }
    /// ```
    /// Note that only first state was "initialized". The proper gameloop would look something like this
    /// ```ignore
    /// let mut actual = Box::new(MenuState{}); // MenuState from previous example
    /// 'scene_loop: loop {
    ///     actual.init();
    ///     loop {
    ///         // Game loop
    ///         match actual.update() {
    ///             Transition::Next(next) => {
    ///                 actual = next;
    ///                 break;
    ///             }
    ///             Transition::Exit => break 'scene_loop,
    ///             Transition::Skip => {}
    ///         }
    ///         actual.render();
    ///     }
    /// }
    /// ```
    pub trait Scene {
        fn init(&mut self) {}
        fn render(&mut self) {}
        fn update(&mut self) -> Transition {
            return Transition::Skip;
        }
    }

    /// [simple_loop] can be just to run simple examples without events or other external functions that need to be executed inside of a game loop
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
