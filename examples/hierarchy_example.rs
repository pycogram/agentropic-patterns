use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Hierarchy Example ===\n");

    // Create hierarchy
    let mut hierarchy = Hierarchy::new("corporate");

    // Define levels
    let ceo_level = Level::new("CEO", LevelType::Strategic, 3);
    let manager_level = Level::new("Manager", LevelType::Tactical, 2);
    let worker_level = Level::new("Worker", LevelType::Operational, 1);

    hierarchy.add_level(ceo_level.clone());
    hierarchy.add_level(manager_level.clone());
    hierarchy.add_level(worker_level.clone());

    // Assign agents
    let ceo = AgentId::new();
    let manager = AgentId::new();
    let worker = AgentId::new();

    hierarchy.assign_agent(ceo.clone(), ceo_level);
    hierarchy.assign_agent(manager.clone(), manager_level);
    hierarchy.assign_agent(worker.clone(), worker_level);

    // Delegate task
    let delegation = Delegation::new(ceo.clone(), manager.clone(), "implement_strategy", 2);
    hierarchy.delegate(delegation);

    println!("Hierarchy: {}", hierarchy.name());
    println!("Levels: {}", hierarchy.levels().len());
    println!("Delegations: {}", hierarchy.delegations().len());
}
