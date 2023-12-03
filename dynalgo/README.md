# dynalgo.github.io

`dynalgo` is a tiny RUST library designed to produce animated SVG images that can illustrate graph algorithms in action.

The library focuces on providing a convenient tiny API for making animations in SVG SMIL format when developping algorithms working with graph structures.

The crate offers a basic `graph` structure representation. Interesting point is that each graph structure modification results in an animation rendered in SVG SMIL format into a HTML page. Several graphs animations can be rendered together in the same HTML page (side to side).

Dynalgo automatically layout nodes according to imaginary springs forces applying to them.
Custom animations can be made by playing with the nodes and links  graphical representations.

### Example: traversing a maze
[See example](https://dynalgo.github.io/dynalgo/Dynalgo_maze_example.html)
```
use dynalgo::graph::Graph;

fn main() {
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

    for graph in [&mut freezed_maze, &mut unfreezed_maze].iter_mut() {
        graph.from_str(config);
        graph.fill_node(node_start, (0, 0, 196));
        graph.fill_node(node_searched, (0, 0, 196));
    }

    deep_first_search(
        &mut freezed_maze,
        node_start,
        node_searched,
        &mut Vec::new(),
    );

    unfreezed_maze.pause();
    for node in unfreezed_maze.nodes() {
        unfreezed_maze.layout_node(node);
    }
    unfreezed_maze.resume();

    deep_first_search(
        &mut unfreezed_maze,
        node_start,
        node_searched,
        &mut Vec::new(),
    );

    Graph::to_html(vec![(
        "Dynalgo maze example",
        vec![&freezed_maze, &unfreezed_maze],
    )])
    .unwrap();
}

fn deep_first_search(
    graph: &mut Graph,
    node_from: char,
    node_searched: char,
    visited: &mut Vec<char>,
) -> bool {
    visited.push(node_from);
    graph.color_node(node_from, (0, 255, 0));

    if node_from == node_searched {
        return true;
    }

    let adja = &graph.adjacency_list();
    let mut found = false;
    for (node_to, _link) in adja.get(&node_from).unwrap() {
        if visited.contains(node_to) {
            continue;
        }
        graph.color_link(node_from, *node_to, (0, 255, 0));

        found = deep_first_search(graph, *node_to, node_searched, visited);
        if found {
            break;
        }
    }

    if !found {
        graph.color_node(node_from, (255, 0, 0));
    }
    found
}
```
