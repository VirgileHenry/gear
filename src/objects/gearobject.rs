use crate::objects::components::component::*;
use std::collections::HashMap;

pub struct GearObject {
    // transform : not here 
    // TODO : childrens ?
    // TODO : components !
    components: HashMap<i32, Box<dyn Component>>
}

impl GearObject {
    pub fn empty() -> GearObject {
        // creates an empty gearObject
        return GearObject {
            components: HashMap::new()
        };
    }

    pub fn add_component<C: 'static + Component>(&mut self) {
        self.components.insert(C::id(), Box::new(C::new()));
    }

    pub fn get_component<C: Component>(&self) -> Option<&C> {
        // get the component from the array under the form of Box<dyn Trait>
        let result = self.components.get(&C::id());
        return match result {
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