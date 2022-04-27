use super::transform::Transform;
use crate::objects::scene::GameScene;

pub struct GearObject<'o, 's:'o> {
    transform: Transform,
    id: u32,
    // TODO : childrens ?
    // TODO : components !
    get_scene: &'o dyn Fn() -> &'s mut GameScene,
}

impl<'s:'o, 'o> GearObject<'o, 's> {
    pub fn empty(id: u32, scene_borrow: &'s mut GameScene) -> GearObject<'o, 's> {
        // creates an empty gearObject
        return GearObject {
            id: id,
            transform: Transform::origin(),
            get_scene: &|| scene_borrow,
        };
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }


    pub fn destroy(&mut self) {
        
    }
}