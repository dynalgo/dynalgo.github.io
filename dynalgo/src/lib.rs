//! # dynalgo
//!
//! `dynalgo` is a tiny RUST library designed to produce animated SVG images that can illustrate graph algorithms in action.
//!
//! The library focuces on providing a convenient tiny API for making animations in SVG SMIL format when developping algorithms working with graph structures.
//!
//! The crate offers a basic `graph` structure representation. Interesting point is that each graph structure modification results in an animation rendered in SVG SMIL format into a HTML page. Several graphs animations can be rendered together in the same HTML page (side to side).
//!
//! Dynalgo automatically layout nodes according to imaginary springs forces applying to them.
//! Custom animations can be made by playing with the nodes and links  graphical representations.
//!
//! The `Algo` module provides animated algorithms applying to graph.
//!
//! ### Example: traversing a maze
//! ```
//! use dynalgo::graph::Graph;
//!
//! let config = "😀 0 0
//!		😁 45 0
//!		😂 90 0
//!		😃 135 0
//!		😄 180 0
//!		😅 225 0
//!		😆 270 0
//!		😇 315 0
//!		😈 0 45
//!		😉 45 45
//!		😊 90 45
//!		😋 135 45
//!		😌 180 45
//!		😍 225 45
//!		😎 270 45
//!		😏 315 45
//!		😐 0 90
//!		😑 45 90
//!		😒 90 90
//!		😓 135 90
//!		😔 180 90
//!		😕 225 90
//!		😖 270 90
//!		😗 315 90
//!		😘 0 135
//!		😙 45 135
//!		😚 90 135
//!		😛 135 135
//!		😜 180 135
//!		😝 225 135
//!		😞 270 135
//!		😟 315 135
//!		😠 0 180
//!		😡 45 180
//!		😢 90 180
//!		😣 135 180
//!		😤 180 180
//!		😥 225 180
//!		😦 270 180
//!		😧 315 180
//!		😨 0 225
//!		😩 45 225
//!		😪 90 225
//!		😫 135 225
//!		😬 180 225
//!		😭 225 225
//!		😮 270 225
//!		😯 315 225
//!		😰 0 270
//!		😱 45 270
//!		😲 90 270
//!		😳 135 270
//!		😴 180 270
//!		😵 225 270
//!		😶 270 270
//!		😷 315 270
//!		😸 0 315
//!		😹 45 315
//!		😺 90 315
//!		😻 135 315
//!		😼 180 315
//!		😽 225 315
//!		😾 270 315
//!		😿 315 315
//!		😀 - 😁 0
//!		😁 - 😉 0
//!		😂 - 😃 0
//!		😂 - 😊 0
//!		😃 - 😄 0
//!		😄 - 😅 0
//!		😅 - 😍 0
//!		😆 - 😎 0
//!		😇 - 😏 0
//!		😈 - 😉 0
//!		😈 - 😐 0
//!		😊 - 😒 0
//!		😋 - 😓 0
//!		😌 - 😔 0
//!		😎 - 😏 0
//!		😎 - 😖 0
//!		😐 - 😑 0
//!		😐 - 😘 0
//!		😑 - 😒 0
//!		😒 - 😓 0
//!		😓 - 😛 0
//!		😔 - 😕 0
//!		😕 - 😖 0
//!		😕 - 😝 0
//!		😗 - 😟 0
//!		😘 - 😠 0
//!		😙 - 😚 0
//!		😚 - 😢 0
//!		😜 - 😝 0
//!		😝 - 😥 0
//!		😞 - 😟 0
//!		😞 - 😦 0
//!		😠 - 😡 0
//!		😡 - 😢 0
//!		😡 - 😩 0
//!		😢 - 😪 0
//!		😣 - 😤 0
//!		😤 - 😬 0
//!		😥 - 😦 0
//!		😥 - 😭 0
//!		😦 - 😧 0
//!		😦 - 😮 0
//!		😧 - 😯 0
//!		😨 - 😩 0
//!		😩 - 😱 0
//!		😪 - 😫 0
//!		😪 - 😲 0
//!		😫 - 😬 0
//!		😬 - 😴 0
//!		😮 - 😶 0
//!		😯 - 😷 0
//!		😰 - 😱 0
//!		😱 - 😹 0
//!		😳 - 😻 0
//!		😴 - 😼 0
//!		😵 - 😶 0
//!		😶 - 😾 0
//!		😷 - 😿 0
//!		😸 - 😹 0
//!		😹 - 😺 0
//!		😻 - 😼 0
//!		😼 - 😽 0
//!		😽 - 😾 0";
//!
//! let node_start = '😀';
//! let node_searched = '😿';
//!
//! let mut freezed_maze = Graph::new();
//! let mut unfreezed_maze = Graph::new();
//!
//! for graph in [&mut freezed_maze, &mut unfreezed_maze].iter_mut() {
//!     graph.from_str(config);
//!     graph.fill_node(node_start, (0, 0, 196));
//!     graph.fill_node(node_searched, (0, 0, 196));
//! }
//!
//! deep_first_search(
//!     &mut freezed_maze,
//!     node_start,
//!     node_searched,
//!     &mut Vec::new(),
//! );
//!
//! unfreezed_maze.pause();
//! for node in unfreezed_maze.nodes() {
//!     unfreezed_maze.unfreeze_node(node);
//! }
//! unfreezed_maze.resume();
//!
//! deep_first_search(
//!     &mut unfreezed_maze,
//!     node_start,
//!     node_searched,
//!     &mut Vec::new(),
//! );
//!
//! Graph::to_html( vec![("Dynalgo maze example", vec![&freezed_maze, &unfreezed_maze])] ).unwrap();
//!
//!
//! fn deep_first_search(
//!     graph: &mut Graph,
//!     node_from: char,
//!     node_searched: char,
//!     visited: &mut Vec<char>,
//! ) -> bool {
//!     visited.push(node_from);
//!     graph.color_node(node_from, (0, 255, 0));
//!
//!    if node_from == node_searched {
//!         return true;
//!     }
//!
//!     let adja = &graph.adjacency_list();
//!     let mut found = false;
//!     for (node_to, _link) in adja.get(&node_from).unwrap() {
//!         if visited.contains(node_to) {
//!             continue;
//!         }
//!         graph.color_link(node_from, *node_to, (0, 255, 0));
//!
//!         found = deep_first_search(graph, *node_to, node_searched, visited);
//!         if found {
//!             break;
//!         }
//!     }
//!
//!     if !found {
//!         graph.color_node(node_from, (255, 0, 0));
//!     }
//!     found
//! }
//! ```
//!
//! ### Example: viewing algorithms in action
//! ```
//! use dynalgo::graph::Graph;
//! use dynalgo::algo::coloration::Coloration;
//! use dynalgo::algo::connectivity::Connectivity;
//! use dynalgo::algo::eulerian::Eulerian;
//! use dynalgo::algo::tree::Tree;
//!
//! let mut g = Graph::new();
//! g.from_str(
//!     "A 0 0, B 100 -100, C 200 -100, D 300 -100, E 300 100, F 200 150,
//!     G 200 50, H 100 100, I 100 0, A > I, I > B, B > C, C > D, D > E, E < F, E > G, F > G, F > H,
//!     G < H, H < I",
//! );
//! let (_c_g, _components) = Connectivity::components(&g);
//! assert!(_components.len() == 1);
//!
//! let mut g = Graph::new();
//! g.from_str(
//!     "F 0 0, E 100 0, A 250 50, H 300 80, C 0 100, J 100 100, K 0 150, B 200 150,
//!     D 100 250, I 200 250, G 300 250, F > J, C > F, J > C, C > E, E > J, C - K, K > D, J > B,
//!     E > A, A - H,
//!     H > G, G > I, I > D, I > B, B > D, B > G",
//! );
//! let (scc_g, sc_components) = Connectivity::strongly_connected_components(&g);
//! assert!(sc_components.len() == 4);
//!
//! let mut g = Graph::new();
//! g.from_str(
//!     "A 0 0, B 200 0, C 0 200, D 160 160, E 250 200, F 100 300,
//!     G 100 100, A - B 2, A - G 5,
//!     B - G 15, B - D 10, B - E 3, C - G 5, C - D 7, C - E 10, C - F 12,
//!     D - G 3, D - E 1, E - F 11",
//! );
//! let mst_tree = Tree::minimal_spanning_tree(&g);
//!
//! let mut g = Graph::new();
//! g.from_str(
//!     "A 0 0, B 100 -100, C 200 -100, D 300 -100, E 300 100, F 200 150,
//!     G 200 50, H 100 100, I 100 0, J 100 -200, A > I, I > B, B > C, C > D, D > E, E < F,
//!     E > G, F > G, F > H,
//!     G < H, H < I, J - B",
//! );
//! let bfs_tree = Tree::bfs_tree(&g, 'A');
//!
//! let mut g = Graph::new();
//! g.from_str(
//!     "A 0 0, B 200 0, C 0 200, D 160 160, E 250 200, F 100 300,
//!     G 100 100, A - B, A - G, B - G, B - D, B - E, C - G, C - E, C - F,
//!     D - G, D - E, E - F",
//! );
//! let (e_g, _cycle) = Eulerian::hierholzer(&g);
//!
//! let mut g = Graph::new();
//! g.from_str(
//!     "A 0 0, B -80 200, C 100 100, D -100 100, E 80 200
//!     F 0 60, G -40 160, H 40 100, I 40 160, J -40 100,
//!     L 0 -60, M 160 100, N 120 240, O -120 240, P -160 100,
//!     Q -160 40, R -190 20, S -130 20,
//!     T 160 180, U 160 20, V 160 -60, W 240 180, X 240 20, Y 240 -60, Z 240 100
//!     A - F, A - D, A - C, C - H, C - E, E - I, E - B, B - G, B - D, D - J,
//!     J - H, J - I, F - I, F - G, H - G,
//!     A  - L, C - M, E - N, B - O, D - P,
//!     P - Q, Q - R, Q - S,
//!     V - X, V - Z, V - W, U - Y, U - Z, U - W, M - X, M - Y, M - W, T - X, T - Y, T - Z",
//! );
//! let (p_g, partitions) = Coloration::quick_partition(&g);
//! assert!(partitions.len() == 3);
//!
//! Graph::to_html(vec![
//!     ("Strongly connected components (Kosaraju)", vec![&scc_g]),
//!     ("Minimal spanning tree (Prim)", vec![&mst_tree]),
//!     ("BFS tree", vec![&bfs_tree]),
//!     ("Eulerian path (Hierholzer)", vec![&e_g]),
//!     ("Color - Quick partition", vec![&p_g]),
//! ])
//! .unwrap();
//! ```

pub mod algo;
pub mod graph;

#[cfg(test)]
mod tests {

    use crate::graph::Graph;

    #[test]
    fn it_works() {
        let config = "A
             B
             C
             A > B 1
             B - C 2";
        let mut graph = Graph::new();
        graph.from_str(config);

        graph.add_node('D', None);

        graph.add_link('C', 'D', true, 3);
        graph.add_link('D', 'A', false, 4);

        graph.swap_nodes('A', 'B');
        graph.swap_nodes('A', 'B');

        let color = (0, 196, 0);
        graph.color_node('A', color);
        graph.color_link('A', 'B', color);
        graph.color_node('B', color);
        graph.color_link('B', 'C', color);
        graph.color_node('C', color);
        graph.color_link('C', 'D', color);
        graph.color_node('D', color);
        graph.color_link('D', 'A', color);

        graph.sleep(1000);

        let timing_add = graph.duration();

        graph.pause();
        graph.add_node('E', None);
        graph.add_link('A', 'E', true, 5);
        graph.add_link('B', 'E', true, 6);
        graph.add_link('C', 'E', true, 7);
        graph.add_link('D', 'E', true, 8);
        graph.step(1000);
        graph.resume();

        graph.color_node('E', (0, 128, 0));
        graph.color_node('E', (128, 0, 0));

        let (x, y, _) = graph.node_position('E');
        graph.move_node('E', (x - 20, y - 20));
        graph.move_node('E', (x - 20, y + 20));
        graph.move_node('E', (x + 20, y + 20));
        graph.move_node('E', (x + 20, y - 20));
        graph.move_node('E', (x - 20, y - 20));
        graph.move_node('E', (x, y));

        let timing_delete = graph.duration();
        graph.delete_node('E');

        println!("nodes_list() -> {:?}", graph.nodes());

        for node in graph.nodes() {
            println!(
                "node_node_neighbors({:?}) -> {:?}",
                &node,
                graph.neighbors(node)
            );
        }
        println!("adjacency_list() -> {:?}", graph.adjacency_list());
        println!("adjacency_matrix() -> {:?}", graph.adjacency_matrix());

        let mut other_graph = Graph::new();
        other_graph.from_str(&graph.to_string());

        other_graph.pause();

        other_graph.step(timing_add - other_graph.duration());

        other_graph.add_node('E', None);
        other_graph.add_link('A', 'E', true, 5);
        other_graph.add_link('B', 'E', true, 6);
        other_graph.add_link('C', 'E', true, 8);
        other_graph.add_link('D', 'E', true, 9);

        other_graph.step(timing_delete - other_graph.duration());
        other_graph.resume();
        other_graph.delete_node('E');
        other_graph.sleep(2000);
        other_graph.fill_node('A', (128, 0, 0));
    }
}
