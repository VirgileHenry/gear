use super::transform::Transform;

pub struct GearObject {
    transform: Transform,
    id: u32,
    // TODO : childrens ?
    // TODO : components !
}

impl<'c> GearObject {
    pub fn empty(id: u32) -> GearObject {
        // creates an empty gearObject
        return GearObject {
            id: id,
            transform: Transform::origin(),
        };
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }


    pub fn destroy(&mut self) {
        
    }
}