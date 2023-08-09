use crate::algo::utils::Names;
use crate::algo::utils::Random;
use crate::graph::Graph;
use std::collections::HashMap;

pub struct Maze {}

impl Maze {
    pub fn new() -> Self {
        Maze {}
    }

    /// Constructs a maze, and then the algorithm traverses the graph to reach the arrival.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::algo::fun::Maze;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let maze = Maze::new();
    /// let graph = maze.run(9);
    /// let html = graph.svg_render_animation_html("maze example");
    /// write!(File::create("example-fun-maze.html").unwrap(), "{}", html).unwrap();
    /// ```

    pub fn run(&self, dimension: u8) -> Graph {
        let chars = Names::emoticon(79).unwrap();

        let dimension_max = (chars.len() as f64).sqrt().floor() as usize;
        let dimension = if dimension as usize > dimension_max {
            dimension_max
        } else if (dimension as usize) < 4 {
            4
        } else {
            dimension as usize
        };
        let radius: usize = 15;

        let mut graph = Graph::new();
        graph.svg_param_display_link_value(false);
        graph.svg_param_radius_node(radius as u32);
        graph.svg_automatic_animation(false);
        graph.svg_automatic_layout(false);
        graph.svg_param_color_tag_created(128, 139, 150);

        let mut walls = Vec::new();
        let mut nodes = HashMap::new();
        for height in 0..dimension {
            for width in 0..dimension {
                let idx = width + height * dimension;
                let c = chars[idx as usize];
                let x = (3 * radius * width) as i16;
                let y = (3 * radius * height) as i16;
                graph.node_add_fixed(c, x, y, None).unwrap();
                nodes.insert(c, idx);
                match height {
                    x if x == dimension - 1 => {}
                    _ => {
                        walls.push((c, chars[(idx + dimension) as usize]));
                    }
                };
                match width {
                    x if x == dimension - 1 => {}
                    _ => {
                        walls.push((c, chars[(idx + 1) as usize]));
                    }
                };
            }
        }

        let mut link_idx = 0;
        while walls.len() > 0 {
            let idx_random = Random::poor_random(walls.len() as u32);
            let (node1, node2) = walls[idx_random as usize];

            let set1 = nodes.get(&node1).unwrap().clone();
            let set2 = nodes.get(&node2).unwrap().clone();
            if set1 != set2 {
                for (_, set) in nodes.iter_mut() {
                    if *set == set2 {
                        *set = set1;
                    }
                }
                graph
                    .link_add(chars[link_idx], node1, node2, true, None)
                    .unwrap();
                link_idx += 1;
            }
            walls.remove(idx_random as usize);
        }
        graph.svg_layout();
        graph.svg_animate(1);
        graph.svg_param_duration_select(300);
        graph.svg_param_duration_color(500);
        graph.svg_automatic_animation(true);

        let node_start = chars[0];
        let node_searched = chars[(dimension * dimension) as usize - 1];
        graph.svg_param_color_tag_selected(0, 0, 255);
        graph.svg_node_selected(node_start, true).unwrap();
        graph.svg_node_selected(node_searched, true).unwrap();
        graph.svg_param_color_tag_selected(191, 255, 0);
        self.dfs_search(&mut graph, node_start, node_searched, &mut Vec::new());

        graph.svg_layout();

        graph
    }

    fn dfs_search(
        &self,
        graph: &mut Graph,
        node_from: char,
        node_searched: char,
        visited: &mut Vec<char>,
    ) -> bool {
        visited.push(node_from);
        graph.svg_node_color(node_from, 0, 255, 0).unwrap();

        if node_from == node_searched {
            return true;
        }

        let adjacencies = &graph.adjacency_list();
        let mut found = false;
        for (node_to, link) in adjacencies.get(&node_from).unwrap() {
            if visited.contains(node_to) {
                continue;
            }
            graph.svg_link_selected(link.0, true).unwrap();

            found = self.dfs_search(graph, *node_to, node_searched, visited);
            if found {
                break;
            }
        }

        if !found {
            graph.svg_node_color(node_from, 255, 0, 0).unwrap();
        }
        found
    }
}
