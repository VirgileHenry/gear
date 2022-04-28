use super::super::objects::components::component::{
    ComponentTable,
    Component,
};

pub struct GearObject {
    id: u32,
    // TODO : childrens ?
    // TODO : components !
}

impl GearObject {
    pub fn empty(id: u32, component_table: &mut ComponentTable) -> GearObject {
        // creates an empty gearObject
        let result = GearObject {
            id: id,
        };

        use super::super::objects::components::transform::Transform;
        // every gearobject have a transform
        result.add_component::<Transform>(component_table);

        return result;
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }

    pub fn add_component<C: Component>(&self, component_table: &mut ComponentTable) {
        component_table.add_component_to::<C>(self.id);
    }

    pub fn get_component<'a, C: Component>(&self, component_table: &'a mut ComponentTable) -> Option<&'a C> {
        return component_table.get_component_on::<C>(self.id);
    }

    pub fn remove_component<C: Component>(&self, component_table: &mut ComponentTable) {
        component_table.remove_component_on::<C>(self.id);
    }


}