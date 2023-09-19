use std::collections::HashMap;
use std::any::{Any, TypeId};

use super::component::Component;

// @TODO: Write comment
pub trait ComponentManagerTrait: Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // @TODO: Write comment

    fn has(&self, agent_id: usize) -> bool;
    fn remove(&mut self, agent_id: usize);
    fn get_type_id(&self) -> TypeId;
}

impl<T: Component> ComponentManagerTrait for ComponentManager<T> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn has(&self, agent_id: usize) -> bool {
        let manager = cast_manager::<T>(self);
        manager.has(agent_id)
    }

    fn remove(&mut self, agent_id: usize) {
        let manager = cast_manager_mut::<T>(self);
        manager.remove(agent_id);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

// @TODO: Write comment
pub fn cast_manager<T: Component>
    (manager: &dyn ComponentManagerTrait) -> &ComponentManager<T> {
    manager
        .as_any()
        .downcast_ref::<ComponentManager<T>>()
        .unwrap()
}

// @TODO: Write comment
pub fn cast_manager_mut<T: Component>
    (manager: &mut dyn ComponentManagerTrait) -> &mut ComponentManager<T> {
    manager
        .as_any_mut()
        .downcast_mut::<ComponentManager<T>>()
        .unwrap()
}

pub struct ComponentManager<T: Component> {
    components: Vec<T>, // Component contents
    agent_ids: Vec<usize>, // Same order with components
    agent_id_map: HashMap<usize, usize>  // agent_id -> index in components
}

impl<T: Component> ComponentManager<T> {
    pub fn new() -> Self {
        ComponentManager {
            components: Vec::new(),
            agent_ids: Vec::new(),
            agent_id_map: HashMap::new(),
        }
    }

    pub fn has(&self, agent_id: usize) -> bool {
        self.agent_id_map.contains_key(&agent_id)
    }

    pub fn add(&mut self, agent_id: usize, component: T) {
        if self.has(agent_id) {
            // Nothing to do? Throw error? Update component?
            return;
        }
        self.components.push(component);
        self.agent_ids.push(agent_id);
        let component_index = self.components.len() - 1;
        self.agent_id_map.insert(agent_id, component_index);
    }

    pub fn remove(&mut self, agent_id: usize) {
        if !self.has(agent_id) {
            // Nothing to do? Throw error? Update component?
            return;
        }
        let index = *self.agent_id_map.get(&agent_id).unwrap();
        self.agent_id_map.insert(*self.agent_ids.last().unwrap(), index);
        self.components.swap_remove(index);
        self.agent_ids.swap_remove(index);
        self.agent_id_map.remove(&agent_id);
    }

    pub fn borrow_component(&self, agent_id: usize) -> Option<&T> {
        if !self.has(agent_id) {
            return None;
        }
        let index = self.agent_id_map.get(&agent_id).unwrap();
        Some(&self.components[*index])
    }

    pub fn borrow_component_mut(&mut self, agent_id: usize) -> Option<&mut T> {
        if !self.has(agent_id) {
            return None;
        }
        let index = self.agent_id_map.get(&agent_id).unwrap();
        Some(&mut self.components[*index])
    }

    pub fn borrow_agent_ids(&self) -> &Vec<usize> {
        &self.agent_ids
    }

    pub fn borrow_components(&self) -> &Vec<T> {
        &self.components
    }

    pub fn borrow_components_mut(&mut self) -> &mut Vec<T> {
        &mut self.components
    }
}
