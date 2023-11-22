use crate::utils::names::Names;
use crate::utils::random::Random;
use dynalgo::graph::Graph;
use std::collections::HashMap;

pub struct Maze {}

impl Maze {
    pub fn solve(graph: &mut Graph) -> bool {
        let nodes = graph.nodes_list();
        if nodes.len() < 2 {
            return false;
        }

        let node_start = nodes[0];
        let node_searched = nodes[nodes.len() - 1];

        graph
            .param_duration_select(300)
            .param_duration_color(500)
            .param_color_tag_selected(0, 0, 255);
        graph.anim_node_select(node_start, true).unwrap();
        graph.anim_node_select(node_searched, true).unwrap();
        graph.param_color_tag_selected(191, 255, 0);
        Self::dfs_search(graph, node_start, node_searched, &mut Vec::new())
    }

    pub fn generate(dim: u8) -> Graph {
        let chars = Names::emoticon((dim * dim).into()).unwrap();

        let dim: u16 = dim as u16;
        let radius = 15;
        let mut graph = Graph::new();
        graph.anim_pause().unwrap();
        graph
            .param_display_link_value(false)
            .param_radius_node(radius as u32)
            .param_color_tag_created(128, 139, 150);

        let mut walls = Vec::new();
        let mut nodes = HashMap::new();
        for height in 0..dim {
            for width in 0..dim {
                let idx = width + height * dim;
                let c = chars[idx as usize];
                let x = (3 * radius * width) as i16;
                let y = (3 * radius * height) as i16;
                graph.node_add_freezed(c, x, y).unwrap();
                nodes.insert(c, idx);
                match height {
                    x if x == dim - 1 => {}
                    _ => {
                        walls.push((c, chars[(idx + dim) as usize]));
                    }
                };
                match width {
                    x if x == dim - 1 => {}
                    _ => {
                        walls.push((c, chars[(idx + 1) as usize]));
                    }
                };
            }
        }

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
                graph.link_add(node1, node2, true, 0).unwrap();
            }
            walls.remove(idx_random as usize);
        }

        graph.anim_resume().unwrap();

        graph
    }

    fn dfs_search(
        graph: &mut Graph,
        node_from: char,
        node_searched: char,
        visited: &mut Vec<char>,
    ) -> bool {
        visited.push(node_from);
        graph.anim_node_color(node_from, 0, 255, 0).unwrap();

        if node_from == node_searched {
            return true;
        }

        let adjacencies = &graph.adjacency_list();
        let mut found = false;
        for (node_to, _link) in adjacencies.get(&node_from).unwrap() {
            if visited.contains(node_to) {
                continue;
            }
            graph.anim_link_select(node_from, *node_to, true).unwrap();

            found = Self::dfs_search(graph, *node_to, node_searched, visited);
            if found {
                break;
            }
        }

        if !found {
            graph.anim_node_color(node_from, 255, 0, 0).unwrap();
        }
        found
    }
}
