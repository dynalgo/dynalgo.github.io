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

struct GraphError {
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
    adjacency: BTreeMap<char, BTreeMap<char, i8>>,
    anim_state: AnimState,
    layout_on_resume: bool,
    duration_on_resume: u32,
    p_speed_factor: f64,
    p_duration_add: u32,
    p_duration_delete: u32,
    p_duration_move: u32,
    p_duration_color: u32,
    p_radius: u8,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.graph_config())
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.graph_config())
    }
}

impl FromStr for Graph {
    type Err = String;
    fn from_str(s: &str) -> Result<Graph, String> {
        let mut graph = Graph::new();
        graph.from_str(s);
        Ok(graph)
    }
}

impl Graph {
    /// Creates an empty graph structure.
    pub fn new() -> Graph {
        let animation_speed_factor = 1.0;
        let node_radius = 13;
        let display_link_value = true;
        let display_node_name = true;
        let renderer = Renderer::new(
            display_link_value,
            display_node_name,
            Color::new(255, 255, 255),
            Color::new(47, 79, 79),
            Color::new(47, 79, 79),
            Color::new(0, 0, 139),
            Color::new(0, 0, 0),
            node_radius,
        );

        let graph = Graph {
            renderer,
            adjacency: BTreeMap::new(),
            anim_state: AnimState::Resumed,
            layout_on_resume: false,
            duration_on_resume: 1,
            p_speed_factor: animation_speed_factor,
            p_duration_add: 300,
            p_duration_delete: 300,
            p_duration_move: 900,
            p_duration_color: 600,
            p_radius: node_radius,
        };

        graph
    }

    /// Appends graph structure elements from a graph configuration.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let config = "A, B, A - B 0";
    /// let mut graph = Graph::new();
    /// graph.from_str(config);
    ///
    /// assert!(graph.nodes() == vec!['A', 'B']);
    /// ```
    pub fn from_str(&mut self, graph_config: &str) {
        let anim_state_init = self.anim_state;
        if anim_state_init == AnimState::Resumed {
            self.pause();
        }

        for line in graph_config.lines() {
            for cmd in line.split(',') {
                let fields: Vec<&str> = cmd.trim().split_whitespace().collect();
                if fields.is_empty() {
                    continue;
                }
                match fields.as_slice() {
                    [node_1, "-", node_2, value] => {
                        self.link_add_from(node_1, node_2, true, value);
                    }
                    [node_1, "-", node_2] => {
                        self.link_add_from(node_1, node_2, true, "_");
                    }
                    [node_from, ">", node_to, value] => {
                        self.link_add_from(node_from, node_to, false, value);
                    }
                    [node_from, ">", node_to] => {
                        self.link_add_from(node_from, node_to, false, "_");
                    }
                    [node_to, "<", node_from, value] => {
                        self.link_add_from(node_from, node_to, false, value);
                    }
                    [node_to, "<", node_from] => {
                        self.link_add_from(node_from, node_to, false, "_");
                    }
                    [node, cx, cy] => {
                        self.node_add_from(node, cx, cy);
                    }
                    [node] => {
                        self.node_add_from(node, "_", "_");
                    }
                    _ => panic!(
                        "{}",
                        GraphError {
                            action: String::from("parse config"),
                            message: format!("line '{} / {}' is invalid", line, cmd),
                        }
                    ),
                }
            }
        }

        if anim_state_init == AnimState::Resumed {
            self.resume();
        }
    }

    fn node_add_from(&mut self, name: &str, cx: &str, cy: &str) {
        if name.chars().count() != 1 {
            panic!(
                "{}",
                GraphError {
                    action: String::from("add a node"),
                    message: format!("'{}' is an invalid node name (char type requiered)", name),
                }
            )
        }
        let name = name.chars().next().unwrap();

        if cx != "_" {
            let cx = match cx.parse::<i16>() {
                Ok(v) => v,
                Err(_) => panic!(
                    "{}",
                    GraphError {
                        action: String::from("add a node"),
                        message: format!("'{}' is an invalid x coordinate for node {}", cx, name),
                    }
                ),
            };

            let cy = match cy.parse::<i16>() {
                Ok(v) => v,
                Err(_) => panic!(
                    "{}",
                    GraphError {
                        action: String::from("add a node"),
                        message: format!("'{}' is an invalid y coordinate for node {}", cy, name),
                    }
                ),
            };

            self.add_node(name, Some((cx, cy)));
        } else {
            self.add_node(name, None);
        }
    }

    fn link_add_from(&mut self, node_from: &str, node_to: &str, bidirect: bool, value: &str) {
        if node_from.chars().count() != 1 {
            panic!(
                "{}",
                GraphError {
                    action: String::from("add a link"),
                    message: format!(
                        "'{}' is an invalid node name (char type requiered)",
                        node_from
                    ),
                }
            )
        }
        if node_to.chars().count() != 1 {
            panic!(
                "{}",
                GraphError {
                    action: String::from("add a link"),
                    message: format!(
                        "'{}' is an invalid node name (char type requiered)",
                        node_to
                    ),
                }
            )
        }

        let node_from = node_from.chars().next().unwrap();
        let node_to = node_to.chars().next().unwrap();
        let value = if value == "_" {
            0
        } else {
            match value.parse::<i8>() {
                Ok(v) => v,
                Err(_) => panic!(
                    "{}",
                    GraphError {
                        action: String::from("add a link"),
                        message: format!(
                            "'{}' is an invalid value for link {}{}",
                            value, node_from, node_to
                        ),
                    }
                ),
            }
        };

        self.add_link(node_from, node_to, bidirect, value);
    }

    /// Adds a node to the graph structure, with an optional (x,y) freezed position (freezed coords of the element representation in SVG context). So the (x,y) position won't change when automatic layout algo runs.
    pub fn add_node(&mut self, name: char, xy: Option<(i16, i16)>) {
        self.node_check_not_exist(name);

        self.bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            Some(vec![(name, xy)]),
            None,
            None,
            None,
            self.p_duration_add,
        );

        self.need_layout();
    }

    /// Deletes a node from the graph structure.
    pub fn delete_node(&mut self, node: char) {
        self.node_check_exists(node);

        let links = self.node_links(node);

        let links_to_delete: Vec<(char, char)> = links
            .iter()
            .map(|(node_from, node_to, _, _)| (*node_from, *node_to))
            .collect();

        self.bulk_changes(
            None,
            None,
            None,
            None,
            Some(links_to_delete),
            Some(vec![node]),
            None,
            None,
            None,
            None,
            self.p_duration_delete,
        );

        self.need_layout();
    }

    /// Adds a link between two nodes. The link can be defined as bidirectional or not.
    pub fn add_link(&mut self, node_from: char, node_to: char, bidirectional: bool, value: i8) {
        self.link_check_not_exist(node_from, node_to);

        self.bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(vec![(node_from, node_to, bidirectional, value)]),
            None,
            None,
            self.p_duration_add,
        );

        self.need_layout();
    }

    /// Deletes a link.
    pub fn delete_link(&mut self, node_from: char, node_to: char) {
        self.link_check_exists(node_from, node_to);

        self.bulk_changes(
            None,
            None,
            None,
            None,
            Some(vec![(node_from, node_to)]),
            None,
            None,
            None,
            None,
            None,
            self.p_duration_delete,
        );

        self.need_layout();
    }

    /// Returns the links names list.
    pub fn links(&self) -> Vec<(char, char)> {
        let mut links = Vec::new();
        let directed = self.directed();
        for (node_from, neihgbors) in &self.adjacency {
            for (node_to, _) in neihgbors {
                let link = (*node_from, *node_to);
                if directed || !links.contains(&(*node_to, *node_from)) {
                    links.push(link);
                }
            }
        }

        links
    }

    /// Returns the nodes names list.
    pub fn nodes(&self) -> Vec<char> {
        self.adjacency.keys().cloned().collect()
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
    /// graph.from_str(config);
    ///
    /// for (node_from, neighbors) in graph.adjacency_list() {
    ///     for (node_to, link_value) in neighbors {
    ///         println!("Can go from {} to {} (link value is {}).", node_from, node_to, link_value);
    ///     }
    /// }
    /// ```
    pub fn adjacency_list(&self) -> BTreeMap<char, BTreeMap<char, i8>> {
        self.adjacency.clone()
    }

    /// Returns True if the graph is directed
    pub fn directed(&self) -> bool {
        for (node_from, neighbors) in &self.adjacency {
            for (node_to, _) in neighbors {
                if self
                    .adjacency
                    .get(node_to)
                    .unwrap()
                    .get(node_from)
                    .is_none()
                {
                    return true;
                }
            }
        }
        false
    }

    /// Returns the adjacency matrix.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let config = "A
    ///               B
    ///               A - B 0";
    /// let mut graph = Graph::new();
    /// graph.from_str(config);
    ///
    /// let adjacency_matrix = graph.adjacency_matrix();
    /// for node_from in graph.nodes().iter() {
    ///     for node_to in graph.nodes().iter() {
    ///         let link_value = adjacency_matrix[&node_from][&node_to];
    ///	        if link_value.is_some() {
    ///             println!("Can go from {} to {} (link value is {}).", node_from, node_to, link_value.unwrap());
    ///         }
    ///     }
    /// }
    /// ```
    pub fn adjacency_matrix(&self) -> BTreeMap<char, BTreeMap<char, Option<i8>>> {
        let mut matrix = BTreeMap::new();

        for node in self.nodes().into_iter() {
            for other in self.nodes().into_iter() {
                matrix
                    .entry(node)
                    .or_insert(BTreeMap::new())
                    .entry(other)
                    .or_insert(self.adjacency.get(&node).unwrap().get(&other).cloned());
            }
        }

        matrix
    }

    /// Returns accessible nodes.
    ///
    /// # Example
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let config = "A
    ///               B
    ///               A - B 0";
    /// let mut graph = Graph::new();
    /// graph.from_str(config);
    ///
    /// for neighbor in graph.neighbors('A') {
    ///     println!("Can go from 'A' to {}.", neighbor);
    /// }
    /// ```
    pub fn neighbors(&self, node: char) -> Vec<char> {
        self.node_check_exists(node);

        let neighbors = self
            .adjacency
            .get(&node)
            .unwrap()
            .iter()
            .map(|(k, _)| (*k))
            .collect();

        neighbors
    }

    /// Swap two nodes in the graph structure and its graphic representation.
    pub fn swap_nodes(&mut self, node_1: char, node_2: char) {
        self.node_check_exists(node_1);
        self.node_check_exists(node_2);

        let (x1, y1, freezed_1) = self.node_position(node_1);
        let (x2, y2, freezed_2) = self.node_position(node_2);

        let mut links = self.node_links(node_1);
        let links_2 = self.node_links(node_2);
        links.extend(&links_2);

        let mut links_to_delete = Vec::new();
        for (node_from, node_to, bidirect, _) in &links {
            if *bidirect && links_to_delete.contains(&(*node_to, *node_from))
                || links_to_delete.contains(&(*node_from, *node_to))
            {
                continue;
            }
            links_to_delete.push((*node_from, *node_to));
        }

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
            if bidirect && links_to_add.contains(&(node_to, node_from, bidirect, value))
                || links_to_add.contains(&(node_from, node_to, bidirect, value))
            {
                continue;
            }
            links_to_add.push((node_from, node_to, bidirect, value));
        }

        self.bulk_changes(
            None,
            None,
            None,
            None,
            Some(links_to_delete),
            None,
            None,
            None,
            None,
            None,
            self.p_duration_delete,
        );

        self.bulk_changes(
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
            None,
            None,
            self.p_duration_move,
        );

        self.bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(links_to_add),
            None,
            None,
            self.p_duration_add,
        );

        self.need_layout();
    }

    /// Returns a formatted string describing the graph structure.
    fn graph_config(&self) -> String {
        let mut config = String::new();

        for (node_from, _) in &self.adjacency {
            let (x, y, freezed) = self.node_position(*node_from);
            let line = match freezed {
                true => format!("{} {} {}\n", node_from, x, y),
                false => format!("{}\n", node_from),
            };
            config.push_str(&line);
        }

        for (node_from, neighbors) in &self.adjacency {
            for (node_to, value) in neighbors {
                let bidirect = match self.adjacency[&node_to].get(&node_from) {
                    Some(_) => "-",
                    None => ">",
                };
                if bidirect == "-" && node_from > node_to {
                    continue;
                }
                let line = format!("{} {} {} {}\n", node_from, bidirect, node_to, value);
                config.push_str(&line);
            }
        }

        config
    }

    fn node_links(&self, node: char) -> Vec<(char, char, bool, i8)> {
        let mut links = Vec::new();

        for (neighbor, value) in self.adjacency.get(&node).unwrap().iter() {
            let bidirect = self.adjacency.get(&neighbor).unwrap().get(&node).is_some();
            links.push((node, *neighbor, bidirect, *value));
        }
        for (node_other, neighbors_other) in self.adjacency.iter() {
            if *node_other == node {
                continue;
            };
            if let Some(value) = neighbors_other.get(&node) {
                if self
                    .adjacency
                    .get(&node)
                    .unwrap()
                    .get(&node_other)
                    .is_none()
                {
                    links.push((*node_other, node, false, *value));
                }
            }
        }

        links
    }

    fn node_check_exists(&self, node: char) {
        if self.adjacency.get(&node).is_none() {
            panic!(
                "{}",
                GraphError {
                    action: String::from("use a node"),
                    message: format!("node '{}' does not exist", node),
                }
            );
        };
    }

    fn node_check_not_exist(&self, node: char) {
        if self.adjacency.get(&node).is_some() {
            panic!(
                "{}",
                GraphError {
                    action: String::from("add a node"),
                    message: format!("node {} already exists", node),
                }
            );
        };
    }

    fn link_check_exists(&self, node_from: char, node_to: char) {
        self.node_check_exists(node_from);
        self.node_check_exists(node_to);
        if self
            .adjacency
            .get(&node_from)
            .unwrap()
            .get(&node_to)
            .is_none()
        {
            panic!(
                "{}",
                GraphError {
                    action: String::from("use a link"),
                    message: format!("link '{}{}' does not exist", node_from, node_to),
                }
            );
        };
    }

    fn link_check_not_exist(&self, node_from: char, node_to: char) {
        self.node_check_exists(node_from);
        self.node_check_exists(node_to);

        if node_from == node_to {
            panic!(
                "{}",
                GraphError {
                    action: String::from("add a link"),
                    message: format!(
                        "link {}{} is invalid (loop is not allowed)",
                        node_from, node_to
                    ),
                }
            );
        }

        if self
            .adjacency
            .get(&node_from)
            .unwrap()
            .get(&node_to)
            .is_some()
        {
            panic!(
                "{}",
                GraphError {
                    action: String::from("add a link"),
                    message: format!("link {}{} already exists", node_from, node_to),
                }
            );
        };

        if self
            .adjacency
            .get(&node_to)
            .unwrap()
            .get(&node_from)
            .is_some()
        {
            panic!(
                "{}",
                GraphError {
                    action: String::from("add a link"),
                    message: format!("directed link {}{} already exists", node_to, node_from),
                }
            );
        };
    }

    /// Reactivates the `auto animation` option.
    /// When this option is activated, each graph structure change causes graphic animation (the animations are rendered one after the other). By default, the option is activated when a graph is created.
    /// When this option is deactivated, all pending animations occur during the same period (i.e. not one after the other) when you manually call the `anim_step()` or `anim_resume()`functions.
    pub fn resume(&mut self) {
        match self.anim_state {
            AnimState::Paused => {
                self.step_speed(self.duration_on_resume, self.p_speed_factor);
                self.anim_state = AnimState::Resumed;
            }
            _ => panic!(
                "{}",
                GraphError {
                    action: String::from("resume animation"),
                    message: "animation has not been paused previoulsy".to_string(),
                }
            ),
        }
    }

    /// Deactivates the `auto animation` option.
    pub fn pause(&mut self) {
        match self.anim_state {
            AnimState::Resumed => {
                self.anim_state = AnimState::Paused;
            }
            _ => panic!(
                "{}",
                GraphError {
                    action: String::from("pause animation"),
                    message: "animation has not been resumed previoulsy".to_string(),
                }
            ),
        }
    }

    /// Creates an animation that show the evolution between the last represented state (when `anim_pause()` function was called previously) and the current state. The pending animations are rendered simultaneously (i.e. not one after the other).
    /// The `anim_pause()` function must have been called previously.
    /// After calling `anim_step()` function,  `auto animation` option still is deactivated.
    pub fn step(&mut self, duration_ms: u32) {
        self.step_speed(duration_ms, 1.);
    }

    fn step_speed(&mut self, duration_ms: u32, speed_factor: f64) {
        match self.anim_state {
            AnimState::Paused => {}
            _ => panic!(
                "{}",
                GraphError {
                    action: String::from("step animation"),
                    message: "animation has not been paused previously".to_string(),
                }
            ),
        }
        if self.layout_on_resume {
            self.layout();
            self.layout_on_resume = false;
        }
        self.animate(duration_ms, speed_factor);
        self.duration_on_resume = 1;
    }

    /// Delay the next animation.
    pub fn sleep(&mut self, duration_ms: u32) {
        self.renderer.sleep(duration_ms);
    }

    /// Indicates whether the animation is paused or not
    pub fn paused(&self) -> bool {
        self.anim_state == AnimState::Paused
    }

    /// Renders the graph animation in SVG SMIL format into a HTML file.
    pub fn render(&self, html_file_name: &str) -> Result<(), std::io::Error> {
        Self::to_html(vec![(html_file_name, vec![self])])
    }

    /// Renders graphs animations in SVG SMIL format into multiple HTML files.
    /// Each HTML page contains a menu to access other pages (if there is more than one page).
    pub fn to_html(pages: Vec<(&str, Vec<&Graph>)>) -> Result<(), std::io::Error> {
        let titles: Vec<String> = pages.iter().map(|(title, _)| title.to_string()).collect();
        let mut file_names: Vec<String> = Vec::new();
        for title in &titles {
            file_names.push(title.replace(" ", "_"));
        }
        for (i, (_title, graphs)) in pages.iter().enumerate() {
            let mut svgs = Vec::new();
            for graph in graphs {
                svgs.push(graph.animation());
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
    pub fn duration(&self) -> u32 {
        self.renderer.duration()
    }

    /// Returns the current x,y coords (and freezed tag) of the node in the SVG graphic context.
    pub fn node_position(&self, node: char) -> (i32, i32, bool) {
        self.node_check_exists(node);

        let point = self.renderer.node_center(node);
        let freezed = self.renderer.node_center_freezed(node);
        (point.x(), point.y(), freezed)
    }

    /// Changes and freezes the x,y coords of the SVG node representation. The node position will not change when automatic layout algo runs.
    pub fn move_node(&mut self, node: char, xy: (i32, i32)) {
        self.node_check_exists(node);

        self.bulk_changes(
            None,
            None,
            Some(vec![(node, xy, true)]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            self.p_duration_move,
        )
    }

    /// Changes the node fill color.
    pub fn fill_node(&mut self, node: char, color: (u8, u8, u8)) {
        self.node_check_exists(node);

        self.bulk_changes(
            None,
            None,
            None,
            Some((vec![node], color)),
            None,
            None,
            None,
            None,
            None,
            None,
            self.p_duration_color,
        )
    }

    /// Changes the node stroke color.
    pub fn color_node(&mut self, node: char, color: (u8, u8, u8)) {
        self.node_check_exists(node);

        self.bulk_changes(
            Some((vec![node], color)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            self.p_duration_color,
        )
    }

    /// Changes the node label color.
    pub fn color_label(&mut self, node: char, color: (u8, u8, u8)) {
        self.node_check_exists(node);

        self.bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some((vec![node], color)),
            None,
            self.p_duration_color,
        )
    }

    /// Unfreezes the node position (coords in SVG graphic context), so the current position will change  when automatic layout algo runs.
    pub fn unfreeze_node(&mut self, node: char) {
        self.node_check_exists(node);
        self.renderer.node_freezed(node, false);
        self.need_layout();
    }

    /// Changes the link stroke color.
    pub fn color_link(&mut self, node_from: char, node_to: char, color: (u8, u8, u8)) {
        self.link_check_exists(node_from, node_to);

        self.bulk_changes(
            None,
            Some((vec![(node_from, node_to)], color)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            self.p_duration_color,
        )
    }

    /// Changes the link value color.
    pub fn color_value(&mut self, node_from: char, node_to: char, color: (u8, u8, u8)) {
        self.link_check_exists(node_from, node_to);

        self.bulk_changes(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some((vec![(node_from, node_to)], color)),
            self.p_duration_color,
        )
    }

    /// Changes the animation speed (from 0.1 to 10.0).
    /// Default value is 1.0
    pub fn speed(&mut self, speed_factor: f64) {
        let speed_factor = if speed_factor < 0.1 {
            0.1
        } else if speed_factor > 10. {
            10.
        } else {
            speed_factor
        };
        self.p_speed_factor = speed_factor;
    }

    fn animate(&mut self, duration_ms: u32, speed_factor: f64) {
        let duration_ms = (duration_ms as f64 / speed_factor) as u32;
        let duration_ms = match duration_ms {
            0 => 1,
            d => d,
        };
        self.renderer.animate(duration_ms);
    }

    fn need_layout(&mut self) {
        if self.anim_state == AnimState::Resumed {
            self.layout();
        } else {
            self.layout_on_resume = true;
        }
    }

    fn layout(&mut self) {
        let mut adjacency = self.adjacency_list();
        for (node_from, neighbors) in &self.adjacency {
            for (node_to, value) in neighbors {
                if self
                    .adjacency
                    .get(node_to)
                    .unwrap()
                    .get(node_from)
                    .is_none()
                {
                    adjacency
                        .get_mut(node_to)
                        .unwrap()
                        .insert(*node_from, *value);
                }
            }
        }

        self.renderer.layout(adjacency);

        if !self.paused() {
            self.animate(self.p_duration_move, self.p_speed_factor);
        }
    }

    fn animation(&self) -> String {
        self.renderer.animation()
    }

    fn bulk_changes(
        &mut self,
        nodes_colered: Option<(Vec<char>, (u8, u8, u8))>,
        links_colered: Option<(Vec<(char, char)>, (u8, u8, u8))>,
        nodes_move: Option<Vec<(char, (i32, i32), bool)>>,
        nodes_filled: Option<(Vec<char>, (u8, u8, u8))>,
        links_deleted: Option<Vec<(char, char)>>,
        nodes_deleted: Option<Vec<char>>,
        nodes_added: Option<Vec<(char, Option<(i16, i16)>)>>,
        links_added: Option<Vec<(char, char, bool, i8)>>,
        nodes_text_colered: Option<(Vec<char>, (u8, u8, u8))>,
        links_text_colered: Option<(Vec<(char, char)>, (u8, u8, u8))>,
        duration_ms: u32,
    ) {
        let anim_state_init = self.anim_state;
        if anim_state_init == AnimState::Resumed {
            self.pause();
        }

        match nodes_colered {
            Some((nodes, color)) => {
                for node in nodes {
                    self.renderer.node_stroke_color(node, color);
                }
            }
            None => {}
        }
        match links_colered {
            Some((links, color)) => {
                for (node_from, node_to) in links {
                    self.renderer.link_stroke_color(node_from, node_to, color);
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
        match nodes_filled {
            Some((nodes, color)) => {
                for node in nodes {
                    self.renderer.node_fill_color(node, color);
                }
            }
            None => {}
        }
        match links_deleted {
            Some(links) => {
                for (node_from, node_to) in links {
                    self.adjacency.get_mut(&node_from).unwrap().remove(&node_to);
                    self.adjacency.get_mut(&node_to).unwrap().remove(&node_from);

                    self.renderer.delete_link(node_from, node_to);
                }
            }
            None => {}
        }
        match nodes_deleted {
            Some(nodes) => {
                for node in nodes {
                    self.adjacency.remove(&node);
                    self.renderer.delete_node(node);
                }
            }
            None => {}
        }
        match nodes_added {
            Some(nodes) => {
                for (node, xy) in nodes {
                    self.adjacency.insert(node, BTreeMap::new());

                    let point = match xy {
                        Some((x, y)) => Some(Point::new(x as i32, y as i32)),
                        None => None,
                    };
                    self.renderer.add_node(node, point);
                }
            }
            None => {}
        }
        match links_added {
            Some(links) => {
                for (node_from, node_to, bidirect, value) in links {
                    self.adjacency
                        .get_mut(&node_from)
                        .unwrap()
                        .insert(node_to, value);
                    if bidirect {
                        self.adjacency
                            .get_mut(&node_to)
                            .unwrap()
                            .insert(node_from, value);
                    }
                    self.renderer.add_link(node_from, node_to, bidirect, value);
                }
            }
            None => {}
        }

        match nodes_text_colered {
            Some((nodes, color)) => {
                for node in nodes {
                    self.renderer.node_text_color(node, color);
                }
            }
            None => {}
        }
        match links_text_colered {
            Some((links, color)) => {
                for (node_from, node_to) in links {
                    self.renderer.link_text_color(node_from, node_to, color);
                }
            }
            None => {}
        }

        if anim_state_init == AnimState::Resumed {
            self.step_speed(duration_ms, self.p_speed_factor);
            self.resume();
        } else {
            self.duration_on_resume = duration_ms;
        }
    }

    /// Returns the radius of nodes
    pub fn node_radius(&self) -> u8 {
        self.p_radius
    }

    /// Returns the graph sequence (outdegree increasing order).
    pub fn sequence(&self) -> Vec<(char, (usize, usize))> {
        let mut degrees = BTreeMap::new();

        for (node_from, neighbors) in &self.adjacency {
            degrees.insert(*node_from, (neighbors.len(), 0));
        }

        for (_, neighbors) in &self.adjacency {
            for node_to in neighbors.keys() {
                let (_, indegree) = degrees.get_mut(node_to).unwrap();
                *indegree += 1;
            }
        }

        let mut sequence: Vec<(char, (usize, usize))> = degrees.into_iter().collect();
        sequence.sort_by(|(_, (a, _)), (_, (b, _))| b.cmp(a));
        sequence
    }
}
