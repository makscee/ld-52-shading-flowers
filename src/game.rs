use super::*;

pub struct Game {
    pub geng: Geng,
    pub logic: Rc<Logic>,
    pub model: Rc<Model>,
    pub assets: Rc<Assets>,
    pub view: Rc<View>,
    pub state: StateManager,
}
