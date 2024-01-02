use crate::algo::connectivity::Connectivity;
use crate::algo::utils::color::Color;
use crate::graph::Graph;

pub struct Eulerian;

impl Eulerian {
    /// Returns an Eulerian path or cycle, if exists.
    pub fn hierholzer(g: &Graph) -> (Graph, Vec<char>) {
        let directed = g.directed();
        let mut eg = Graph::new();
        eg.from_str(&g.to_string());

        let mut cycle = Vec::new();

        let (_, components) = Connectivity::strongly_connected_components(&eg);
        assert!(components.len() == 1);

        let sequence = eg.sequence();
        let mut extra_link = None;
        if directed {
            let odd_node: Vec<(char, (usize, usize))> =
                sequence.into_iter().filter(|(_, (o, i))| o != i).collect();
            match odd_node.len() {
                0 => {}
                2 => {
                    let ((n1, (o1, i1)), (n2, (o2, i2))) = (odd_node[0], odd_node[1]);
                    assert!(o1 + o2 == i1 + i2);
                    let (node_from, node_to) = if o1 > i1 { (n2, n1) } else { (n1, n2) };
                    extra_link = Some((node_from, node_to));
                    eg.delete_link(node_to, node_from);
                    eg.add_link(node_from, node_to, true, 0);
                    eg.color_link(node_from, node_to, Color::disabled());
                }
                _ => return (eg, cycle),
            }
        } else {
            let odd_node: Vec<(char, (usize, usize))> = sequence
                .into_iter()
                .filter(|(_, (o, _))| o % 2 != 0)
                .collect();
            match odd_node.len() {
                0 => {}
                2 => {
                    let ((n1, _), (n2, _)) = (odd_node[0], odd_node[1]);
                    extra_link = Some((n2, n1));
                    eg.add_link(n2, n1, true, 0);
                    eg.color_link(n2, n1, Color::disabled());
                }
                _ => return (eg, cycle),
            }
        }
        let mut adj = eg.adjacency_list();

        let colors = Color::colors();
        let mut it_colors = colors.into_iter().cycle();
        let mut start = g.nodes()[0];
        while !adj.is_empty() {
            let color = it_colors.next().unwrap();
            //eg.color_node(start, color);
            let mut c = Vec::new();
            let mut prev = start;
            let mut next = prev;

            loop {
                c.push(next);
                prev = next;
                next = *adj[&prev].keys().next().unwrap();
                eg.color_link(prev, next, color);
                //eg.color_node(next, color);
                assert!(adj.get_mut(&prev).unwrap().remove(&next).is_some());
                if adj.get_mut(&prev).unwrap().is_empty() {
                    assert!(adj.remove(&prev).is_some());
                }
                if !directed {
                    assert!(adj.get_mut(&next).unwrap().remove(&prev).is_some());
                    if adj.get_mut(&next).unwrap().is_empty() {
                        assert!(adj.remove(&next).is_some());
                    }
                }

                if next == start {
                    break;
                }
            }

            if cycle.is_empty() {
                cycle.append(&mut c);
            } else {
                let pos = cycle.iter().position(|n| *n == start).unwrap();
                cycle.rotate_left(pos);
                cycle.append(&mut c);
            }

            for node in &cycle {
                if adj.get(node).is_some() {
                    start = *node;
                    break;
                }
            }
        }

        if let Some((extra_from, extra_to)) = extra_link {
            let mut prec = None;
            let mut idx = usize::MAX;
            for (i, node) in cycle.iter().enumerate() {
                if let Some(prec_node) = prec {
                    if (prec_node == extra_from && *node == extra_to)
                        || (!directed && prec_node == extra_to && *node == extra_from)
                    {
                        idx = i;
                        break;
                    }
                }
                prec = Some(*node);
            }
            cycle.rotate_left(idx);
            eg.delete_link(extra_from, extra_to);
            if directed {
                eg.add_link(extra_to, extra_from, false, 0);
            }
        } else {
            cycle.push(cycle[0]);
        }

        let color = it_colors.next().unwrap();
        let mut prec = None;
        for node in &cycle {
            if let Some(prec_node) = prec {
                eg.color_link(prec_node, *node, color);
            }
            //eg.color_node(*node, color);
            prec = Some(*node);
        }

        (eg, cycle)
    }
}
