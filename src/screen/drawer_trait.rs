use crate::world::World;

pub trait DrawerTrait {
    fn draw(&self, world: &World);
}
