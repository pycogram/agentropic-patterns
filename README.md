# agentropic-patterns

[![Crates.io](https://img.shields.io/crates/v/agentropic-patterns.svg)](https://crates.io/crates/agentropic-patterns)
[![Documentation](https://docs.rs/agentropic-patterns/badge.svg)](https://docs.rs/agentropic-patterns)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

**Multi-agent system coordination patterns and organizational structures.**

`agentropic-patterns` provides high-level organizational patterns and coordination mechanisms for building sophisticated multi-agent systems. It implements proven architectural patterns for agent collaboration, from hierarchical command structures to emergent swarm behaviors.

---

## 🎯 Purpose

This crate provides:

- **Organizational Patterns**: Hierarchy, holarchy, teams, coalitions
- **Coordination Mechanisms**: Swarm intelligence, market-based, voting
- **Interaction Protocols**: Negotiation, auction, contract net
- **Emergent Behaviors**: Flocking, consensus, collective decision-making

---

## 🧩 Core Patterns

### 1. Hierarchical Organization

Command and control structure with clear authority lines:
```rust
use agentropic_patterns::{Hierarchy, HierarchyBuilder, Role};

// Build a hierarchy
let hierarchy = HierarchyBuilder::new()
    .add_level("executive", vec![ceo_agent])
    .add_level("management", vec![cto_agent, cfo_agent])
    .add_level("operational", vec![dev1, dev2, dev3])
    .build();

// Delegate task down the hierarchy
hierarchy.delegate_task(task, "executive").await?;

// Report results up the hierarchy
hierarchy.report_result(result, "operational").await?;
```

### 2. Team Organization

Peer-based collaboration with shared goals:
```rust
use agentropic_patterns::{Team, TeamRole};

// Create a team
let mut team = Team::new("development-team");

// Add members with roles
team.add_member(agent1, TeamRole::Leader);
team.add_member(agent2, TeamRole::Member);
team.add_member(agent3, TeamRole::Member);

// Set shared goal
team.set_goal(Goal::new("build_feature"));

// Coordinate work
team.coordinate().await?;
```

### 3. Swarm Intelligence

Decentralized, emergent behavior through local interactions:
```rust
use agentropic_patterns::{Swarm, SwarmBehavior};

// Create a swarm
let mut swarm = Swarm::new()
    .with_behavior(SwarmBehavior::Flocking)
    .with_agents(drone_agents);

// Define simple local rules
swarm.add_rule("separation", |agent, neighbors| {
    agent.avoid(neighbors.close_by())
});

swarm.add_rule("alignment", |agent, neighbors| {
    agent.align_with(neighbors.nearby())
});

swarm.add_rule("cohesion", |agent, neighbors| {
    agent.move_toward(neighbors.center())
});

// Execute swarm
swarm.execute().await?;
```

### 4. Market-Based Coordination

Resource allocation through bidding and pricing:
```rust
use agentropic_patterns::{Market, Auction, AuctionType};

// Create a market
let market = Market::new("task-market");

// Announce task (seller)
let auction = Auction::new(task, AuctionType::English)
    .starting_price(100.0)
    .reserve_price(150.0);

market.announce(auction).await?;

// Submit bids (buyers)
for agent in agents {
    let bid = agent.evaluate_task(&task).await?;
    market.submit_bid(agent.id(), bid).await?;
}

// Award to winner
let winner = market.close_auction(auction.id()).await?;
```

### 5. Coalition Formation

Dynamic grouping based on shared interests:
```rust
use agentropic_patterns::{Coalition, CoalitionFormation};

// Find agents with compatible goals
let formation = CoalitionFormation::new(agents);

// Form coalitions
let coalitions = formation
    .with_strategy(FormationStrategy::Greedy)
    .form().await?;

// Each coalition works together
for coalition in coalitions {
    coalition.execute_joint_plan().await?;
}
```

### 6. Blackboard System

Shared knowledge space for problem-solving:
```rust
use agentropic_patterns::{Blackboard, KnowledgeSource};

// Create blackboard
let blackboard = Blackboard::new();

// Register knowledge sources (specialized agents)
blackboard.register(
    KnowledgeSource::new("parser", parser_agent)
        .monitors("raw_data")
        .produces("parsed_data")
);

blackboard.register(
    KnowledgeSource::new("analyzer", analyzer_agent)
        .monitors("parsed_data")
        .produces("analysis_result")
);

// Solve problem collaboratively
blackboard.solve(problem).await?;
```

### 7. Holarchy

Nested hierarchy where each unit is both whole and part:
```rust
use agentropic_patterns::{Holarchy, Holon};

// Each holon can contain sub-holons
let company = Holon::new("company")
    .add_sub_holon(
        Holon::new("engineering")
            .add_sub_holon(Holon::new("frontend-team"))
            .add_sub_holon(Holon::new("backend-team"))
    )
    .add_sub_holon(
        Holon::new("sales")
            .add_sub_holon(Holon::new("enterprise"))
            .add_sub_holon(Holon::new("smb"))
    );

let holarchy = Holarchy::new(company);
```

### 8. Federation

Autonomous units with coordinated policies:
```rust
use agentropic_patterns::{Federation, FederationPolicy};

// Create federation
let federation = Federation::new()
    .add_member(org1, FederationPolicy::Autonomous)
    .add_member(org2, FederationPolicy::Autonomous)
    .set_coordination_protocol(Protocol::Consensus);

// Members remain autonomous but coordinate on shared concerns
federation.coordinate_policy("data-sharing").await?;
```

---

## 📦 What's Included

### Organizational Patterns

- `Hierarchy` - Top-down command and control
- `Team` - Peer-based collaboration
- `Coalition` - Dynamic temporary groups
- `Holarchy` - Nested autonomous units
- `Federation` - Coordinated autonomous organizations
- `Matrix` - Dual reporting structures

### Coordination Mechanisms

- `Swarm` - Emergent collective behavior
- `Market` - Economic-based allocation
- `Voting` - Democratic decision-making
- `Consensus` - Agreement protocols
- `Blackboard` - Shared knowledge space
- `Stigmergy` - Environment-mediated coordination

### Interaction Protocols

- `ContractNet` - Task allocation through bidding
- `Negotiation` - Bilateral/multilateral bargaining
- `Auction` - English, Dutch, sealed-bid auctions
- `Matchmaking` - Capability-based pairing
- `Broker` - Intermediary-based coordination

### Behavioral Patterns

- `Flocking` - Cohesive group movement
- `Foraging` - Resource discovery and collection
- `Formation` - Spatial pattern maintenance
- `Queuing` - Fair resource access
- `LoadBalancing` - Work distribution

---

## 🚀 Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
agentropic-patterns = "0.1.0"
agentropic-core = "0.1.0"
agentropic-messaging = "0.1.0"
```

### Complete Swarm Example
```rust
use agentropic_patterns::{Swarm, SwarmAgent, SwarmBehavior};
use agentropic_core::{Agent, AgentId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create swarm agents
    let agents: Vec<Box<dyn SwarmAgent>> = (0..50)
        .map(|_| Box::new(DroneAgent::new()) as Box<dyn SwarmAgent>)
        .collect();

    // Configure swarm behavior
    let mut swarm = Swarm::builder()
        .agents(agents)
        .behavior(SwarmBehavior::Flocking)
        .parameter("separation_distance", 2.0)
        .parameter("alignment_weight", 1.0)
        .parameter("cohesion_weight", 1.0)
        .build();

    // Add obstacles
    swarm.add_obstacle(Obstacle::sphere(Point3::new(50, 50, 0), 10.0));

    // Set collective goal
    swarm.set_goal(Point3::new(100, 100, 0));

    // Execute swarm
    loop {
        swarm.step().await?;
        
        if swarm.goal_reached() {
            break;
        }
        
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
```

### Market-Based Task Allocation
```rust
use agentropic_patterns::{Market, Task, Bid, AuctionType};

async fn allocate_tasks(
    tasks: Vec<Task>,
    agents: Vec<AgentId>
) -> Result<(), Box<dyn std::error::Error>> {
    let market = Market::new("task-allocation");

    for task in tasks {
        // Create auction
        let auction = market.create_auction(
            task.clone(),
            AuctionType::FirstPrice,
            Duration::from_secs(30)
        ).await?;

        // Agents submit bids
        for agent_id in &agents {
            // Agent evaluates task
            let valuation = evaluate_task(agent_id, &task).await?;
            
            // Submit bid
            let bid = Bid::new(*agent_id, valuation);
            market.submit_bid(auction.id(), bid).await?;
        }

        // Close auction and allocate
        let winner = market.close_auction(auction.id()).await?;
        println!("Task {} allocated to agent {}", task.id(), winner);
    }

    Ok(())
}
```

### Hierarchical Delegation
```rust
use agentropic_patterns::{Hierarchy, HierarchyLevel, DelegationStrategy};

async fn hierarchical_task_management() -> Result<(), Box<dyn std::error::Error>> {
    // Build organizational hierarchy
    let hierarchy = Hierarchy::builder()
        .add_level(
            HierarchyLevel::new("executive")
                .add_agent(ceo_agent)
        )
        .add_level(
            HierarchyLevel::new("management")
                .add_agent(vp_engineering)
                .add_agent(vp_sales)
                .add_agent(vp_operations)
        )
        .add_level(
            HierarchyLevel::new("operational")
                .add_agents(engineers)
                .add_agents(sales_reps)
                .add_agents(operators)
        )
        .delegation_strategy(DelegationStrategy::Capability)
        .build();

    // Task arrives at top
    let task = Task::new("increase_revenue");

    // Delegate down hierarchy
    hierarchy.delegate(task).await?;

    // Each level breaks down and delegates
    // Results flow back up

    Ok(())
}
```

### Coalition Formation Example
```rust
use agentropic_patterns::{Coalition, CoalitionFormation, FormationStrategy};

async fn form_coalitions(
    agents: Vec<AgentId>,
    tasks: Vec<Task>
) -> Result<Vec<Coalition>, Box<dyn std::error::Error>> {
    let formation = CoalitionFormation::new(agents)
        .with_strategy(FormationStrategy::OptimalGreedy)
        .with_tasks(tasks);

    // Form coalitions based on:
    // - Complementary capabilities
    // - Shared goals
    // - Resource constraints
    let coalitions = formation.form().await?;

    // Each coalition has:
    // - Member agents
    // - Assigned tasks
    // - Resource pool
    // - Coordination protocol

    for coalition in &coalitions {
        println!("Coalition {}: {} members, {} tasks",
            coalition.id(),
            coalition.members().len(),
            coalition.tasks().len()
        );

        // Execute coalition plan
        coalition.execute().await?;
    }

    Ok(coalitions)
}
```

### Consensus Protocol
```rust
use agentropic_patterns::{Consensus, ConsensusProtocol, Vote};

async fn reach_consensus(
    agents: Vec<AgentId>,
    proposal: Proposal
) -> Result<bool, Box<dyn std::error::Error>> {
    let consensus = Consensus::new(ConsensusProtocol::ByzantineFaultTolerant)
        .with_agents(agents)
        .threshold(0.67); // 2/3 majority

    // Each agent votes
    for agent in agents {
        let vote = agent.evaluate_proposal(&proposal).await?;
        consensus.cast_vote(agent, vote).await?;
    }

    // Check if consensus reached
    let result = consensus.tally().await?;

    Ok(result.accepted)
}
```

---

## 🏗️ Architecture

### Pattern Categories

**Structural Patterns** - How agents are organized:
- Hierarchy, holarchy, team, federation

**Behavioral Patterns** - How agents interact:
- Swarm, market, blackboard, stigmergy

**Coordination Patterns** - How agents synchronize:
- Consensus, voting, negotiation, auction

### Pattern Selection Guide

| Use Case | Pattern | Characteristics |
|----------|---------|----------------|
| Clear command structure | Hierarchy | Centralized, efficient |
| Peer collaboration | Team | Distributed, flexible |
| Large-scale coordination | Swarm | Emergent, scalable |
| Resource allocation | Market | Economic, competitive |
| Shared problem-solving | Blackboard | Collaborative, opportunistic |
| Temporary collaboration | Coalition | Dynamic, goal-oriented |
| Autonomous units | Federation | Independent, coordinated |

---

## 🔗 Related Crates

- **[agentropic-core](../agentropic-core)** - Agent primitives and traits
- **[agentropic-messaging](../agentropic-messaging)** - Communication protocols
- **[agentropic-cognition](../agentropic-cognition)** - Reasoning and planning
- **[agentropic-runtime](../agentropic-runtime)** - Agent execution engine

---

## 📚 Documentation

Full API documentation is available on [docs.rs](https://docs.rs/agentropic-patterns).

For guides and tutorials, see [agentropic-docs](https://github.com/agentropic/agentropic-docs).

---

## 🎓 References

This crate is inspired by:

- **Swarm Intelligence** - Kennedy & Eberhart (1995)
- **Market-Based Control** - Clearwater (1996)
- **Contract Net Protocol** - Smith (1980)
- **Blackboard Systems** - Erman et al. (1980)
- **Holonic Systems** - Koestler (1967)
- **Coalition Formation** - Shehory & Kraus (1998)

---

## 🤝 Contributing

Contributions are welcome! Please see the [contributing guidelines](../../CONTRIBUTING.md).

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## 🌟 Status

**Active Development** - This crate is under active development. APIs may change before 1.0 release.

---

*Part of the [Agentropic](https://github.com/agentropic) ecosystem for agent-oriented programming in Rust.*