use super::agent_manager::{AgentIdAccessor, AgentManager};
use super::component::Component;
use super::system::System;
use super::agent::RoleSetting;
use super::agent::Agent;
pub struct Environment {
    agent_manager: AgentManager,
    agent_id_accessor: AgentIdAccessor,
    systems: Vec<Box<dyn System>>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            agent_manager: AgentManager::new(),
            agent_id_accessor: AgentIdAccessor::new(),
            systems: vec![]
        }
    }

    pub fn add_agent(&mut self, agent: Agent) -> usize {
        self.agent_manager.add_agent(agent)
    }

    pub fn create_agent(&mut self) -> usize {
        self.agent_manager.create_agent()
    }

    pub fn remove_agent(&mut self, agent_id: usize) {
        self.agent_manager.remove_agent(agent_id);
    }

    pub fn register_component<T: Component>(&mut self) -> &mut Self {
        self.agent_manager.register::<T>();
        self
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn add_component_to_agent<T: Component>(&mut self, agent_id: usize, component: T) -> &mut Self {
        self.agent_manager.add_component_to_agent(agent_id, component);
        self
    }

    pub fn init_default_component(&mut self) -> &mut Self {
        // self.register_component::<RoleSetting>();
        self
    }

    pub async fn update(&mut self) {
        for system in self.systems.iter_mut() {
            system.update(&mut self.agent_manager, &mut self.agent_id_accessor).await;
            self.agent_manager.increment_frame();
        }
    }
    
}
