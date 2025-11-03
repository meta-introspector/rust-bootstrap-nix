use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::toposort;

#[derive(Debug, Deserialize, Clone)]
struct Task {
    title: String,
    description: String,
    status: String,
    created_at: String,
    parent_task: Option<String>,
    stage_id: Option<u32>,
    step_id: Option<String>,
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tasks_dir = Path::new("/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/platform-tools-agave-rust-solana/vendor/rust-src/vendor/rust/rust-bootstrap-nix/tasks");
    let mut tasks: HashMap<String, Task> = HashMap::new();
    let mut all_task_ids: HashSet<String> = HashSet::new();
    let mut all_parent_task_names: HashSet<String> = HashSet::new();

    let mut task_file_paths: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(tasks_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "toml") {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();

            // Skip the original plan files (which are not individual tasks)
            if file_name == "01_flake_lattice_plan.toml" || 
               file_name == "03_project_analysis_plan.toml" ||
               file_name == "02_current_development_plan.toml" ||
               file_name == "04_qa_plan.toml" {
                continue;
            }
            task_file_paths.push(path);
        }
    }

    for path in task_file_paths {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let content = fs::read_to_string(&path)?;
        let task: Task = toml::from_str(&content)?;

        all_task_ids.insert(file_name.clone());
        if let Some(parent) = &task.parent_task {
            all_parent_task_names.insert(parent.clone());
        }
        tasks.insert(file_name.clone(), task.clone());
    }

    let recognized_virtual_nodes: HashSet<String> = [
        "bootstrap.md".to_string(),
        "lattice.md".to_string(),
        "plan.md".to_string(),
        "01_flake_lattice_plan.toml".to_string(),
        "03_project_analysis_plan.toml".to_string(),
        "02_current_development_plan.toml".to_string(),
        "04_qa_plan.toml".to_string(),
        "docs/QA_Plan.md".to_string(), // Added this line
    ].iter().cloned().collect();

    // Validate parent_task references
    let mut unresolved_parents = Vec::new();
    for (task_id, task) in &tasks {
        if let Some(parent) = &task.parent_task {
            if !all_task_ids.contains(parent) && !recognized_virtual_nodes.contains(parent) {
                unresolved_parents.push(format!("Task '{}' references unresolved parent: '{}'", task_id, parent));
            }
        }
    }

    if !unresolved_parents.is_empty() {
        eprintln!("Error: Unresolved parent task references found:");
        for error_msg in unresolved_parents {
            eprintln!("  {}", error_msg);
        }
        return Err("Unresolved parent task references".into());
    }

    // Build the graph
    let mut graph = DiGraph::<String, String>::new();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

    // Add nodes for all individual tasks
    for task_id in &all_task_ids {
        let node_idx = graph.add_node(task_id.clone());
        node_indices.insert(task_id.clone(), node_idx);
    }

    // Add nodes for all unique parent_task names (virtual nodes for plan files)
    for parent_name in &all_parent_task_names {
        if !node_indices.contains_key(parent_name) {
            let node_idx = graph.add_node(parent_name.clone());
            node_indices.insert(parent_name.clone(), node_idx);
        }
    }

    // Add edges based on parent_task relationships
    for task_id in &all_task_ids {
        let task = tasks.get(task_id).unwrap();
        if let Some(parent_task_name) = &task.parent_task {
            let parent_node_idx = *node_indices.get(parent_task_name).unwrap();
            let child_node_idx = *node_indices.get(task_id).unwrap();
            graph.add_edge(parent_node_idx, child_node_idx, "parent_of".to_string());
        }
    }

    // Perform topological sort
    match toposort(&graph, None) {
        Ok(order) => {
            let mut topological_order_content = String::new();
            topological_order_content.push_str("# Topological Order of Tasks\n");
            topological_order_content.push_str("tasks = [\n");
            for node_idx in order {
                let task_name = graph.node_weight(node_idx).unwrap();
                // Only include actual task files in the output
                if all_task_ids.contains(task_name) {
                    topological_order_content.push_str(&format!("    \"{}\"", task_name));
                    topological_order_content.push_str(",\n");
                }
            }
            topological_order_content.push_str("]\n");

            let output_path = Path::new("taskorder.toml");
            fs::write(output_path, topological_order_content)?;
            println!("Topological order written to taskorder.toml");
        },
        Err(cycle) => {
            eprintln!("Error: Cycle detected in task dependencies. Topological sort not possible.");
            eprintln!("Node in cycle: {}", graph.node_weight(cycle.node_id()).unwrap());
        }
    }

    Ok(())
}