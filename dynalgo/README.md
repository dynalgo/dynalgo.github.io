# dynalgo.github.io

`dynalgo` is a tiny RUST library designed to produce animated SVG images that can illustrate graph algorithms in action.

The library only focuces on providing a convenient tiny API for making animations in SVG SMIL format when developping algorithms working with graph structures.

The crate offers a basic `graph` structure representation. Interesting point is that each graph structure modification results in an animation rendered in SVG SMIL format into a HTML page. Several graphs animations can be rendered together in the same HTML page (side to side).

Dynalgo automatically layout nodes according to imaginary springs forces applying to them.
Custom animations can be made by playing with the nodes and links  graphical representations.

### Example: basic uses
[See example](https://dynalgo.github.io/dynalgo/Dynalgo_Rust_library_demo.html)
```
use dynalgo::graph::Graph;
use std::fs::File;
use std::io::Write;

let config = "A
        B
        C
        A > B 1
        B - C 2";
let mut graph = Graph::new();
graph.append_from_config(config);

graph.node_add('D');

graph.link_add('C', 'D', true, 3);
graph.link_add('D', 'A', false, 4);

graph.nodes_exchange('A', 'B');
graph.nodes_exchange('A', 'B');

graph.anim_node_select('A', true);
graph.anim_link_select('A', 'B', true);
graph.anim_node_select('B', true);
graph.anim_link_select('B', 'C', true);
graph.anim_node_select('C', true);
graph.anim_link_select('C', 'D', true);
graph.anim_node_select('D', true);
graph.anim_link_select('D', 'A', true);

let timing_add = graph.render_duration();

graph.anim_pause();
graph.node_add('E');
graph.link_add('A', 'E', true, 5);
graph.link_add('B', 'E', true, 6);
graph.link_add('C', 'E', true, 7);
graph.link_add('D', 'E', true, 8);
graph.anim_step(1000);
graph.anim_resume();

graph.anim_node_color('E', 0, 128, 0);
graph.anim_node_color('E', 128, 0, 0);

let (x, y, _) = graph.anim_node_position('E').unwrap();
graph.anim_node_move('E', x - 20, y - 20, false);
graph.anim_node_move('E', x - 20, y + 20, false);
graph.anim_node_move('E', x + 20, y + 20, false);
graph.anim_node_move('E', x + 20, y - 20, false);
graph.anim_node_move('E', x - 20, y - 20, false);
graph.anim_node_move('E', x, y, false);
        
let timing_delete = graph.render_duration();
graph.node_delete('E');


let mut other_graph = Graph::new();
other_graph.append_from_graph(&graph);

other_graph.anim_pause();

other_graph.anim_step(timing_add - other_graph.render_duration());

other_graph.node_add('E');
other_graph.link_add('A', 'E', true, 5);
other_graph.link_add('B', 'E', true, 6);
other_graph.link_add('C', 'E', true, 7);
other_graph.link_add('D', 'E', true, 8);

other_graph.anim_step(1);

other_graph.anim_step(timing_delete - other_graph.render_duration());
other_graph.anim_resume();
other_graph.node_delete('E');

let html = Graph::render_to_html("Dynalgo Rust library demo", vec![&graph, &other_graph]);
write!(File::create("Dynalgo_Rust_library_demo.html").unwrap(), "{}", html);
```


### Example: rendering to html files
[See example](https://dynalgo.github.io/dynalgo/K_3,4,5,6_complete_graph.html)
```
let mut pages = Vec::new();
let mut graphs = Vec::new();
for i in 3..12 {
    let names: Vec<char> = ('A'..'Z')
        .collect::<Vec<char>>()
        .into_iter()
        .take(i)
        .collect();
    let config =
        Graph::config_with_graph_sequence(vec![names.len() - 1; names.len()], names).unwrap();
    let mut graph = Graph::new();
    graph.append_from_config(&config);
    graphs.push(graph);
}
pages.push((
    "K 3,4,5,6 complete graph",
    vec![&graphs[0], &graphs[1], &graphs[2], &graphs[3]],
));
pages.push((
    "K 7,8,9 complete graph",
    vec![&graphs[4], &graphs[5], &graphs[6]],
));
pages.push(("K 10,11 complete graph", vec![&graphs[7], &graphs[8]]));

Graph::render_to_html_files(pages).unwrap();
```


### Example: maze demo
[See example](https://dynalgo.github.io/dynalgo/Dynalgo_maze_example.html)
```
use dynalgo::graph::Graph;
use std::fs::File;
use std::io::Write;

let config = "😀 0 0
			😁 45 0
			😂 90 0
			😃 135 0
			😄 180 0
			😅 225 0
			😆 270 0
			😇 315 0
			😈 0 45
			😉 45 45
			😊 90 45
			😋 135 45
			😌 180 45
			😍 225 45
			😎 270 45
			😏 315 45
			😐 0 90
			😑 45 90
			😒 90 90
			😓 135 90
			😔 180 90
			😕 225 90
			😖 270 90
			😗 315 90
			😘 0 135
			😙 45 135
			😚 90 135
			😛 135 135
			😜 180 135
			😝 225 135
			😞 270 135
			😟 315 135
			😠 0 180
			😡 45 180
			😢 90 180
			😣 135 180
			😤 180 180
			😥 225 180
			😦 270 180
			😧 315 180
			😨 0 225
			😩 45 225
			😪 90 225
			😫 135 225
			😬 180 225
			😭 225 225
			😮 270 225
			😯 315 225
			😰 0 270
			😱 45 270
			😲 90 270
			😳 135 270
			😴 180 270
			😵 225 270
			😶 270 270
			😷 315 270
			😸 0 315
			😹 45 315
			😺 90 315
			😻 135 315
			😼 180 315
			😽 225 315
			😾 270 315
			😿 315 315
			😀 - 😁 0
			😁 - 😉 0
			😂 - 😃 0
			😂 - 😊 0
			😃 - 😄 0
			😄 - 😅 0
			😅 - 😍 0
			😆 - 😎 0
			😇 - 😏 0
			😈 - 😉 0
			😈 - 😐 0
			😊 - 😒 0
			😋 - 😓 0
			😌 - 😔 0
			😎 - 😏 0
			😎 - 😖 0
			😐 - 😑 0
			😐 - 😘 0
			😑 - 😒 0
			😒 - 😓 0
			😓 - 😛 0
			😔 - 😕 0
			😕 - 😖 0
			😕 - 😝 0
			😗 - 😟 0
			😘 - 😠 0
			😙 - 😚 0
			😚 - 😢 0
			😜 - 😝 0
			😝 - 😥 0
			😞 - 😟 0
			😞 - 😦 0
			😠 - 😡 0
			😡 - 😢 0
			😡 - 😩 0
			😢 - 😪 0
			😣 - 😤 0
			😤 - 😬 0
			😥 - 😦 0
			😥 - 😭 0
			😦 - 😧 0
			😦 - 😮 0
			😧 - 😯 0
			😨 - 😩 0
			😩 - 😱 0
			😪 - 😫 0
			😪 - 😲 0
			😫 - 😬 0
			😬 - 😴 0
			😮 - 😶 0
			😯 - 😷 0
			😰 - 😱 0
			😱 - 😹 0
			😳 - 😻 0
			😴 - 😼 0
			😵 - 😶 0
			😶 - 😾 0
			😷 - 😿 0
			😸 - 😹 0
			😹 - 😺 0
			😻 - 😼 0
			😼 - 😽 0
			😽 - 😾 0";

let node_start = '😀';
let node_searched = '😿';

let mut freezed_maze = Graph::new();
let mut unfreezed_maze = Graph::new();

for graph in [&mut freezed_maze, &mut unfreezed_maze] {
    graph
        .param_display_link_value(false)
        .param_radius_node(15)
        .param_color_tag_created(128, 139, 150)
        .param_duration_select(300)
        .param_duration_color(500)
        .param_color_tag_selected(0, 0, 255);

    graph.append_from_config(config);

    graph.anim_node_select(node_start, true);
    graph.anim_node_select(node_searched, true);
    graph.param_color_tag_selected(191, 255, 0);
}

freezed_maze.anim_pause();
freezed_maze.anim_step(5000);
freezed_maze.anim_resume();
deep_first_search(
    &mut freezed_maze,
    node_start,
    node_searched,
    &mut Vec::new(),
);

unfreezed_maze.anim_pause();
unfreezed_maze.anim_step(3000);
for node in unfreezed_maze.nodes_list() {
    unfreezed_maze.anim_node_freeze(node, false);
}
unfreezed_maze.anim_step(2000);
unfreezed_maze.anim_resume();

deep_first_search(
    &mut unfreezed_maze,
    node_start,
    node_searched,
    &mut Vec::new(),
);

let html = Graph::render_to_html("Dynalgo maze example", vec![&freezed_maze, &unfreezed_maze]);
write!(File::create("Dynalgo_maze_example.html").unwrap(), "{}", html );

fn deep_first_search(
    graph: &mut Graph,
    node_from: char,
    node_searched: char,
    visited: &mut Vec<char>,
) -> bool {
    visited.push(node_from);
    graph.anim_node_color(node_from, 0, 255, 0);

    if node_from == node_searched {
        return true;
    }

    let adja = &graph.adjacency_list();
    let mut found = false;
    for (node_to, _link) in adja.get(&node_from).unwrap() {
        if visited.contains(node_to) {
            continue;
        }
        graph.anim_link_select(node_from, *node_to, true);

        found = deep_first_search(graph, *node_to, node_searched, visited);
        if found {
            break;
        }
    }

    if !found {
        graph.anim_node_color(node_from, 255, 0, 0);
    }
    found
}
```
