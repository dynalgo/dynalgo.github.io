use crate::graph::Graph;

pub struct Dfs {}

impl Dfs {
    pub fn new() -> Self {
        Dfs {}
    }

    /// Runs the Depth-First Search algorithm on a graph (starting from an optional start node).
    /// Returns a list of trees
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use dynalgo::algo::travers::Dfs;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// let dyna = String::from(
    ///        "N A _ _ 1
    ///         N B _ _ 2
    ///         N C _ _ 3
    ///         N D _ _ 4
    ///         N E _ _ 5
    ///         N F _ _ 6
    ///         N G _ _ 7
    ///         N H _ _ 8
    ///         N I _ _ 9
    ///         N J _ _ _
    ///         N K _ _ 11
    ///         N L _ _ 12
    ///         N M _ _ _
    ///         N N _ _ 14
    ///         N O _ _ 15
    ///         N P _ _ 16
    ///         N Q _ _ 17
    ///         N R _ _ 18
    ///         N S _ _ 19
    ///         N T _ _ 21
    ///         N U _ _ 22
    ///         L a B G true 1
    ///         L b F C true 2
    ///         L c B C true 3
    ///         L d C G true 4
    ///         L e G F false 5
    ///         L f F B true 6
    ///         L g F E true 7
    ///         L h F J true 8
    ///         L i E I true 9
    ///         L j I J false _
    ///         L k K J true 11
    ///         L l A J true 12
    ///         L m I A true 13
    ///         L n K G true 14
    ///         L o K D false 15
    ///         L p K H true 16
    ///         L q K L true 17
    ///         L r L M true 18
    ///         L s L S true 19
    ///         L t L O false _
    ///         L u N O true 21
    ///         L v N P true 22
    ///         L w P Q true 23
    ///         L x P R true 24
    ///         L y P T false 25
    ///         L z T U true 26"
    /// );
    /// graph.dyna_from(dyna);
    ///
    /// let dfs = Dfs::new();
    /// let mut trees = dfs.run(&mut graph, None);
    /// for (i, tree) in trees.iter_mut().enumerate() {
    ///     tree.svg_layout();
    ///     let html = tree.svg_render_animation_html("DFS");
    ///     let html_file_name = format!("example-Travers-DFS_tree{}.html",i);
    ///     write!(File::create(html_file_name).unwrap(), "{}", html).unwrap();
    /// }
    /// let html = graph.svg_render_animation_html("traversal DFS example");
    /// write!(File::create("example-Travers-DFS.html").unwrap(), "{}", html).unwrap();     
    /// ```
    pub fn run(&self, graph: &mut Graph, start_node: Option<char>) -> Vec<Graph> {
        let adjacencies = &graph.adjacency_list();
        let mut trees = Vec::new();
        let mut visited = Vec::new();

        graph.svg_param_duration_select(500);            
        graph.svg_param_duration_color(500);
        for (node_from, _) in adjacencies {
            match start_node {
                Some(n) => {
                    if n != *node_from {
                        continue;
                    }
                },
                None => {},
            };
            
            if visited.contains(node_from) {
                continue;
            }
            visited.push(*node_from);            
            graph.svg_node_color(*node_from, 0, 255, 0).unwrap();
            graph.svg_node_selected(*node_from, true).unwrap();

            let mut tree = Graph::new();
            tree.svg_automatic_animation(false);
            tree.node_add(*node_from, graph.node_value(*node_from).unwrap()).unwrap();
            tree.svg_layout();
            
            self.dfs_node(node_from, graph, &mut visited, &mut tree);
            trees.push(tree);
        }

        trees
    }

    fn dfs_node(
        &self,
        node_from: &char,
        graph: &mut Graph,
        visited: &mut Vec<char>,
        tree: &mut Graph,
    ) {
        let adjacencies = &graph.adjacency_list();

        for (node_to, link) in adjacencies.get(node_from).unwrap() {
            if visited.contains(node_to) {
                graph.svg_param_color_tag_selected(128, 0, 0);
                graph.svg_link_selected(link.0, true).unwrap();
                graph.svg_param_color_tag_selected(191, 255, 0);
                continue;
            }
            visited.push(*node_to);
            graph.svg_link_selected(link.0, true).unwrap();
            graph.svg_node_selected(*node_to, true).unwrap();
            graph.svg_node_color(*node_to, 191, 255, 0).unwrap();


            tree.node_add(*node_to, graph.node_value(*node_to).unwrap()).unwrap();
            tree.link_add(link.0, *node_from, *node_to, true, link.1).unwrap();

            self.dfs_node(node_to, graph, visited, tree);
        }
    }
}
