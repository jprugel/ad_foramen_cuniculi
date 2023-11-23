pub mod entity {
    use crate::component::Component;
    #[derive(Default)]
    pub struct Entity {
        pub id: usize,
        name: String,
        components: Vec<Box<dyn Component>>,
    }

    impl Entity {
        pub fn set_id(&mut self, id: usize) {
            self.id = id;
        }

        pub fn set_name(&mut self, name: &str) {
            self.name = name.to_string();
        }

        pub fn add_component<T: Default + 'static>(&mut self) -> &mut Self {
            self.components.push(Box::<T>::default());
            self
        }

        pub fn add_component_dec(&mut self, component: Box<dyn Component>) {
            self.components.push(component);
        }

        pub fn has_component<T: Component + 'static>(&self) -> bool {
            self.components.iter().any(|x| x.as_any().is::<T>())
        }

        pub fn get_component<T: 'static>(&self) -> Option<&T> {
            let pos = self
                .components
                .iter()
                .position(|x| (**x).as_any().is::<T>())?;
            let value = self.components.get(pos)?;
            let cast = (**value).as_any().downcast_ref::<T>()?;
            Some(cast)
        }

        pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
            let pos = self
                .components
                .iter()
                .position(|x| (**x).as_any().is::<T>())?;
            let value = self.components.get_mut(pos)?;
            let cast = (**value).as_any_mut().downcast_mut::<T>()?;
            Some(cast)
        }
    }

    #[derive(Default)]
    pub struct EntityBuilder {
        id: usize,
        name: String,
        components: Vec<Box<dyn Component>>,
    }

    impl EntityBuilder {
        pub fn set_id(mut self, id: usize) -> Self {
            self.id = id;
            self
        }

        pub fn set_name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }

        pub fn with(mut self, component: Box<dyn Component>) -> Self {
            self.components.push(component);
            self
        }

        pub fn build(self) -> Entity {
            Entity {
                id: self.id,
                name: self.name,
                components: self.components,
            }
        }
    }
}

pub mod component {
    use std::any::Any;

    pub trait Component {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
    }

    impl<T: 'static> Component for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
}

pub mod system {
    use crate::entity::{Entity, EntityBuilder};

    #[derive(Default)]
    pub struct System {
        pub entities: Vec<Entity>,
    }

    impl System {
        pub fn create_entity(&mut self) -> EntityBuilder {
            EntityBuilder::default()
        }

        pub fn get_entity(&self, id: usize) -> &Entity {
            let pos = self.entities.iter().position(|x| x.id == id).unwrap();
            self.entities.get(pos).unwrap()
        }

        pub fn get_entity_mut(&mut self, id: usize) -> &mut Entity {
            let pos = self.entities.iter().position(|x| x.id == id).unwrap();
            self.entities.get_mut(pos).unwrap()
        }
    }
}
