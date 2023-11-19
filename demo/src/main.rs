mod algo;
mod fun;
mod utils;

use crate::algo::compare::Compare;
use crate::fun::maze::Maze;
use crate::utils::colors::Colors;
use dynalgo::graph::Graph;
use dynalgo::graph::GraphError;

fn main() {
    println!("Dynalgo demo");

    let mut results = Vec::new();

    match demo_maze() {
        Ok((title, graphs)) => results.push((title, graphs)),
        Err(e) => {
            eprintln!("Demo maze: {}", e);
            return;
        }
    };

    match demo_isomorphic() {
        Ok((title, graphs)) => results.push((title, graphs)),
        Err(e) => {
            eprintln!("Demo isomorphic: {}", e);
            return;
        }
    };

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
    let (graph_freezed, graph_unfreezed) = Maze::generate_and_solve(8);
    Ok((String::from("maze"), vec![graph_freezed, graph_unfreezed]))
}

fn demo_isomorphic() -> Result<(String, Vec<Graph>), GraphError> {
    let mut graph1 = Graph::new();
    graph1.anim_pause()?;
    graph1.node_add('A')?;
    graph1.node_add('B')?;
    graph1.node_add('C')?;
    graph1.node_add('D')?;
    graph1.link_add('A', 'B', false, 0)?;
    graph1.link_add('C', 'B', true, 0)?;
    graph1.link_add('D', 'C', true, 0)?;
    graph1.link_add('D', 'A', false, 0)?;
    graph1.anim_resume()?;
    let mut graph2 = Graph::new();
    graph2.anim_pause()?;
    graph2.node_add('G')?;
    graph2.node_add('H')?;
    graph2.link_add('H', 'G', true, 0)?;
    graph2.node_add('E')?;
    graph2.link_add('H', 'E', false, 0)?;
    graph2.node_add('F')?;
    graph2.link_add('E', 'F', false, 0)?;
    graph2.link_add('G', 'F', true, 0)?;
    graph2.anim_resume()?;
    let bijection = Compare::isomorphic(&graph1, &graph2);
    match bijection {
        Some(bij) => {
            graph1.param_duration_color(300);
            graph2.param_duration_color(300);
            let mut colors = Colors::colors(graph1.nodes_list().len()).into_iter();
            for (node, node_bij) in bij.iter() {
                let (r, g, b) = colors.next().unwrap();
                graph1.anim_node_color(*node_bij, r, g, b)?;
                graph2.anim_node_color(*node, r, g, b)?;
            }
        }
        None => {}
    }

    Ok((String::from("isomorphic"), vec![graph1, graph2]))
}

fn demo_equal() -> Result<(String, Vec<Graph>), GraphError> {
    let mut graph1 = Graph::new();
    graph1.anim_pause()?;
    graph1.node_add('A')?;
    graph1.node_add('B')?;
    graph1.node_add('C')?;
    graph1.node_add('D')?;
    graph1.link_add('A', 'B', false, 0)?;
    graph1.link_add('C', 'B', true, 0)?;
    graph1.link_add('D', 'C', true, 0)?;
    graph1.link_add('D', 'A', false, 0)?;
    graph1.anim_resume()?;

    let mut graph2 = Graph::new();
    graph2.anim_pause()?;
    graph2.node_add('G')?;
    graph2.node_add('H')?;
    graph2.link_add('H', 'G', true, 0)?;
    graph2.node_add('E')?;
    graph2.link_add('H', 'E', false, 0)?;
    graph2.node_add('F')?;
    graph2.link_add('E', 'F', false, 0)?;
    graph2.link_add('G', 'F', true, 0)?;
    graph2.anim_resume()?;

    let equal2 = Compare::equal(&graph1, &graph2, false);
    let (r, g, b) = match equal2 {
        true => (0, 196, 0),
        false => (196, 0, 0),
    };
    graph2.param_duration_color(100);
    for node in graph2.nodes_list() {
        graph2.anim_node_color(node, r, g, b)?;
    }

    let mut graph3 = Graph::new();
    graph3.anim_pause()?;
    graph3.node_add('A')?;
    graph3.node_add('D')?;
    graph3.link_add('D', 'A', false, 0)?;
    graph3.node_add('B')?;
    graph3.link_add('A', 'B', false, 0)?;
    graph3.node_add('C')?;
    graph3.link_add('C', 'B', true, 0)?;
    graph3.link_add('D', 'C', true, 0)?;
    graph3.anim_resume()?;

    let equal3 = Compare::equal(&graph1, &graph3, false);
    let (r, g, b) = match equal3 {
        true => (0, 196, 0),
        false => (196, 0, 0),
    };
    graph3.param_duration_color(100);
    for node in graph3.nodes_list() {
        graph3.anim_node_color(node, r, g, b)?;
    }

    Ok((String::from("equal"), vec![graph1, graph2, graph3]))
}
