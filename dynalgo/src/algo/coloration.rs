use crate::algo::connectivity::Connectivity;
use crate::algo::utils::color::Color;
use crate::algo::utils::transform::Transform;
use crate::graph::Graph;
use std::f64::consts::PI;

pub struct Coloration {}

impl Coloration {
    /// Partitions nodes in empty subgraphs.
    /// Nodes in the same subgraph can be coloured with a single color.
    pub fn quick_partition(g: &Graph) -> (Graph, Vec<Vec<char>>) {
        let mut cg = Graph::new();
        cg.from_str(&g.to_string());
        if cg.directed() {
            cg.pause();
            Transform::undirect(&mut cg);
            cg.resume();
        }

        let (_, components) = Connectivity::components(g);
        assert!(components.len() == 1);

        let mut adj_list = g.adjacency_list();
        let mut leafs = Vec::new();
        let mut leaf_added = true;
        while leaf_added {
            let mut leafs_add = Vec::new();
            for (node_from, neighbors) in &adj_list {
                if neighbors.len() == 1 {
                    let (node_to, _) = adj_list.get(node_from).unwrap().first_key_value().unwrap();
                    leafs_add.push((*node_from, *node_to));
                }
            }
            if leafs_add.is_empty() {
                leaf_added = false;
                continue;
            }
            for (leaf, _) in &leafs_add {
                if adj_list.len() == 1 {
                    leaf_added = false;
                    break;
                }
                adj_list.remove(leaf);
                for (_, neighbors) in adj_list.iter_mut() {
                    neighbors.remove(leaf);
                }
                cg.color_node(*leaf, Color::disabled());
            }
            leafs.extend(leafs_add);
        }

        let mut nodes: Vec<char> = adj_list.iter().map(|(k, _)| *k).collect();
        let mut matrix = Vec::new();
        for node_from in &nodes {
            let mut v = Vec::new();
            for node_to in &nodes {
                v.push(adj_list.get(node_from).unwrap().get(node_to).is_some());
            }
            matrix.push(v);
        }

        let mut it_colors = Color::colors().into_iter().cycle();
        let mut color = it_colors.next().unwrap();
        let mut colors = Vec::new();

        let mut partitions = Vec::new();
        let mut partition = Vec::new();
        let mut left;
        let mut right = nodes.len() - 1;
        for i in 0..nodes.len() {
            left = i;

            let mut maxi = 0;
            let mut maxi_idx = 0;
            for j in left..=right {
                let cnt = matrix[j][left..=right].iter().filter(|x| !*x).count();
                if cnt > maxi {
                    maxi = cnt;
                    maxi_idx = j;
                }
            }
            assert!(maxi > 0);
            matrix.swap(i, maxi_idx);
            for k in 0..nodes.len() {
                matrix[k].swap(i, maxi_idx);
            }
            nodes.swap(i, maxi_idx);
            partition.push(nodes[i]);
            cg.color_node(nodes[i], color);

            while left < right {
                while !matrix[i][left] {
                    left += 1;
                    if left == right {
                        break;
                    }
                }
                while matrix[i][right] {
                    right -= 1;
                    if left == right {
                        break;
                    }
                }
                if left < right && matrix[i][left] && !matrix[i][right] {
                    matrix.swap(left, right);
                    for k in 0..nodes.len() {
                        matrix[k].swap(left, right);
                    }
                    nodes.swap(left, right);
                }
            }

            if i == nodes.len() - 1 || matrix[i][i + 1] {
                partitions.push(partition);
                partition = Vec::new();
                right = nodes.len() - 1;
                colors.push(color);

                color = it_colors.next().unwrap();
            }
        }

        for (leaf, neighbor) in leafs.iter().rev() {
            for (i, set) in partitions.iter().enumerate() {
                if set.contains(neighbor) {
                    for (j, color) in colors.iter().enumerate() {
                        if i != j {
                            partitions[j].push(*leaf);
                            cg.pause();
                            cg.color_node(*leaf, *color);
                            cg.fill_node(*leaf, Color::disabled());
                            cg.resume();
                            break;
                        }
                    }
                    break;
                }
            }
        }

        let perimeter = ((1.5 * g.node_radius() as f64) as u32
            * (nodes.len() + leafs.len() + partitions.len()) as u32
            * 2) as f64;
        let radius = (perimeter / (2. * PI)) as f64;
        let angle = 2. * PI / (nodes.len() + leafs.len() + partitions.len()) as f64;

        let mut i = 0;
        cg.sleep(2000);
        cg.pause();
        for set in partitions.iter() {
            for node in set.iter() {
                let x = (radius * (i as f64 * angle).cos()) as i32;
                let y = (radius * (i as f64 * angle).sin()) as i32;
                i += 1;
                cg.move_node(*node, (x, y));
            }
            i += 1;
        }
        cg.resume();

        (cg, partitions)
    }
}
