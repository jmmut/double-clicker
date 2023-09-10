use crate::world::World;

pub trait DrawerTrait {
    fn draw(&mut self, world: &World);
}
