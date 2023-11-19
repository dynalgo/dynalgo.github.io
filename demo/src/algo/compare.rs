use super::combination::Combination;
use dynalgo::graph::Graph;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub struct Compare {}

impl Compare {
    pub fn equal(graph_1: &Graph, graph_2: &Graph, link_value_check: bool) -> bool {
        if graph_1.graph_sequence() != graph_2.graph_sequence()
            || graph_1.nodes_list() != graph_2.nodes_list()
        {
            return false;
        }
        Self::equal_adj(
            graph_1.adjacency_list(),
            graph_2.adjacency_list(),
            link_value_check,
            None,
        )
    }

    pub fn isomorphic(graph_1: &Graph, graph_2: &Graph) -> Option<HashMap<char, char>> {
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
        //println!("combinations : {:?}", combinations);

        for combination in combinations {
            assert_eq!(combination.len(), nodes_names_2.len());
            let mut chars = combination.into_iter();
            let mut bijection = HashMap::new();
            for node_name in &nodes_names_2 {
                bijection.insert(*node_name, chars.next().unwrap());
            }

            //println!("bijection : {:?}", bijection);
            if Self::equal_adj(
                graph_1.adjacency_list(),
                graph_2.adjacency_list(),
                false,
                Some(&bijection),
            ) {
                let bijection: HashMap<char, char> = bijection.into_iter().collect();
                return Some(bijection);
            } else {
            }
        }

        None
    }

    fn equal_adj(
        adja_1: BTreeMap<char, BTreeMap<char, u8>>,
        adja_2: BTreeMap<char, BTreeMap<char, u8>>,
        link_value_check: bool,
        bijection_2: Option<&HashMap<char, char>>,
    ) -> bool {
        if adja_1.len() != adja_2.len() || adja_1.is_empty() {
            return false;
        }
        let adja_2 = match bijection_2 {
            None => adja_2,
            Some(bijection) => {
                let mut adja_bij = BTreeMap::new();
                for (node, neihgbors) in adja_2 {
                    let node_bij = bijection.get(&node).unwrap();
                    let entry_node_bij = adja_bij.entry(*node_bij).or_insert(BTreeMap::new());
                    for (node_to, link_value) in neihgbors {
                        let node_to_bij = bijection.get(&node_to).unwrap();
                        entry_node_bij.insert(*node_to_bij, link_value);
                    }
                }
                adja_bij
            }
        };
        let mut iter_1 = adja_1.iter();
        let mut iter_2 = adja_2.iter();
        for _ in 0..adja_1.len() {
            let (node_1, neighbors_1) = iter_1.next().unwrap();
            let (node_2, neighbors_2) = iter_2.next().unwrap();
            assert!(node_1 == node_2);
            for (neighbor, link_value) in neighbors_1 {
                match neighbors_2.get(neighbor) {
                    Some(v) => {
                        if link_value_check && v != link_value {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            for (neighbor, link_value) in neighbors_2 {
                match neighbors_1.get(neighbor) {
                    Some(v) => {
                        if link_value_check && v != link_value {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }

        true
    }
}
