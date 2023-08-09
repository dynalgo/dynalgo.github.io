//! Compare two graphs (equality or isomorphism)

use crate::algo::utils::Combination;
use crate::graph::Graph;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub struct Compare {}

impl Compare {
    /// Returns true if the graphs are equal
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::algo::compare::Compare;
    /// use dynalgo::graph::Graph;
    /// let mut graph1 = Graph::new();
    /// graph1.node_add('A', Some(11));
    /// graph1.node_add('B', Some(12));
    /// graph1.node_add('C', Some(13));
    /// graph1.node_add('D', Some(14));
    /// graph1.link_add('?', 'A', 'B', false, Some(15));
    /// graph1.link_add('β', 'C', 'B', true, Some(16));
    /// let mut graph2 = Graph::new();
    /// graph2.node_add('A', Some(11));
    /// graph2.node_add('B', Some(12));
    /// graph2.node_add('C', Some(13));
    /// graph2.node_add('D', Some(14));
    /// graph2.link_add('α', 'A', 'B', false, Some(15));
    /// graph2.link_add('β', 'C', 'B', true, Some(16));
    /// let equal = Compare::equal(&graph1, &graph2, false, true, true);
    /// assert!(equal);
    /// ```
    pub fn equal(
        graph_1: &Graph,
        graph_2: &Graph,
        link_name_check: bool,
        node_value_check: bool,
        link_value_check: bool,
    ) -> bool {
        if graph_1.graph_sequence() != graph_2.graph_sequence() {
            return false;
        }

        let mut nodes_values_1: HashMap<char, Option<u8>> = HashMap::new();
        let mut nodes_values_2: HashMap<char, Option<u8>> = HashMap::new();
        if node_value_check {
            for node in graph_1.nodes_list() {
                nodes_values_1.insert(node, graph_1.node_value(node).unwrap());
            }
            for node in graph_2.nodes_list() {
                nodes_values_2.insert(node, graph_2.node_value(node).unwrap());
            }
        }
        Self::equal_adj(
            &graph_1.adjacency_list(),
            &graph_2.adjacency_list(),
            &nodes_values_1,
            &nodes_values_2,
            link_name_check,
            node_value_check,
            link_value_check,
            None,
        )
    }

    /// Returns a Map that is a bijection from the nodes of graph_2 to the nodes of graph_1.
    /// The return value is None if the graphs are not isomorphic.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::algo::compare::Compare;
    /// use dynalgo::algo::utils::Colors;
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    /// let mut graph1 = Graph::new();
    /// graph1.node_add('A', None);
    /// graph1.node_add('B', None);
    /// graph1.node_add('C', None);
    /// graph1.node_add('D', None);
    /// graph1.link_add('a', 'A', 'B', false, None);
    /// graph1.link_add('b', 'C', 'B', true, None);
    /// let mut graph2 = Graph::new();
    /// graph2.node_add('E', None);
    /// graph2.node_add('F', None);
    /// graph2.node_add('G', None);
    /// graph2.node_add('H', None);
    /// graph2.link_add('e', 'E', 'F', false, None);
    /// graph2.link_add('f', 'G', 'F', true, None);
    /// let bijection = Compare::isomorphic(&graph1, &graph2);
    /// assert!(bijection.is_some());
    /// let mut colors = Colors::colors(graph1.nodes_list().len()).into_iter();
    /// for (node, node_bij) in bijection.unwrap().iter() {
    ///     let (r, g, b) = colors.next().unwrap();
    ///     graph1.svg_node_color(*node_bij, r, g, b);
    ///     graph2.svg_node_color(*node, r, g, b);
    /// }
    /// let html_file_content = Graph::svg_render_animations_html(
    ///     "svg_render_animations_html example",
    ///     vec![&graph1, &graph2],
    /// );
    /// let mut html_file = File::create("example-compare-isomorphic.html").unwrap();
    /// write!(html_file, "{}", html_file_content);
    /// ```
    pub fn isomorphic(graph_1: &Graph, graph_2: &Graph) -> Option<BTreeMap<char, char>> {
        if graph_1.graph_sequence() != graph_2.graph_sequence() {
            return None;
        }

        let degrees = graph_1.nodes_degrees();
        let mut degrees_nodes_1 = BTreeMap::new();
        for (node, degree) in degrees.into_iter() {
            degrees_nodes_1
                .entry(degree)
                .or_insert(Vec::new())
                .push(node);
        }

        let degrees = graph_2.nodes_degrees();
        let mut degrees_nodes_2 = BTreeMap::new();
        for (node, degree) in degrees.into_iter() {
            degrees_nodes_2
                .entry(degree)
                .or_insert(Vec::new())
                .push(node);
        }

        let mut elements_list: Vec<(Vec<char>, usize, usize)> = Vec::new();
        for (_degree, nodes) in degrees_nodes_1.into_iter().rev() {
            let mut elements = Vec::new();
            for node in nodes.into_iter() {
                elements.push(node);
            }
            let length = elements.len();
            elements_list.push((elements, length, length));
        }

        let mut nodes_names_2 = Vec::new();
        for (_degree, nodes) in degrees_nodes_2.into_iter().rev() {
            for node in nodes.into_iter() {
                nodes_names_2.push(node);
            }
        }
        let combinations = Combination::combine::<char>(elements_list);
        println!("combinations : {:?}", combinations);

        for combination in combinations {
            assert_eq!(combination.len(), nodes_names_2.len());
            let mut chars = combination.into_iter();
            let mut bijection = HashMap::new();
            for node_name in &nodes_names_2 {
                bijection.insert(*node_name, chars.next().unwrap());
            }

            println!("bijection : {:?}", bijection);
            if Self::equal_adj(
                &graph_1.adjacency_list(),
                &graph_2.adjacency_list(),
                &HashMap::new(),
                &HashMap::new(),
                false,
                false,
                false,
                Some(&bijection),
            ) {
                let bijection: BTreeMap<char, char> = bijection.into_iter().collect();
                return Some(bijection);
            } else {
            }
        }

        None
    }

    fn equal_adj(
        adj_list_1: &HashMap<char, HashMap<char, (char, Option<u8>)>>,
        adj_list_2: &HashMap<char, HashMap<char, (char, Option<u8>)>>,
        nodes_values_1: &HashMap<char, Option<u8>>,
        nodes_values_2: &HashMap<char, Option<u8>>,
        link_name_check: bool,
        node_value_check: bool,
        link_value_check: bool,
        bijection_2: Option<&HashMap<char, char>>,
    ) -> bool {
        let mut adj_list_2 = match bijection_2 {
            None => adj_list_2.clone(),
            Some(bijection) => {
                assert!(!node_value_check);
                let mut adj_list_bij: HashMap<char, HashMap<char, (char, Option<u8>)>> =
                    HashMap::new();
                for (node_2, adj_2) in adj_list_2 {
                    let node_bij = bijection.get(node_2).unwrap();
                    let entry_node_bij = adj_list_bij.entry(*node_bij).or_insert(HashMap::new());
                    for (node_to_2, (link_name_2, link_value_2)) in adj_2 {
                        let node_to_bij = bijection.get(node_to_2).unwrap();
                        entry_node_bij.insert(*node_to_bij, (*link_name_2, *link_value_2));
                    }
                }
                adj_list_bij
            }
        };
        for (node_1, adj_1) in adj_list_1 {
            match adj_list_2.get_mut(node_1) {
                None => return false,
                Some(adj_2) => {
                    if node_value_check {
                        assert!(bijection_2.is_none()); // case not supported
                        let node_value_1 = nodes_values_1.get(node_1);
                        let node_value_2 = nodes_values_2.get(node_1);
                        match (node_value_1, node_value_2) {
                            (Some(v1), Some(v2)) if v1 != v2 => {
                                return false;
                            }
                            (Some(_), None) | (None, Some(_)) => {
                                return false;
                            }
                            _ => {}
                        }
                    }
                    for (node_to_1, (link_name_1, link_value_1)) in adj_1 {
                        match adj_2.get(node_to_1) {
                            None => return false,
                            Some((link_name_2, link_value_2)) => {
                                if link_name_check {
                                    if link_name_1 != link_name_2 {
                                        return false;
                                    }
                                }
                                if link_value_check {
                                    match (link_value_1, link_value_2) {
                                        (Some(v1), Some(v2)) if v1 != v2 => {
                                            return false;
                                        }
                                        (Some(_), None) | (None, Some(_)) => {
                                            return false;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        adj_2.remove(&node_to_1);
                    }
                    if !adj_2.is_empty() {
                        return false;
                    }
                }
            }
            adj_list_2.remove(node_1);
        }
        if !adj_list_2.is_empty() {
            return false;
        }

        true
    }
}
