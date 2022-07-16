use super::super::utils::app::*;
use super::super::render::*;
use super::scene::World;
pub struct Client {

}
impl Client {
    pub fn new(w: World) {
        let my_client = app::new();
        let camera = Camera::new(&my_client.screen);

    }
}