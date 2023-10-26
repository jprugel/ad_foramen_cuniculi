use std::any::{Any, TypeId};
use std::collections::HashMap;
// Entity ------------------------------------------------------------------------------------------
pub struct Entity {
    id: EntityId,
    name: String,
    components: HashMap<TypeId, Box<dyn Component>>,
}
// Entity Builder ----------------------------------------------------------------------------------
#[derive(Default)]
pub struct EntityBuilder {
    id: Option<EntityId>,
    name: Option<String>,
    components: Option<HashMap<TypeId, Box<dyn Component>>>,
}
impl EntityBuilder {
    pub fn set_id(mut self, id: EntityId) -> Self {
        self.id =  Some(id);
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn add_component<T: Component + 'static>(mut self, component: T) -> Self {
        match self.components.as_mut() {
            Some(thing) => thing.insert(component.type_id(), Box::new(component)),
            None => {
                self.components = Some(HashMap::new());
                self.components
                    .as_mut()
                    .unwrap()
                    .insert(component.type_id(), Box::new(component))
            },
        };
        self
    }

    pub fn build(self) -> Entity {
        let id = match self.id {
            Some(id) => id,
            None => {
                println!("Please give an id to the entity!");
                0
            },
        };

        let name = match self.name {
            Some(a) => a,
            None => String::new(),
        };

        let components = match self.components {
            Some(a) => a,
            None => HashMap::new(),
        };

        Entity {
            id,
            name,
            components,
        }
    }

}
// Component ---------------------------------------------------------------------------------------
pub trait Component {
    fn as_any(&self) -> &dyn Any where Self: Sized + 'static {
        self as &dyn Any
    }
}
// System ------------------------------------------------------------------------------------------
#[derive(Default)]
pub struct System {
    entities: HashMap<EntityId, Entity>,
}
impl System {
    pub fn create_entity(&mut self) -> EntityBuilder {
        EntityBuilder::default()
    }
}
// Utility -----------------------------------------------------------------------------------------
type EntityId = usize;
