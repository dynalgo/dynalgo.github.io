use crate::algo::utils::color::Color;
use crate::algo::utils::transform::Transform;
use crate::graph::Graph;
pub struct Connectivity;

impl Connectivity {
    /// Returns a list of connected components
    pub fn components(g: &Graph) -> (Graph, Vec<Vec<char>>) {
        let mut cg = Graph::new();
        cg.from_str(&g.to_string());

        if cg.directed() {
            cg.pause();
            Transform::undirect(&mut cg);
            cg.resume();
            assert!(!cg.directed());
        }

        let nodes = cg.nodes();
        Self::dfs_components(cg, nodes)
    }

    fn dfs_components(mut cg: Graph, mut nodes: Vec<char>) -> (Graph, Vec<Vec<char>>) {
        let mut components = Vec::new();

        let colors = Color::colors();
        let mut it_colors = colors.into_iter().cycle();
        while let Some(start) = nodes.pop() {
            let already: Vec<char> = components.iter().flatten().cloned().collect();
            if already.contains(&start) {
                continue;
            }
            let mut visited = Vec::new();
            let mut backtracked = Vec::new();
            Self::dfs_components_rec(&mut cg, start, &mut visited, &mut backtracked, &already);
            assert!(visited.is_empty());
            let color = it_colors.next().unwrap();
            cg.pause();
            for node in backtracked.iter() {
                cg.color_node(*node, color);
            }
            cg.resume();
            components.push(backtracked);
        }

        (cg, components)
    }

    fn dfs_components_rec(
        g: &mut Graph,
        start: char,
        visited: &mut Vec<char>,
        backtracked: &mut Vec<char>,
        already: &Vec<char>,
    ) {
        if backtracked.contains(&start) || visited.contains(&start) || already.contains(&start) {
            return;
        }

        let prec_start = match visited.last() {
            Some(n) => *n,
            None => ' ',
        };
        visited.push(start);
        g.fill_node(start, (0, 196, 0));
        for neighbor in g.neighbors(start) {
            if neighbor == prec_start {
                continue;
            }

            Self::dfs_components_rec(g, neighbor, visited, backtracked, already);
        }
        assert!(visited.pop() == Some(start));
        backtracked.push(start);
        g.pause();
        g.fill_node(start, (255, 255, 255));
        g.color_label(start, (0, 196, 0));
        g.resume();
    }

    /// Returns a list of strongly connected components
    pub fn strongly_connected_components(g: &Graph) -> (Graph, Vec<Vec<char>>) {
        let mut cg = Graph::new();
        cg.from_str(&g.to_string());

        let nodes = cg.nodes();
        let (mut cg, backtracked) = Self::dfs_components(cg, nodes);
        let nodes: Vec<char> = backtracked.into_iter().flatten().collect();
        cg.pause();
        Transform::transpose(&mut cg);
        cg.resume();
        cg.pause();
        let color = Color::default();
        for node in cg.nodes() {
            cg.color_node(node, color);
        }
        cg.resume();
        let (mut cg, components) = Self::dfs_components(cg, nodes);
        cg.pause();
        Transform::transpose(&mut cg);
        cg.resume();

        (cg, components)
    }
}
