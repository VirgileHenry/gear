use crate::objects::components::component::*;
use std::collections::HashMap;
use super::transform::Transform;
use crate::objects::scene::GameScene;

pub struct GearObject<'a> {
    parent_scene: &'a GameScene<'a>,
    pub transform: Transform,
    // TODO : childrens ?
    // TODO : components !
    components: HashMap<i32, &'a Box<dyn Component>>
}

impl<'a> GearObject<'a> {
    pub fn empty<'s>(scene: &'a GameScene) -> GearObject<'a> {
        // creates an empty gearObject
        return GearObject {
            parent_scene: scene,
            transform: Transform::origin(),
            components: HashMap::new(),
        };
    }

    pub fn add_component<C: 'static + Component>(&mut self) -> Option<&C> {
        // ask the scene to create a component in the table and keep a reference to it
        let component_ref = self.parent_scene.create_new_component::<C>();

        // how to return it and store it ?
        println!("WARNING -> using a unfinished function");
        return None;
    }

    pub fn get_component<C: Component>(&self) -> Option<&C> {
        // get the component from the array under the form of Box<dyn Trait>
        let option_boxed: Option<&&'a Box<dyn Component>> = self.components.get(&C::id());
        return match option_boxed {
            Some(boxed_component) => {
                // cast the dyn Trait back to component using ComponentToAny
                let it = boxed_component.as_any();
                match it.downcast_ref::<C>() {
                    Some(component) => Some(component),
                    // if we find none here, we couldn't cast the component :
                    // it was stored under a wrong id ! check unique ids !
                    None => {
                        println!("WARNING -> unable to cast component to desired type.
                            This may be due to components ids not being uniques !");
                        None
                    },
                }
            },
            // if we have none here, the desired component is not on the object
            None => None,
        }
    }
}