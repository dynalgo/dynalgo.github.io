//! Basic `graph` structure representation with animation properties.

mod renderer;

use renderer::color::Color;
use renderer::html::Html;
use renderer::point::Point;
use renderer::Renderer;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug)]
pub struct GraphError {
    action: String,
    message: String,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A Dynalgo graph error occurred while trying to {}.\nCause is : \"{}\"",
            self.action, self.message
        )
    }
}

#[derive(PartialEq, Clone, Copy)]
enum AnimState {
    Paused,
    Resumed,
}

pub struct Graph {
    renderer: Renderer,
    auto_animation: bool,
    adja: BTreeMap<char, BTreeMap<char, u8>>,
    anim_state: AnimState,
    layout_on_resume: bool,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.graph_config())
    }
}

impl FromStr for Graph {
    type Err = String;
    fn from_str(s: &str) -> Result<Graph, String> {
        let mut graph = Graph::new();
        match graph.graph_from_config(s) {
            Ok(_) => (),
            Err(graph_error) => Err(format!("{}", graph_error))?,
        }
        Ok(graph)
    }
}

impl Graph {
    /// Creates an empty graph structure.
    pub fn new() -> Graph {
        Graph {
            renderer: Renderer::new(),
            auto_animation: false,
            adja: BTreeMap::new(),
            anim_state: AnimState::Resumed,
            layout_on_resume: false,
        }
    }

    /// Appends graph structure elements from an existing graph.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A');
    /// graph.node_add('B');
    /// graph.link_add('A', 'B', true, 0);
    ///
    /// let mut other_graph = Graph::new();
    /// other_graph.append_from_graph(&graph);
    ///
    /// assert!(other_graph.nodes_list() == vec!['A', 'B']);
    /// assert!(other_graph.graph_config() == graph.graph_config());
    /// ```
    pub fn append_from_graph(&mut self, graph: &Graph) -> Result<&mut Self, GraphError> {
        self.append_from_config(&graph.to_string())?;

        Ok(self)
    }

    /// Appends graph structure elements from a graph configuration.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let config = "A
    ///               B
    ///               A - B 0";
    /// let mut graph = Graph::new();
    /// graph.append_from_config(config);
    ///
    /// assert!(graph.nodes_list() == vec!['A', 'B']);
    /// ```
    pub fn append_from_config(&mut self, graph_config: &str) -> Result<&mut Self, GraphError> {
        self.graph_from_config(graph_config)?;

        Ok(self)
    }

    fn node_add_from(&mut self, name: &str, cx: &str, cy: &str) -> Result<&mut Self, GraphError> {
        if name.chars().count() != 1 {
            Err(GraphError {
                action: String::from("add a node"),
                message: format!("'{}' is an invalid node name (char type requiered)", name),
            })?
        }
        let name = name.chars().next().unwrap();

        if cx != "_" {
            let cx = match cx.parse::<i16>() {
                Ok(v) => v,
                Err(_) => Err(GraphError {
                    action: String::from("add a node"),
                    message: format!("'{}' is an invalid x coordinate for node {}", cx, name),
                })?,
            };

            let cy = match cy.parse::<i16>() {
                Ok(v) => v,
                Err(_) => Err(GraphError {
                    action: String::from("add a node"),
                    message: format!("'{}' is an invalid y coordinate for node {}", cy, name),
                })?,
            };

            self.node_add_xy(name, Some((cx, cy)))?;
        } else {
            self.node_add(name)?;
        }

        Ok(self)
    }

    /// Adds a node to the graph structure.
    pub fn node_add(&mut self, name: char) -> Result<&mut Self, GraphError> {
        self.node_add_xy(name, None)
    }

    /// Adds a node to the graph structure, with a freezed (x,y) position (freezed coords of the element representation in SVG context). So the (x,y) position won't change when automatic layout algo runs.
    pub fn node_add_freezed(
        &mut self,
        name: char,
        x: i16,
        y: i16,
    ) -> Result<&mut Self, GraphError> {
        self.node_add_xy(name, Some((x, y)))
    }

    fn node_add_xy(&mut self, name: char, xy: Option<(i16, i16)>) -> Result<&mut Self, GraphError> {
        self.node_check_not_exist(name)?;

        self.anim_bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            Some(vec![(name, xy)]),
            None,
            self.renderer.p_duration_add,
        )?;

        self.anim_layout_need();

        Ok(self)
    }

    /// Deletes a node from the graph structure.
    pub fn node_delete(&mut self, node: char) -> Result<&mut Self, GraphError> {
        let links = self.node_links(node)?;

        let links_to_delete: Vec<(char, char)> = links
            .iter()
            .map(|(node_from, node_to, _, _)| (*node_from, *node_to))
            .collect();

        self.anim_bulk_changes(
            None,
            None,
            None,
            None,
            Some(links_to_delete),
            Some(vec![node]),
            None,
            None,
            self.renderer.p_duration_delete,
        )?;

        self.anim_layout_need();

        Ok(self)
    }

    fn link_add_from(
        &mut self,
        node_from: &str,
        node_to: &str,
        bidirect: bool,
        value: &str,
    ) -> Result<&mut Self, GraphError> {
        if node_from.chars().count() != 1 {
            Err(GraphError {
                action: String::from("add a link"),
                message: format!(
                    "'{}' is an invalid node name (char type requiered)",
                    node_from
                ),
            })?
        }
        if node_to.chars().count() != 1 {
            Err(GraphError {
                action: String::from("add a link"),
                message: format!(
                    "'{}' is an invalid node name (char type requiered)",
                    node_to
                ),
            })?
        }

        let node_from = node_from.chars().next().unwrap();
        let node_to = node_to.chars().next().unwrap();
        let value = if value == "_" {
            0
        } else {
            match value.parse::<u8>() {
                Ok(v) => v,
                Err(_) => Err(GraphError {
                    action: String::from("add a link"),
                    message: format!(
                        "'{}' is an invalid value for link {}{}",
                        value, node_from, node_to
                    ),
                })?,
            }
        };

        self.link_add(node_from, node_to, bidirect, value)?;

        Ok(self)
    }

    /// Adds a link between two nodes. The link can be defined as bidirectional or not.
    pub fn link_add(
        &mut self,
        node_from: char,
        node_to: char,
        bidirectional: bool,
        value: u8,
    ) -> Result<&mut Self, GraphError> {
        self.link_check_not_exist(node_from, node_to)?;

        self.anim_bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(vec![(node_from, node_to, bidirectional, value)]),
            self.renderer.p_duration_add,
        )?;

        self.anim_layout_need();

        Ok(self)
    }

    /// Deletes a link.
    pub fn link_delete(&mut self, node_from: char, node_to: char) -> Result<&mut Self, GraphError> {
        self.link_check_exists(node_from, node_to)?;

        self.anim_bulk_changes(
            None,
            None,
            None,
            None,
            Some(vec![(node_from, node_to)]),
            None,
            None,
            None,
            self.renderer.p_duration_delete,
        )?;

        self.anim_layout_need();

        Ok(self)
    }

    /// Returns the list of nodes names.
    pub fn nodes_list(&self) -> Vec<char> {
        self.adja.keys().cloned().collect()
    }

    /// Returns the adjacency list.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let config = "A
    ///               B
    ///               A - B 0";
    /// let mut graph = Graph::new();
    /// graph.append_from_config(config);
    ///
    /// for (node_from, neighbors) in graph.adjacency_list() {
    ///     for (node_to, link_value) in neighbors {
    ///         println!("Can go from {} to {} (link value is {}).", node_from, node_to, link_value);
    ///     }
    /// }
    /// ```
    pub fn adjacency_list(&self) -> BTreeMap<char, BTreeMap<char, u8>> {
        self.adja.clone()
    }

    fn adjacency_list_undirected(&self) -> BTreeMap<char, BTreeMap<char, u8>> {
        let mut adja = self.adjacency_list();

        for (node_from, neighbors) in &self.adja {
            for (node_to, value) in neighbors {
                if self.adja.get(node_to).unwrap().get(node_from).is_none() {
                    adja.get_mut(node_to).unwrap().insert(*node_from, *value);
                }
            }
        }

        adja
    }

    /// Returns the nodes degrees.
    pub fn nodes_degrees(&self) -> BTreeMap<char, usize> {
        let mut degrees = BTreeMap::new();

        for (node_from, neighbors) in &self.adja {
            degrees.insert(*node_from, neighbors.len());
        }

        for (node_from, neighbors) in &self.adja {
            for (node_to, _) in neighbors {
                if self.adja.get(&node_to).unwrap().get(&node_from).is_none() {
                    let degree = degrees.get_mut(&node_to).unwrap();
                    *degree += 1;
                }
            }
        }

        degrees
    }

    fn node_links(&self, node: char) -> Result<Vec<(char, char, bool, u8)>, GraphError> {
        self.node_check_exists(node)?;

        let mut links = Vec::new();

        for (neighbor, value) in self.adja.get(&node).unwrap().iter() {
            let bidirect = self.adja.get(&neighbor).unwrap().get(&node).is_some();
            links.push((node, *neighbor, bidirect, *value));
        }
        for (node_other, neighbors_other) in self.adja.iter() {
            if *node_other == node {
                continue;
            };
            if let Some(value) = neighbors_other.get(&node) {
                if self.adja.get(&node).unwrap().get(&node_other).is_none() {
                    links.push((*node_other, node, false, *value));
                }
            }
        }

        Ok(links)
    }

    /// Returns the neighbors accessible from a node.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let config = "A
    ///               B
    ///               A - B 0";
    /// let mut graph = Graph::new();
    /// graph.append_from_config(config);
    ///
    /// for (neighbor, link_value) in graph.node_neighbors('A').unwrap() {
    ///     println!("Can go from 'A' to {} (link value is {}).", neighbor, link_value);
    /// }
    /// ```
    pub fn node_neighbors(&self, node: char) -> Result<Vec<(char, u8)>, GraphError> {
        self.node_check_exists(node)?;

        let neighbors = self
            .adja
            .get(&node)
            .unwrap()
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect();

        Ok(neighbors)
    }

    /// Exchanges two nodes in the graph structure and its graphic representation.
    pub fn nodes_exchange(&mut self, node_1: char, node_2: char) -> Result<&mut Self, GraphError> {
        self.node_check_exists(node_1)?;
        self.node_check_exists(node_2)?;

        let (x1, y1, freezed_1) = self.anim_node_position(node_1)?;
        let (x2, y2, freezed_2) = self.anim_node_position(node_2)?;

        let mut links = self.node_links(node_1)?;
        let links_2 = self.node_links(node_2)?;
        links.extend(&links_2);

        let links_to_delete: Vec<(char, char)> = links
            .iter()
            .map(|(node_from, node_to, _, _)| (*node_from, *node_to))
            .collect();

        let mut links_to_add = Vec::new();
        for (node_from, node_to, bidirect, value) in links {
            let node_from = match node_from {
                n if n == node_1 => node_2,
                n if n == node_2 => node_1,
                n => n,
            };
            let node_to = match node_to {
                n if n == node_1 => node_2,
                n if n == node_2 => node_1,
                n => n,
            };
            links_to_add.push((node_from, node_to, bidirect, value));
        }

        self.anim_bulk_changes(
            None,
            None,
            None,
            None,
            Some(links_to_delete),
            None,
            None,
            None,
            self.renderer.p_duration_delete,
        )?;

        self.anim_bulk_changes(
            None,
            None,
            Some(vec![
                (node_1, (x2, y2), freezed_2),
                (node_2, (x1, y1), freezed_1),
            ]),
            None,
            None,
            None,
            None,
            None,
            self.renderer.p_duration_move,
        )?;

        self.anim_bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(links_to_add),
            self.renderer.p_duration_add,
        )?;

        self.anim_layout_need();

        Ok(self)
    }

    /// Returns the graph order.
    pub fn graph_order(&self) -> usize {
        self.adja.len()
    }

    /// Returns the graph size.
    pub fn graph_size(&self) -> usize {
        let sum: usize = self.graph_sequence().iter().sum();
        assert!(sum % 2 == 0);
        sum / 2
    }

    /// Returns the sequence (degrees in decreasing order).
    pub fn graph_sequence(&self) -> Vec<usize> {
        let degrees = self.nodes_degrees();
        let mut sequence = Vec::new();
        for (_, degree) in degrees {
            sequence.push(degree);
        }
        sequence.sort_by(|a, b| b.cmp(a));
        sequence
    }

    fn graph_from_config(&mut self, graph_config: &str) -> Result<&mut Self, GraphError> {
        let anim_state_init = self.anim_state;
        if anim_state_init == AnimState::Resumed {
            self.anim_pause()?;
        }

        for lign in graph_config.lines() {
            let fields: Vec<&str> = lign.trim().split_whitespace().collect();
            match fields.as_slice() {
                [node_1, "-", node_2, value] => {
                    self.link_add_from(node_1, node_2, true, value)?;
                }
                [node_1, "-", node_2] => {
                    self.link_add_from(node_1, node_2, true, "_")?;
                }
                [node_from, ">", node_to, value] => {
                    self.link_add_from(node_from, node_to, false, value)?;
                }
                [node_from, ">", node_to] => {
                    self.link_add_from(node_from, node_to, false, "_")?;
                }
                [node, cx, cy] => {
                    self.node_add_from(node, cx, cy)?;
                }
                [node] => {
                    self.node_add_from(node, "_", "_")?;
                }
                _ => Err(GraphError {
                    action: String::from("parse config"),
                    message: format!("line '{}' is invalid", lign),
                })?,
            }
        }

        if anim_state_init == AnimState::Resumed {
            self.anim_resume()?;
        }

        Ok(self)
    }

    /// Returns a formatted string describing the graph structure.
    pub fn graph_config(&self) -> String {
        let mut config = String::new();

        for (node_from, _) in &self.adja {
            let (x, y, freezed) = self.anim_node_position(*node_from).unwrap();
            let lign = match freezed {
                true => format!("{} {} {}\n", node_from, x, y),
                false => format!("{}\n", node_from),
            };
            config.push_str(&lign);
        }

        for (node_from, neighbors) in &self.adja {
            for (node_to, value) in neighbors {
                let bidirect = match self.adja[&node_to].get(&node_from) {
                    Some(_) => "-",
                    None => ">",
                };
                if bidirect == "-" && node_from > node_to {
                    continue;
                }
                let lign = format!("{} {} {} {}\n", node_from, bidirect, node_to, value);
                config.push_str(&lign);
            }
        }

        config
    }

    fn node_check_exists(&self, node: char) -> Result<(), GraphError> {
        if self.adja.get(&node).is_none() {
            Err(GraphError {
                action: String::from("use a node"),
                message: format!("node '{}' does not exist", node),
            })?;
        };

        Ok(())
    }

    fn node_check_not_exist(&self, node: char) -> Result<(), GraphError> {
        if self.adja.get(&node).is_some() {
            Err(GraphError {
                action: String::from("add a node"),
                message: format!("node {} already exists", node),
            })?;
        };

        Ok(())
    }

    fn link_check_exists(&self, node_from: char, node_to: char) -> Result<(), GraphError> {
        self.node_check_exists(node_from)?;
        self.node_check_exists(node_to)?;
        if self.adja.get(&node_from).unwrap().get(&node_to).is_none() {
            Err(GraphError {
                action: String::from("use a link"),
                message: format!("link '{}{}' does not exist", node_from, node_to),
            })?;
        };

        Ok(())
    }

    fn link_check_not_exist(&self, node_from: char, node_to: char) -> Result<(), GraphError> {
        self.node_check_exists(node_from)?;
        self.node_check_exists(node_to)?;

        if node_from == node_to {
            Err(GraphError {
                action: String::from("add a link"),
                message: format!(
                    "link {}{} is invalid (loop is not allowed)",
                    node_from, node_to
                ),
            })?;
        }

        if self.adja.get(&node_from).unwrap().get(&node_to).is_some() {
            Err(GraphError {
                action: String::from("add a link"),
                message: format!("link {}{} already exists", node_from, node_to),
            })?;
        };

        if self.adja.get(&node_to).unwrap().get(&node_from).is_some() {
            Err(GraphError {
                action: String::from("add a link"),
                message: format!("directed link {}{} already exists", node_to, node_from),
            })?;
        };

        Ok(())
    }

    /// Reactivates the `auto animation` option.
    /// When this option is activated, each graph structure change causes graphic animation (the animations are rendered one after the other). By default, the option is activated when a graph is created.
    /// When this option is deactivated, all pending animations occur during the same period (i.e. not one after the other) when you manually call the `anim_step()` or `anim_resume()`functions.
    pub fn anim_resume(&mut self) -> Result<&mut Self, GraphError> {
        match self.anim_state {
            AnimState::Paused => {
                self.anim_step(1)?;
                self.anim_state = AnimState::Resumed;
            }
            _ => Err(GraphError {
                action: String::from("resume animation"),
                message: "animation has not been paused previoulsy".to_string(),
            })?,
        }

        self.auto_animation = true;

        Ok(self)
    }

    /// Deactivates the `auto animation` option.
    pub fn anim_pause(&mut self) -> Result<&mut Self, GraphError> {
        match self.anim_state {
            AnimState::Resumed => {
                self.anim_state = AnimState::Paused;
            }
            _ => Err(GraphError {
                action: String::from("pause animation"),
                message: "animation has not been resumed previoulsy".to_string(),
            })?,
        }
        self.auto_animation = false;

        Ok(self)
    }

    /// Creates an animation that show the evolution between the last represented state (when `anim_pause()` function was called previously) and the current state. The pending animations are rendered simultaneously (i.e. not one after the other).
    /// The `anim_pause()` function must have been called previously.
    /// After calling `anim_step()` function,  `auto animation` option still is deactivated.
    pub fn anim_step(&mut self, duration_ms: u32) -> Result<&mut Self, GraphError> {
        match self.anim_state {
            AnimState::Paused => {}
            _ => Err(GraphError {
                action: String::from("step animation"),
                message: "animation has not been paused previously".to_string(),
            })?,
        }
        if self.layout_on_resume {
            self.anim_layout();
            self.layout_on_resume = false;
        }
        self.anim_animate(duration_ms);

        Ok(self)
    }

    /// Renders graphs animations in SVG SMIL format to HTML format content.
    /// The HTML code contains a little javascript so that each animation can be paused and resumed by clicking on the image.
    pub fn render_to_html(title: &str, graphs: Vec<&Graph>) -> String {
        let mut svgs = Vec::new();
        for graph in graphs {
            svgs.push(graph.anim_animation());
        }
        Html::render_flexbox(&vec![title.to_string()], 0, &vec!["".to_string()], svgs)
    }

    /// Renders graphs animations in SVG SMIL format into multiple HTML files.
    /// Each HTML page contains a menu to access other pages (if there is more than one page).
    pub fn render_to_html_files(pages: Vec<(&str, Vec<&Graph>)>) -> Result<(), std::io::Error> {
        let titles: Vec<String> = pages.iter().map(|(title, _)| title.to_string()).collect();
        let mut file_names: Vec<String> = Vec::new();
        for title in &titles {
            file_names.push(title.replace(" ", "_"));
        }
        for (i, (_title, graphs)) in pages.iter().enumerate() {
            let mut svgs = Vec::new();
            for graph in graphs {
                svgs.push(graph.anim_animation());
            }
            let html = Html::render_flexbox(&titles, i, &file_names, svgs);
            write!(
                File::create(format!("{}.{}", file_names[i], "html"))?,
                "{}",
                html
            )?;
        }
        Ok(())
    }

    /// Returns the total duration in milliseconds of rendered animations since the graph was created.
    pub fn render_duration(&self) -> u32 {
        self.renderer.duration()
    }

    /// Returns the current x,y coords of the node in the SVG graphic context.
    pub fn anim_node_position(&self, node: char) -> Result<(i32, i32, bool), GraphError> {
        self.node_check_exists(node)?;

        let point = self.renderer.node_center(node);
        let freezed = self.renderer.node_center_freezed(node);
        Ok((point.x(), point.y(), freezed))
    }

    /// Changes the x,y coords of the SVG node representation. If `freezed` parameter is set to `True` then node position will not change when automatic layout algo runs.
    pub fn anim_node_move(
        &mut self,
        node: char,
        cx: i32,
        cy: i32,
        freezed: bool,
    ) -> Result<&mut Self, GraphError> {
        self.node_check_exists(node)?;

        self.anim_bulk_changes(
            None,
            None,
            Some(vec![(node, (cx, cy), freezed)]),
            None,
            None,
            None,
            None,
            None,
            self.renderer.p_duration_move,
        )
    }

    /// Changes the fill color of the SVG node representation.
    pub fn anim_node_color(
        &mut self,
        node: char,
        red: u8,
        green: u8,
        blue: u8,
    ) -> Result<&mut Self, GraphError> {
        self.node_check_exists(node)?;

        self.anim_bulk_changes(
            None,
            None,
            None,
            Some((vec![node], (red, green, blue))),
            None,
            None,
            None,
            None,
            self.renderer.p_duration_color,
        )
    }

    /// Changes the selected status of a node (affects only SVG rendering).
    /// The selected status affects the stroke color of the node.
    pub fn anim_node_selected(
        &mut self,
        node: char,
        selected: bool,
    ) -> Result<&mut Self, GraphError> {
        self.node_check_exists(node)?;

        self.anim_bulk_changes(
            Some((vec![node], selected)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            self.renderer.p_duration_select,
        )
    }

    /// Freezes or unfreezes the node position (coords in SVG graphic context), so the current position will change or not when automatic layout algo runs.
    pub fn anim_node_freeze(&mut self, node: char, set: bool) -> Result<&mut Self, GraphError> {
        self.node_check_exists(node)?;
        self.renderer.node_freezed(node, set);

        if !set {
            self.anim_layout_need();
        }

        Ok(self)
    }

    /// Changes the selected status of a link (affects only SVG rendering).
    /// The selected status affects the stroke color of the link.
    pub fn anim_link_selected(
        &mut self,
        node_from: char,
        node_to: char,
        selected: bool,
    ) -> Result<&mut Self, GraphError> {
        self.link_check_exists(node_from, node_to)?;

        self.anim_bulk_changes(
            None,
            Some((vec![(node_from, node_to)], selected)),
            None,
            None,
            None,
            None,
            None,
            None,
            self.renderer.p_duration_select,
        )
    }

    fn anim_animate(&mut self, duration_ms: u32) -> &mut Self {
        let duration_ms = match duration_ms {
            0 => 1,
            d => d,
        };
        self.renderer.animate(duration_ms);

        self
    }

    fn anim_layout_need(&mut self) -> &mut Self {
        if self.anim_state == AnimState::Resumed {
            self.anim_layout();
        } else {
            self.layout_on_resume = true;
        }

        self
    }

    fn anim_layout(&mut self) -> &mut Self {
        self.renderer.layout(self.adjacency_list_undirected());
        self.anim_animate(self.renderer.p_duration_move);

        self
    }

    fn anim_animation(&self) -> String {
        self.renderer.animation()
    }

    fn anim_bulk_changes(
        &mut self,
        nodes_selected: Option<(Vec<char>, bool)>, // ( nodes, selected )
        links_selected: Option<(Vec<(char, char)>, bool)>, // ( links, selected )
        nodes_move: Option<Vec<(char, (i32, i32), bool)>>, // Vec<(node, (cx: i32, cy: i32, freezed))>
        nodes_color: Option<(Vec<char>, (u8, u8, u8))>,    // ( nodes, (red, green, blue) )
        links_deleted: Option<Vec<(char, char)>>,
        nodes_deleted: Option<Vec<char>>,
        nodes_added: Option<Vec<(char, Option<(i16, i16)>)>>,
        links_added: Option<Vec<(char, char, bool, u8)>>,
        duration_ms: u32,
    ) -> Result<&mut Self, GraphError> {
        let anim_state_init = self.anim_state;
        if anim_state_init == AnimState::Resumed {
            self.anim_pause()?;
        }

        match nodes_selected {
            Some((nodes, selected)) => {
                for node in nodes {
                    self.renderer.node_selected(node, selected);
                }
            }
            None => {}
        }
        match links_selected {
            Some((links, selected)) => {
                for (node_from, node_to) in links {
                    self.renderer.link_selected(node_from, node_to, selected);
                }
            }
            None => {}
        }
        match nodes_move {
            Some(moves) => {
                for (node, (cx, cy), freezed) in moves {
                    self.renderer.node_move(node, Point::new(cx, cy), freezed);
                }
            }
            None => {}
        }
        match nodes_color {
            Some((nodes, (red, green, blue))) => {
                for node in nodes {
                    self.renderer.node_color(node, (red, green, blue));
                }
            }
            None => {}
        }
        match links_deleted {
            Some(links) => {
                for (node_from, node_to) in links {
                    self.adja.get_mut(&node_from).unwrap().remove(&node_to);
                    self.adja.get_mut(&node_to).unwrap().remove(&node_from);

                    self.renderer.link_delete(node_from, node_to);
                }
            }
            None => {}
        }
        match nodes_deleted {
            Some(nodes) => {
                for node in nodes {
                    self.adja.remove(&node);
                    self.renderer.node_delete(node);
                }
            }
            None => {}
        }
        match nodes_added {
            Some(nodes) => {
                for (node, xy) in nodes {
                    self.adja.insert(node, BTreeMap::new());

                    let point = match xy {
                        Some((x, y)) => Some(Point::new(x as i32, y as i32)),
                        None => None,
                    };
                    self.renderer.node_add(node, point);
                }
            }
            None => {}
        }
        match links_added {
            Some(links) => {
                for (node_from, node_to, bidirect, value) in links {
                    self.adja
                        .get_mut(&node_from)
                        .unwrap()
                        .insert(node_to, value);
                    if bidirect {
                        self.adja
                            .get_mut(&node_to)
                            .unwrap()
                            .insert(node_from, value);
                    }
                    self.renderer.link_add(node_from, node_to, bidirect, value);
                }
            }
            None => {}
        }

        if anim_state_init == AnimState::Resumed {
            self.anim_step(duration_ms)?;
            self.anim_resume()?;
        }

        Ok(self)
    }

    /// Creates a config for an undirected and unvalued graph with a defined sequence.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let config = Graph::config_with_graph_sequence(vec![4,3,3,3,3], vec!['A','B','C','D','E']).unwrap();
    /// let mut graph = Graph::new();
    /// graph.append_from_config(&config);
    ///
    /// assert_eq!(graph.graph_sequence(), vec![4,3,3,3,3]);
    /// ```
    pub fn config_with_graph_sequence(
        sequence: Vec<usize>,
        node_names: Vec<char>,
    ) -> Result<String, GraphError> {
        if node_names.len() != sequence.len() {
            Err(GraphError {
                action: String::from("create a config for a graph with a defined sequence"),
                message: format!(
                    "the sequence length is different from the node_names length ({} <> {})",
                    sequence.len(),
                    node_names.len()
                ),
            })?;
        }

        let mut sequence: Vec<(char, usize)> =
            node_names.into_iter().zip(sequence.into_iter()).collect();

        let mut graph = Graph::new();
        graph.anim_pause()?;

        while !sequence.is_empty() {
            sequence.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
            let (node, degree) = sequence.remove(0);
            if !graph.nodes_list().contains(&node) {
                graph.node_add(node)?;
            }
            if degree == 0 {
                continue;
            }
            if sequence.len() < degree {
                Err(GraphError {
                    action: String::from("create a config for a graph with a defined sequence"),
                    message: String::from(
                        "the sequence is not valid (graph with such a sequence can't exist)",
                    ),
                })?;
            }
            for (n, d) in sequence.iter_mut().take(degree) {
                *d -= 1;
                if !graph.nodes_list().contains(n) {
                    graph.node_add(*n)?;
                }
                graph.link_add(node, *n, true, 0)?;
            }
        }

        Ok(graph.graph_config())
    }

    /// Changes the value of the `duration_add` parameter.
    /// This parameter affects the animation duration when node or link is added.
    /// Default value is 1000 ms
    pub fn param_duration_add(&mut self, duration_ms: u32) -> &mut Self {
        self.renderer.p_duration_add = duration_ms;

        self
    }

    /// Changes the value of the `duration_delete` parameter.
    /// This parameter affects the animation duration when node or link is deleted.
    /// Default value is 1000 ms
    pub fn param_duration_delete(&mut self, duration_ms: u32) -> &mut Self {
        self.renderer.p_duration_delete = duration_ms;

        self
    }

    /// Changes the value of the `duration_move` parameter.
    /// This parameter affects the animation duration when node is moved.
    /// Default value is 1000 ms
    pub fn param_duration_move(&mut self, duration_ms: u32) -> &mut Self {
        self.renderer.p_duration_move = duration_ms;

        self
    }

    /// Changes the value of the `duration_select` parameter.
    /// This parameter affects the animation duration when node or link is selected or deselected.
    /// Default value is 1000 ms
    pub fn param_duration_select(&mut self, duration_ms: u32) -> &mut Self {
        self.renderer.p_duration_select = duration_ms;

        self
    }

    /// Changes the value of the `duration_color` parameter.
    /// This parameter affects the animation duration when node is colored.
    /// Default value is 1000 ms
    pub fn param_duration_color(&mut self, duration_ms: u32) -> &mut Self {
        self.renderer.p_duration_color = duration_ms;

        self
    }

    /// Changes the value of the `color_tag_created` parameter.
    /// This parameter affects the node or link stroke color when just created.
    /// Default rgb  value is (0, 0, 255)
    pub fn param_color_tag_created(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.renderer.p_color_tag_created = Color::new(red, green, blue);

        self
    }

    /// Changes the value of the `color_tag_selected` parameter.
    /// This parameter affects the node or link stroke color when selected.
    /// Default rgb  value is (191, 255, 0)
    pub fn param_color_tag_selected(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.renderer.p_color_tag_selected = Color::new(red, green, blue);

        self
    }

    /// Changes the value of the `color_tag_deleted` parameter.
    /// This parameter affects the node or link color when deleted.
    /// Default rgb  value is (255, 0, 0)
    pub fn param_color_tag_deleted(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.renderer.p_color_tag_deleted = Color::new(red, green, blue);

        self
    }

    /// Changes the value of the `color_node_fill` parameter.
    /// This parameter affects the node fill color.
    /// Default rgb  value is (255, 255, 255)
    pub fn param_color_node_fill(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.renderer.p_color_node_fill = Color::new(red, green, blue);

        self
    }

    /// Changes the value of the `color_node_stroke` parameter.
    /// This parameter affects the node stroke color.
    /// Default rgb  value is (128, 139, 150)
    pub fn param_color_node_stroke(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.renderer.p_color_node_stroke = Color::new(red, green, blue);

        self
    }

    /// Changes the value of the `color_link_stroke` parameter.
    /// This parameter affects the link stroke color.
    /// Default rgb  value is (128, 139, 150)
    pub fn param_color_link_stroke(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.renderer.p_color_link_stroke = Color::new(red, green, blue);

        self
    }

    /// Changes the value of the `color_text` parameter.
    /// This parameter affects the font color.
    /// Default rgb  value is (0, 0, 0)
    pub fn param_color_text(&mut self, red: u8, green: u8, blue: u8) -> &mut Self {
        self.renderer.p_color_text = Color::new(red, green, blue);

        self
    }

    /// Changes the value of the `display_node_label` parameter.
    /// This parameter indicates whether the node label is displayed or not.
    /// Default value is True
    pub fn param_display_node_label(&mut self, display: bool) -> &mut Self {
        self.renderer.p_display_node_label(display);

        self
    }

    /// Changes the value of the `display_link_value` parameter.
    /// This parameter indicates whether the link value is displayed or not.
    /// Default value is True
    pub fn param_display_link_value(&mut self, display: bool) -> &mut Self {
        self.renderer.p_display_link_value(display);

        self
    }

    /// Changes the value of the `stroke_width_node` parameter.
    /// This parameter affects the node stroke width.
    /// Default value is 2
    pub fn param_stroke_width_node(&mut self, width: u32) -> &mut Self {
        self.renderer.p_stroke_width_node(width);

        self
    }

    /// Changes the value of the `stroke_width_link` parameter.
    /// This parameter affects the link stroke width.
    /// Default value is 2
    pub fn param_stroke_width_link(&mut self, width: u32) -> &mut Self {
        self.renderer.p_stroke_width_link(width);

        self
    }

    /// Changes the value of the `radius_node` parameter.
    /// This parameter affects the node radius.
    /// Default value is 20
    pub fn param_radius_node(&mut self, radius: u32) -> &mut Self {
        self.renderer.p_radius_node(radius);

        self
    }
}
