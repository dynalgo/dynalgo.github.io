use crate::algo::connectivity::Connectivity;
use crate::algo::utils::transform::Transform;
use crate::graph::Graph;
use std::cmp::max;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Tree {}

impl Tree {
    // Layouts the graph as a tree
    pub fn layout_as_tree(g: &Graph, root: char) -> Graph {
        Self::bfs(g, root, true)
    }

    // Returns a tree from BFS traversing
    pub fn bfs_tree(g: &Graph, root: char) -> Graph {
        Self::bfs(g, root, false)
    }

    fn bfs(g: &Graph, root: char, layout: bool) -> Graph {
        let (_, components) = Connectivity::components(g);
        assert!(components.len() == 1);

        let mut tree = Graph::new();
        tree.from_str(&g.to_string());
        if tree.directed() {
            tree.pause();
            Transform::undirect(&mut tree);
            tree.resume();
        }

        let mut steps: BTreeMap<i32, BTreeMap<char, Vec<char>>> = BTreeMap::new();
        let mut parents_order: BTreeMap<i32, Vec<char>> = BTreeMap::new();
        let mut visited = HashSet::new();
        let mut inqueued = HashSet::new();
        let mut queue = VecDeque::new();
        let mut deleted_links = Vec::new();
        queue.push_back((0, ' ', root));
        inqueued.insert(root);
        tree.color_label(root, (0, 255, 0));
        while let Some((step, parent, child)) = queue.pop_front() {
            inqueued.remove(&child);
            visited.insert(child);
            tree.color_node(child, (0, 192, 0));

            steps
                .entry(step)
                .or_default()
                .entry(parent)
                .or_default()
                .push(child);
            let po_entry = parents_order.entry(step + 1).or_default();
            if !po_entry.contains(&child) {
                po_entry.push(child);
            }

            for node in tree.neighbors(child) {
                if node == child {
                    continue;
                }
                if visited.contains(&node) {
                    continue;
                }
                if inqueued.contains(&node) {
                    tree.color_link(child, node, (192, 0, 0));
                    tree.delete_link(child, node);
                    deleted_links.push((child, node));
                    continue;
                }

                tree.color_label(node, (0, 255, 0));
                inqueued.insert(node);
                queue.push_back((step + 1, child, node));
            }
        }

        tree.pause();
        let (x_r, y_r, _) = tree.node_position(root);
        let d = 60;
        for (step, parents) in &steps {
            if *step == 0 {
                continue;
            }
            let y_c = y_r + step * d;
            let mut x_c = x_r;
            for parent in parents_order.get(step).unwrap() {
                if let Some(childs) = parents.get(parent) {
                    let (x_p, _, _) = tree.node_position(*parent);
                    x_c = max(x_c, x_p);
                    for child in childs {
                        tree.move_node(*child, (x_c, y_c));
                        x_c += d;
                    }
                }
            }
        }

        for (step, parents) in steps.iter().rev() {
            if *step == 0 {
                continue;
            }

            let mut x_prec = 0;
            for parent in parents_order.get(step).unwrap() {
                let (mut x_p, y_p, _) = tree.node_position(*parent);
                if let Some(childs) = parents.get(parent) {
                    x_p = 0;
                    for child in childs {
                        let (x_c, _, _) = tree.node_position(*child);
                        x_p += x_c;
                    }
                    x_p /= childs.len() as i32;
                } else {
                    x_p = max(x_p, x_prec + d);
                }
                tree.move_node(*parent, (x_p, y_p));
                x_prec = x_p;
            }
        }
        tree.resume();

        if layout {
            for (node_from, node_to) in deleted_links {
                tree.add_link(node_from, node_to, true, 0);
            }
        }

        tree
    }

    // Returns a minimal spanning tree
    pub fn minimal_spanning_tree(g: &Graph) -> Graph {
        let mut stg = Graph::new();
        stg.from_str(&g.to_string());

        if stg.directed() {
            stg.pause();
            Transform::undirect(&mut stg);
            stg.resume();
        }

        let (_, components) = Connectivity::components(&stg);
        assert!(components.len() == 1);

        let nodes = stg.nodes();
        assert!(!nodes.is_empty());
        let mut start = nodes[0];
        stg.color_node(start, (0, 192, 0));
        stg.color_label(start, (0, 255, 0));

        let mut links = Vec::new();
        let adj = stg.adjacency_list();
        for (node_to, link_value) in adj.get(&start).unwrap() {
            links.push((start, *node_to, *link_value));
            stg.color_label(*node_to, (0, 255, 0));
        }

        let mut visited = Vec::new();
        visited.push(start);
        while !links.is_empty() {
            links.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));
            let (node_from, node_to, _) = links.pop().unwrap();
            if visited.contains(&node_to) {
                stg.color_link(node_from, node_to, (192, 0, 0));
                stg.delete_link(node_from, node_to);
                continue;
            }
            stg.color_link(node_from, node_to, (0, 192, 0));
            stg.color_node(node_to, (0, 196, 0));
            visited.push(node_to);
            start = node_to;
            for (node_to, link_value) in adj.get(&start).unwrap() {
                if visited.contains(&node_to) {
                    continue;
                }
                links.push((start, *node_to, *link_value));
                stg.color_label(*node_to, (0, 255, 0));
            }
        }

        stg
    }
}
