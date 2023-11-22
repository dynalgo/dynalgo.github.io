use super::combination::Combination;
use crate::utils::colors::Colors;
use dynalgo::graph::Graph;
use dynalgo::graph::GraphError;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub struct Compare {}

impl Compare {
    pub fn equal(graph_1: &mut Graph, graph_2: &mut Graph, link_value_check: bool) -> bool {
        Self::equal_adj(graph_1, graph_2, link_value_check, None).unwrap()
    }

    pub fn isomorphic(graph_1: &mut Graph, graph_2: &mut Graph) -> Option<HashMap<char, char>> {
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

        for combination in combinations {
            assert_eq!(combination.len(), nodes_names_2.len());
            let mut chars = combination.into_iter();
            let mut bijection = HashMap::new();
            for node_name in &nodes_names_2 {
                bijection.insert(*node_name, chars.next().unwrap());
            }

            if Self::equal_adj(graph_1, graph_2, false, Some(&bijection)).unwrap() {
                let colors = Colors::colors(graph_1.nodes_list().len());
                graph_1.param_duration_color(300);
                graph_2.param_duration_color(300);
                graph_1.anim_pause().unwrap();
                graph_2.anim_pause().unwrap();

                for (i, (node, node_bij)) in bijection.iter().enumerate() {
                    let (r, g, b) = colors[i];
                    graph_1.anim_node_color(*node_bij, r, g, b).unwrap();
                    graph_2.anim_node_color(*node, r, g, b).unwrap();
                }
                graph_1.anim_resume().unwrap();
                graph_2.anim_resume().unwrap();

                let bijection: HashMap<char, char> = bijection.into_iter().collect();
                return Some(bijection);
            }
        }

        None
    }

    fn equal_adj(
        g_1: &mut Graph,
        g_2: &mut Graph,
        link_value_check: bool,
        bij_2: Option<&HashMap<char, char>>,
    ) -> Result<bool, GraphError> {
        let adja_1 = g_1.adjacency_list();
        let adja_2 = g_2.adjacency_list();
        if bij_2.is_none() {
            g_1.param_color_tag_selected(0, 255, 0)
                .param_duration_select(300)
                .param_duration_color(500);
            g_2.param_color_tag_selected(0, 255, 0)
                .param_duration_select(300)
                .param_duration_color(500);
        }
        //if g_1.sequence() != g_2.sequence() {
        if adja_1.len() != adja_2.len() {
            if bij_2.is_none() {
                g_2.param_color_tag_selected(196, 0, 0);
                g_2.anim_nodes_select(g_2.nodes_list(), true)?;
            }
            return Ok(false);
        }

        let adja_2 = match bij_2 {
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
            if bij_2.is_none() {
                g_1.anim_node_select(*node_1, true).unwrap();
            }

            let (node_2, neighbors_2) = iter_2.next().unwrap();
            if node_1 != node_2 {
                if bij_2.is_none() {
                    g_2.param_color_tag_selected(255, 0, 0);
                    g_2.anim_node_select(*node_2, true).unwrap();
                }
                return Ok(false);
            }
            if bij_2.is_none() {
                g_2.anim_node_select(*node_2, true).unwrap();
            }
            for (neighbor, link_value) in neighbors_1 {
                match neighbors_2.get(neighbor) {
                    Some(v) => {
                        if link_value_check && v != link_value {
                            if bij_2.is_none() {
                                g_1.param_color_tag_selected(255, 0, 0);
                                g_1.anim_link_select(*node_1, *neighbor, true)?;
                                g_2.param_color_tag_selected(255, 0, 0);
                                g_2.anim_link_select(*node_2, *neighbor, true)?;
                            }
                            return Ok(false);
                        }
                    }
                    None => {
                        if bij_2.is_none() {
                            g_1.param_color_tag_selected(255, 0, 0);
                            g_1.anim_link_select(*node_1, *neighbor, true)?;
                            g_1.param_color_tag_selected(255, 0, 0);
                            g_1.anim_node_select(*neighbor, true).unwrap();
                        }
                        return Ok(false);
                    }
                }
            }
            for (neighbor, link_value) in neighbors_2 {
                match neighbors_1.get(neighbor) {
                    Some(v) => {
                        if link_value_check && v != link_value {
                            if bij_2.is_none() {
                                g_1.param_color_tag_selected(255, 0, 0);
                                g_1.anim_link_select(*node_1, *neighbor, true)?;
                                g_2.param_color_tag_selected(255, 0, 0);
                                g_2.anim_link_select(*node_2, *neighbor, true)?;
                            }
                            return Ok(false);
                        }
                    }
                    None => {
                        if bij_2.is_none() {
                            g_2.param_color_tag_selected(255, 0, 0);
                            g_2.anim_link_select(*node_2, *neighbor, true)?;
                            g_2.param_color_tag_selected(255, 0, 0);
                            g_2.anim_node_select(*neighbor, true).unwrap();
                        }
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }
}
