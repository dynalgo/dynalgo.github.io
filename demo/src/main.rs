mod algo;
mod fun;
mod utils;

use crate::algo::compare::Compare;
use crate::fun::maze::Maze;
use crate::utils::random::Random;
//use crate::utils::colors::Colors;
use dynalgo::graph::Graph;
use dynalgo::graph::GraphError;

fn main() {
    println!("Dynalgo demo");

    let mut results = Vec::new();

    println!("demo_maze");
    match demo_maze() {
        Ok((title, graphs)) => results.push((title, graphs)),
        Err(e) => {
            eprintln!("Demo maze: {}", e);
            return;
        }
    };

    println!("demo_isomorphic");
    match demo_isomorphic() {
        Ok((title, graphs)) => results.push((title, graphs)),
        Err(e) => {
            eprintln!("Demo isomorphic: {}", e);
            return;
        }
    };

    println!("demo_equal");
    match demo_equal() {
        Ok((title, graphs)) => results.push((title, graphs)),
        Err(e) => {
            eprintln!("Demo equal: {}", e);
            return;
        }
    };

    let mut pages = Vec::new();
    for (title, graphs) in &results {
        let mut graphs_refs = Vec::new();
        for graph in graphs {
            graphs_refs.push(graph);
        }
        pages.push((title.as_str(), graphs_refs));
    }

    Graph::render_to_html_files(pages).unwrap();
}

fn demo_maze() -> Result<(String, Vec<Graph>), GraphError> {
    let mut maze_freezed = Maze::generate(6);
    maze_freezed.anim_pause()?;
    maze_freezed.anim_step(4000)?;
    maze_freezed.anim_resume()?;

    let mut maze_unfreezed = Graph::new();
    maze_unfreezed.anim_pause()?;
    maze_unfreezed
        .param_radius_node(15)
        .param_color_tag_created(128, 139, 150);
    maze_unfreezed.append_from_graph(&maze_freezed)?;
    maze_unfreezed.anim_step(2000)?;
    maze_unfreezed.anim_nodes_freeze(maze_unfreezed.nodes_list(), false)?;
    maze_unfreezed.anim_step(2000)?;
    maze_unfreezed.anim_resume()?;

    Maze::solve(&mut maze_freezed);
    Maze::solve(&mut maze_unfreezed);
    Ok((
        String::from("Traverse a maze"),
        vec![maze_freezed, maze_unfreezed],
    ))
}

fn demo_isomorphic() -> Result<(String, Vec<Graph>), GraphError> {
    let dim = 3;
    let graph_freezed = Maze::generate(dim);

    let mut graph1 = Graph::new();
    graph1.param_radius_node(15);
    graph1.append_from_graph(&graph_freezed)?;

    let mut graph2 = Graph::new();
    graph2.param_radius_node(15);
    graph2.append_from_graph(&graph_freezed)?;
    let nodes = graph2.nodes_list();
    for _ in 0..(nodes.len() / 2) {
        let idx = Random::poor_random((nodes.len() - 1) as u32) + 1;
        graph2.nodes_exchange(nodes[0], nodes[idx as usize])?;
    }

    Compare::isomorphic(&mut graph1, &mut graph2);

    Ok((String::from("Check isomorphism"), vec![graph1, graph2]))
}

fn demo_equal() -> Result<(String, Vec<Graph>), GraphError> {
    let dim = 6;
    let mut graph1 = Maze::generate(dim);
    graph1.anim_pause()?;
    graph1.anim_step(5000)?;
    graph1.anim_resume()?;

    let mut graph2 = Graph::new();
    graph2.param_radius_node(15);
    graph2.append_from_graph(&graph1)?;
    let nodes = graph2.nodes_list();
    graph2.anim_pause()?;
    graph2.anim_step(1000)?;
    graph2.anim_resume()?;
    graph2.nodes_exchange(
        nodes[(dim * dim - 2) as usize],
        nodes[(dim * dim - 3) as usize],
    )?;
    graph2.anim_pause()?;
    graph2.anim_step(1000)?;
    graph2.anim_resume()?;

    if Compare::equal(&mut graph1, &mut graph2, true) {
        graph1.anim_nodes_color(graph1.nodes_list(), 0, 255, 0)?;
        graph2.anim_nodes_color(graph2.nodes_list(), 0, 255, 0)?;
    }

    Ok((String::from("Check equality"), vec![graph1, graph2]))
}
