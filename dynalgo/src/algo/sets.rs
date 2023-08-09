//! Set operations

use crate::algo::utils::Names;
use crate::graph::Graph;

pub struct Sets {}

impl Sets {
    /// creates a Graph resulting of the union of two graphs
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    /// use dynalgo::algo::sets::Sets;
    /// let mut graph1 = Graph::new();
    /// graph1.svg_automatic_animation(false);
    /// graph1.svg_automatic_layout(false);
    /// graph1.svg_param_duration_move(1);
    /// graph1.node_add('A', None).unwrap();
    /// graph1.node_add('B', None).unwrap();
    /// graph1.link_add('α', 'A', 'B', false, None);
    /// graph1.svg_layout();
    /// graph1.svg_animate(1);
    /// let mut graph2 = Graph::new();
    /// graph2.svg_automatic_animation(false);
    /// graph2.svg_automatic_layout(false);
    /// graph2.svg_param_duration_move(1);
    /// graph2.node_add('A', None).unwrap();
    /// graph2.node_add('B', None).unwrap();
    /// graph2.node_add('C', None).unwrap();
    /// graph2.link_add('α', 'B', 'C', false, None);
    /// graph2.link_add('β', 'B', 'A', false, None);
    /// graph2.svg_layout();
    /// graph2.svg_animate(1);
    /// let graph_union = Sets::union(&graph1, &graph2).unwrap();
    ///
    /// let html_file_content = Graph::svg_render_animations_html(
    ///     "sets-union example",
    ///     vec![&graph1, &graph2, &graph_union],
    /// );
    /// write!(File::create("example-sets-union.html").unwrap(), "{}", html_file_content);
    /// ```
    pub fn union(graph_1: &Graph, graph_2: &Graph) -> Result<Graph, String> {
        let mut g = graph_1.clone();

        g.svg_param_duration_select(500);
        g.svg_param_duration_color(500);
        g.svg_param_color_tag_selected(0, 255, 0);

        let adj_list_1 = g.adjacency_list();
        let adj_list_2 = graph_2.adjacency_list();

        for (node, _) in &adj_list_2 {
            if !adj_list_1.contains_key(node) {
                let value = graph_2.node_value(*node).unwrap();
                g.node_add(*node, value).unwrap();
                g.svg_node_color(*node, 0, 255, 0).unwrap();
            }
        }

        let mut link_names = Vec::new();
        for (node_from, adj) in &adj_list_1 {
            for (node_to, (link_name, link_value)) in adj {
                link_names.push(*link_name);
                let mut bidi = adj_list_1.get(&node_to).unwrap().get(&node_from).is_some();
                if !bidi && adj_list_2.contains_key(&node_from) && adj_list_2.contains_key(&node_to)
                {
                    bidi = adj_list_2.get(&node_to).unwrap().get(&node_from).is_some();
                    if bidi {
                        g.link_delete(*link_name).unwrap();
                        g.link_add(*link_name, *node_from, *node_to, bidi, *link_value)
                            .unwrap();
                        g.svg_link_selected(*link_name, true).unwrap();
                    }
                }
            }
        }

        let mut linked_nodes = Vec::new();
        let mut other_names: Vec<char> = Vec::new();
        for (node_from, adj) in &adj_list_2 {
            linked_nodes.push(*node_from);
            for (node_to, (link_name, link_value)) in adj {
                if adj_list_1.contains_key(&node_from) && adj_list_1.contains_key(&node_to) {
                    if adj_list_1.get(&node_from).unwrap().get(&node_to).is_some()
                        || adj_list_1.get(&node_to).unwrap().get(&node_from).is_some()
                    {
                        continue;
                    }
                }
                let bidi = adj_list_2.get(&node_to).unwrap().get(&node_from).is_some();
                if linked_nodes.contains(node_to) && bidi {
                    continue;
                }
                let mut link_name = *link_name;
                while link_names.contains(&link_name) {
                    if other_names.is_empty() {
                        other_names = Names::emoticon(300).unwrap();
                    }
                    link_name = match other_names.pop() {
                        Some(n) => n,
                        None => Err("There is no more available link name")?,
                    };
                }
                link_names.push(link_name);
                g.link_add(link_name, *node_from, *node_to, bidi, *link_value)
                    .unwrap();
                g.svg_link_selected(link_name, true).unwrap();
            }
        }

        Ok(g)
    }

    /// creates a Graph resulting of the intersection of two graphs
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    /// use dynalgo::algo::sets::Sets;
    /// let mut graph1 = Graph::new();
    /// graph1.svg_automatic_animation(false);
    /// graph1.svg_automatic_layout(false);
    /// graph1.svg_param_duration_move(1);
    /// graph1.node_add('A', None).unwrap();
    /// graph1.node_add('B', None).unwrap();
    /// graph1.node_add('C', None).unwrap();
    /// graph1.node_add('D', None).unwrap();
    /// graph1.link_add('α', 'A', 'B', false, None);
    /// graph1.link_add('β', 'C', 'B', true, None);
    /// graph1.svg_layout();
    /// graph1.svg_animate(1);
    /// let mut graph2 = Graph::new();
    /// graph2.svg_automatic_animation(false);
    /// graph2.svg_automatic_layout(false);
    /// graph2.svg_param_duration_move(1);
    /// graph2.node_add('A', None).unwrap();
    /// graph2.node_add('B', None).unwrap();
    /// graph2.node_add('C', None).unwrap();
    /// graph2.link_add('α', 'B', 'C', false, None);
    /// graph2.svg_layout();
    /// graph2.svg_animate(1);
    /// let graph_intersec = Sets::intersection(&graph1, &graph2).unwrap();
    ///
    /// let html = Graph::svg_render_animations_html(
    ///     "sets-intersection example",
    ///     vec![&graph1, &graph2, &graph_intersec],
    /// );
    /// write!(File::create("example-sets-intersection.html").unwrap(), "{}", html);
    /// ```
    pub fn intersection(graph_1: &Graph, graph_2: &Graph) -> Result<Graph, String> {
        let mut g = graph_1.clone();

        g.svg_param_duration_select(500);
        g.svg_param_duration_color(500);
        g.svg_param_color_tag_selected(0, 255, 0);

        let adj_list_1 = g.adjacency_list();
        let adj_list_2 = graph_2.adjacency_list();
        for (node, _) in &adj_list_1 {
            if !adj_list_2.contains_key(node) {
                g.node_delete(*node).unwrap();
            }
        }
        let adj_list_1 = g.adjacency_list();

        let mut linked_nodes = Vec::new();
        for (node_from, adj) in &adj_list_1 {
            linked_nodes.push(*node_from);
            for (node_to, (link_name, link_value)) in adj {
                if linked_nodes.contains(node_to)
                    && adj_list_1.get(&node_to).unwrap().get(&node_from).is_some()
                {
                    continue;
                }
                match (
                    adj_list_1.get(&node_to).unwrap().get(&node_from),
                    adj_list_2.get(&node_from).unwrap().get(&node_to),
                    adj_list_2.get(&node_to).unwrap().get(&node_from),
                ) {
                    (None, Some(_), _) | (Some(_), Some(_), Some(_)) => {}
                    (_, None, None) | (None, None, _) => {
                        g.link_delete(*link_name).unwrap();
                    }
                    (Some(_), Some(_), None) => {
                        g.link_delete(*link_name).unwrap();
                        g.link_add(*link_name, *node_from, *node_to, false, *link_value)
                            .unwrap();
                        g.svg_link_selected(*link_name, true).unwrap();
                    }
                    (Some(_), None, Some(_)) => {
                        g.link_delete(*link_name).unwrap();
                        g.link_add(*link_name, *node_to, *node_from, false, *link_value)
                            .unwrap();
                        g.svg_link_selected(*link_name, true).unwrap();
                    }
                }
            }
        }

        Ok(g)
    }

    /// creates the complementary of a graph
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    /// use dynalgo::algo::sets::Sets;
    /// let mut graph = Graph::new();
    /// graph.svg_automatic_animation(false);
    /// graph.svg_automatic_layout(false);
    /// graph.svg_param_duration_move(1);
    /// graph.node_add('A', None).unwrap();
    /// graph.node_add('B', None).unwrap();
    /// graph.node_add('C', None).unwrap();
    /// graph.node_add('D', None).unwrap();
    /// graph.link_add('α', 'A', 'B', false, None);
    /// graph.link_add('β', 'C', 'B', true, None);
    /// graph.svg_layout();
    /// graph.svg_animate(1);
    /// let graph_compl = Sets::complementary(&graph).unwrap();
    ///
    /// let html = Graph::svg_render_animations_html(
    ///     "sets-complementary example",
    ///     vec![&graph, &graph_compl],
    /// );
    /// write!(File::create("example-sets-complementary.html").unwrap(), "{}", html);
    /// ```
    pub fn complementary(graph: &Graph) -> Result<Graph, String> {
        let mut g = graph.clone();

        g.svg_param_duration_select(500);
        g.svg_param_duration_color(500);
        g.svg_param_color_tag_selected(0, 255, 0);

        let mut linked_nodes = Vec::new();
        let adj_list = g.adjacency_list();
        let nodes_list = g.nodes_list();

        let mut links_deleted = Vec::new();
        for (_, adj) in &adj_list {
            for (_, (link_name, _)) in adj {
                if !links_deleted.contains(link_name) {
                    g.link_delete(*link_name).unwrap();
                    links_deleted.push(*link_name);
                }
            }
        }

        let mut other_names = Names::emoticon(300).unwrap();

        for (node_from, adj) in &adj_list {
            linked_nodes.push(*node_from);
            for node_to in &nodes_list {
                if linked_nodes.contains(&node_to) {
                    continue;
                }

                if let (Some(_), Some(_)) = (
                    adj.get(&node_to),
                    adj_list.get(&node_to).unwrap().get(node_from),
                ) {
                    continue;
                }

                let link_name = match other_names.pop() {
                    Some(n) => n,
                    None => Err("There is no more available link name")?,
                };
                match (
                    adj.get(&node_to),
                    adj_list.get(&node_to).unwrap().get(node_from),
                ) {
                    (Some(_), Some(_)) => {
                        continue;
                    }

                    (Some((_, _)), None) => {
                        g.link_add(link_name, *node_to, *node_from, false, None)
                            .unwrap();
                    }
                    (None, Some((_, _))) => {
                        g.link_add(link_name, *node_from, *node_to, false, None)
                            .unwrap();
                    }
                    (None, None) => {
                        g.link_add(link_name, *node_from, *node_to, true, None)
                            .unwrap();
                    }
                }
                g.svg_link_selected(link_name, true).unwrap();
            }
        }

        Ok(g)
    }
}
