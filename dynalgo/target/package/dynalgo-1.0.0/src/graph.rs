mod link;
mod node;
mod renderer;

use link::Link;
use node::Node;
use renderer::color::Color;
use renderer::html::Html;
use renderer::point::Point;
use renderer::Renderer;
use std::collections::HashMap;

pub struct Graph {
    nodes: HashMap<char, Node>,
    links: HashMap<char, Link>,
    renderer: Renderer,
    svg_automatic_animation: bool,
    svg_automatic_layout: bool,
}

impl Graph {
    /// Creates an empty graph structure
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// let graph = Graph::new();
    /// ```
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            links: HashMap::new(),
            renderer: Renderer::new(),
            svg_automatic_animation: true,
            svg_automatic_layout: true,
        }
    }

    fn node_add_from(&mut self, name: &str, cx: &str, cy: &str, value: &str) -> Result<(), String> {
        let name = name.chars().next().unwrap();

        let value = if value == "_" {
            None
        } else {
            match value.parse::<u8>() {
                Ok(v) => Some(v),
                Err(_) => Err(format!("Node {} - Invalid indicator value {}", name, value))?,
            }
        };

        if cx != "_" {
            let cx = match cx.parse::<i16>() {
                Ok(v) => v,
                Err(_) => Err(format!("Node {} - Invalid cx value {}", name, cx))?,
            };

            let cy = match cy.parse::<i16>() {
                Ok(v) => v,
                Err(_) => Err(format!("Node {} - Invalid cy value {}", name, cy))?,
            };

            self.node_add_xy(name, Some((cx, cy)), value)?;
        } else {
            self.node_add(name, value)?;
        }

        Ok(())
    }

    /// Adds a node to the graph structure.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, None);
    /// graph.node_add('C', None);
    /// graph.link_add('β', 'C', 'B', true, None);
    /// graph.node_add('D', None);
    /// graph.link_add('γ', 'D', 'A', true, None);
    /// graph.link_add('δ', 'B', 'C', true, None);
    ///
    /// let html = graph.svg_render_animation_html("node_add example");
    /// write!(File::create("example-node_add.html").unwrap(), "{}", html);
    /// ```
    pub fn node_add(&mut self, name: char, value: Option<u8>) -> Result<(), String> {
        self.node_add_xy(name, None, value)
    }

    /// Adds a node to the graph structure, with a fixed (x,y) position (fixed coords of the element representation in SVG context).
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add_fixed('A', 400, 400, None);
    /// graph.node_add_fixed('B', 400, 100, None);
    /// graph.node_add_fixed('C', 100, 400, None);
    /// graph.node_add_fixed('D', 100, 100, None);
    /// graph.node_add('E', None);
    /// graph.link_add('α', 'E', 'A', true, None);
    /// graph.link_add('β', 'E', 'B', true, None);
    /// graph.link_add('γ', 'E', 'C', true, None);
    /// graph.link_add('δ', 'E', 'D', true, None);
    ///
    /// let html = graph.svg_render_animation_html("node_add_fixed example");
    /// write!(File::create("example-node_add_fixed.html").unwrap(), "{}", html);
    /// ```
    pub fn node_add_fixed(
        &mut self,
        name: char,
        x: i16,
        y: i16,
        value: Option<u8>,
    ) -> Result<(), String> {
        self.node_add_xy(name, Some((x, y)), value)
    }

    fn node_add_xy(
        &mut self,
        name: char,
        xy: Option<(i16, i16)>,
        value: Option<u8>,
    ) -> Result<(), String> {
        self.node_check_not_exist(name)?;

        let node = Node::new(name, value);

        self.nodes.insert(name, node);
        let point = match xy {
            Some((x, y)) => Some(Point::new(x as i32, y as i32)),
            None => None,
        };
        self.renderer.node_add(name, point, value);

        if self.svg_automatic_animation {
            self.svg_animate(self.renderer.p_duration_add);
            if self.svg_automatic_layout {
                self.svg_layout_nodes(vec![name])?;
            }
        }

        Ok(())
    }

    /// Deletes a node from the graph structure.
    /// The associated links are also deleted.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, None);
    /// graph.node_delete('B');
    ///
    /// let html = graph.svg_render_animation_html("node_delete example");
    /// write!(File::create("example-node_delete.html").unwrap(), "{}", html);
    /// ```
    pub fn node_delete(&mut self, name: char) -> Result<(), String> {
        self.node_check_exists(name)?;
        for link in self.node_links(name)? {
            self.link_delete(link.name())?;
        }
        self.nodes.remove(&name);
        self.renderer.node_delete(name);

        if self.svg_automatic_animation {
            self.svg_animate(self.renderer.p_duration_delete);
        }

        Ok(())
    }

    fn link_add_from(
        &mut self,
        name: &str,
        from_node: &str,
        to_node: &str,
        bidirectional: &str,
        value: &str,
    ) -> Result<(), String> {
        let name = name.chars().next().unwrap();
        let from = from_node.chars().next().unwrap();
        let to = to_node.chars().next().unwrap();

        let bidirect = match bidirectional.parse::<bool>() {
            Ok(v) => v,
            Err(_) => Err(format!(
                "Link {} - Invalid bidirect value {}",
                name, bidirectional
            ))?,
        };

        let value = if value == "_" {
            None
        } else {
            match value.parse::<u8>() {
                Ok(v) => Some(v),
                Err(_) => Err(format!("Link {} - Invalid indicator value {}", name, value))?,
            }
        };

        self.link_add(name, from, to, bidirect, value)?;

        Ok(())
    }

    /// Adds a link between two nodes.
    /// The link can be defined as bidirectional or not.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    ///
    /// let html = graph.svg_render_animation_html("link_add example");
    /// write!(File::create("example-link_add.html").unwrap(), "{}", html);
    /// ```
    pub fn link_add(
        &mut self,
        name: char,
        from_node: char,
        to_node: char,
        bidirectional: bool,
        value: Option<u8>,
    ) -> Result<(), String> {
        self.link_check_not_exist(name, from_node, to_node)?;
        self.node_check_exists(from_node)?;
        self.node_check_exists(to_node)?;

        let link = Link::new(name, from_node, to_node, bidirectional, value);

        self.links.insert(name, link);
        self.renderer
            .link_add(name, from_node, to_node, bidirectional, value);

        if self.svg_automatic_animation {
            self.svg_animate(self.renderer.p_duration_add);
            if self.svg_automatic_layout {
                self.svg_layout_nodes(vec![from_node, to_node])?;
            }
        }

        Ok(())
    }

    /// Deletes a link.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.link_delete('α');
    ///
    /// let html = graph.svg_render_animation_html("link_delete example");
    /// write!(File::create("example-link_delete.html").unwrap(), "{}", html);
    /// ```
    pub fn link_delete(&mut self, name: char) -> Result<(), String> {
        self.link_check_exists(name)?;

        //let link = self.links.get(&name).unwrap();
        //let (from_node, to_node) = (link.from(), link.to());

        self.links.remove(&name);
        self.renderer.link_delete(name);

        if self.svg_automatic_animation {
            self.svg_animate(self.renderer.p_duration_delete);
            //if self.svg_automatic_layout {
            //    self.svg_layout(vec![from_node, to_node]);
            //}
        }

        Ok(())
    }

    /// Returns the optional value associated to the node.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', Some(10));
    /// let value : Option<u8> = graph.node_value('A').unwrap();
    /// ```
    pub fn node_value(&self, name: char) -> Result<Option<u8>, String> {
        self.node_check_exists(name)?;
        Ok(self.nodes.get(&name).unwrap().value())
    }

    /// Returns the list of nodes names.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// let nodes : Vec<char> = graph.nodes_list();
    /// ```
    pub fn nodes_list(&self) -> Vec<char> {
        self.nodes.keys().cloned().collect()
    }

    fn node_links(&self, name: char) -> Result<Vec<Link>, String> {
        self.node_check_exists(name)?;

        let mut links = Vec::new();
        for (_, link) in &self.links {
            if link.from() != name && link.to() != name {
                continue;
            }
            links.push(link.clone());
        }

        Ok(links)
    }

    /// Returns an adjacency list.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::collections::HashMap;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    ///
    /// let adjacency : HashMap<char, HashMap<char, (char, Option<u8>)>> = graph.adjacency_list();
    /// for (node_from, adjacent) in adjacency {
    ///     for (node_to, link) in adjacent {
    ///         println!("Can go from node {} to node {}", node_from, node_to);
    ///         match link {
    ///             (link_name, Some(value)) => println!("Link {} has value {}", link_name, value),
    ///             (link_name, None) => println!("Link is {}", link_name),
    ///         }
    ///     }
    /// }
    /// ```
    pub fn adjacency_list(&self) -> HashMap<char, HashMap<char, (char, Option<u8>)>> {
        let mut adjacency = HashMap::new();
        for (name, _) in &self.nodes {
            adjacency.insert(*name, HashMap::new());
        }

        for (_, link) in &self.links {
            adjacency
                .get_mut(&link.from())
                .unwrap()
                .insert(link.to(), (link.name(), link.value()));

            if link.bidirect() {
                adjacency
                    .get_mut(&link.to())
                    .unwrap()
                    .insert(link.from(), (link.name(), link.value()));
            }
        }

        adjacency
    }

    /// Exchanges two nodes in the graph structure.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.svg_param_display_link_label(true);
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.node_add('C', None);
    /// graph.link_add('α', 'A', 'B', true, None);
    /// graph.link_add('β', 'B', 'C', true, None);
    /// graph.link_add('γ', 'C', 'A', true, None);
    /// graph.nodes_exchange('A', 'B');
    ///
    /// let html = graph.svg_render_animation_html("nodes_exchange example");
    /// write!(File::create("example-nodes_exchange.html").unwrap(), "{}", html);
    /// ```
    pub fn nodes_exchange(&mut self, name_1: char, name_2: char) -> Result<(), String> {
        self.node_check_exists(name_1)?;
        self.node_check_exists(name_2)?;

        let (x1, y1) = self.svg_node_position(name_1)?;
        let (x2, y2) = self.svg_node_position(name_2)?;

        let links_1 = self.node_links(name_1)?;
        let links_2 = self.node_links(name_2)?;

        let automatic_layout = self.svg_automatic_layout;
        self.svg_automatic_layout(false);
        let automatic_animation = self.svg_automatic_animation;
        self.svg_automatic_animation(false);

        let mut already = Vec::new();
        for link in &links_1 {
            self.link_delete(link.name())?;
            already.push(link.name());
        }
        for link in &links_2 {
            if already.contains(&link.name()) {
                continue;
            }
            self.link_delete(link.name())?;
        }

        if automatic_animation {
            self.svg_animate(self.renderer.p_duration_delete);
        }
        self.svg_bulk_changes(
            None,
            None,
            Some(vec![(name_1, (x2, y2)), (name_2, (x1, y1))]),
            None,
            self.renderer.p_duration_move,
        )?;

        if automatic_animation {
            self.svg_animate(self.renderer.p_duration_move);
        }

        for link in &links_1 {
            let from = match link.from() {
                n if n == name_1 => name_2,
                n if n == name_2 => name_1,
                n => n,
            };
            let to = match link.to() {
                n if n == name_1 => name_2,
                n if n == name_2 => name_1,
                n => n,
            };
            self.link_add(link.name(), from, to, link.bidirect(), link.value())?;
        }
        for link in &links_2 {
            if already.contains(&link.name()) {
                continue;
            }
            let from = match link.from() {
                n if n == name_1 => name_2,
                n if n == name_2 => name_1,
                n => n,
            };
            let to = match link.to() {
                n if n == name_1 => name_2,
                n if n == name_2 => name_1,
                n => n,
            };
            self.link_add(link.name(), from, to, link.bidirect(), link.value())?;
        }

        if automatic_animation {
            self.svg_animate(self.renderer.p_duration_add);
        }
        self.svg_automatic_animation(automatic_animation);
        self.svg_automatic_layout(automatic_layout);

        Ok(())
    }

    /// Imports a graph structure from a formatted String
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// let dyna = String::from(
    ///        "N A _ _ _
    ///         N B _ _ _
    ///         N C _ _ _
    ///         L a A B true 1
    ///         L b A C true 2
    ///         L c B C true 3"
    /// );
    /// graph.dyna_from(dyna);
    ///
    /// let html = graph.svg_render_animation_html("dyna_from example");
    /// write!(File::create("example-dyna_from.html").unwrap(), "{}", html);
    /// ```
    pub fn dyna_from(&mut self, graph_config: String) -> Result<(), String> {
        let automatic_animation = self.svg_automatic_animation;
        self.svg_automatic_animation(false);

        for lign in graph_config.lines() {
            let fields: Vec<&str> = lign.trim().split_whitespace().collect();
            match fields.as_slice() {
                ["N", name, cx, cy, value] => {
                    self.node_add_from(name, cx, cy, value)?;
                }
                ["L", name, from, to, bidirect, value] => {
                    self.link_add_from(name, from, to, bidirect, value)?;
                }
                _ => Err(format!("Invalid graph config file - Line: {}", lign))?,
            }
        }

        self.svg_animate(self.renderer.p_duration_add);
        self.svg_automatic_animation(automatic_animation);

        if self.svg_automatic_animation && self.svg_automatic_layout {
            self.svg_layout();
        }

        Ok(())
    }

    /// Exports a graph structure to a formatted String
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', Some(2));
    /// graph.link_add('α', 'A', 'B', true, Some(1));
    /// let dyna = graph.dyna_to();   
    ///    
    /// let mut dyna_file = File::create("example-dyna_to.dyna").unwrap();
    /// write!(dyna_file, "{}", dyna);
    /// ```
    pub fn dyna_to(&self) -> String {
        let mut dyna = String::new();

        for (name, node) in self.nodes.iter() {
            let value = match node.value() {
                Some(v) => format!("{}", v),
                None => "_".to_string(),
            };
            let center = self.renderer.node_center(*name);
            let lign = format!(
                "N {} {} {} {}\n",
                node.name(),
                center.x(),
                center.y(),
                value
            );
            dyna.push_str(&lign);
        }

        for (_, link) in self.links.iter() {
            let value = match link.value() {
                Some(v) => format!("{}", v),
                None => "_".to_string(),
            };
            let lign = format!(
                "L {} {} {} {} {}\n",
                link.name(),
                link.from(),
                link.to(),
                link.bidirect(),
                value
            );
            dyna.push_str(&lign);
        }

        dyna
    }

    fn node_check_exists(&self, name: char) -> Result<(), String> {
        if self.nodes.get(&name).is_none() {
            Err(format!("Node {} does not exist", name))?;
        };

        Ok(())
    }

    fn node_check_not_exist(&self, name: char) -> Result<(), String> {
        if self.nodes.get(&name).is_some() {
            Err(format!("Node {} already exists", name))?;
        };

        Ok(())
    }

    fn link_check_exists(&self, name: char) -> Result<(), String> {
        if self.links.get(&name).is_none() {
            Err(format!("Link {} does not exist", name))?;
        };

        Ok(())
    }

    fn link_check_not_exist(
        &self,
        name: char,
        from_node: char,
        to_node: char,
    ) -> Result<(), String> {
        if self.links.get(&name).is_some() {
            Err(format!("Link {} already exists", name))?;
        }

        if from_node == to_node {
            Err(format!("Link {} is invalid (loop is not allowed)", name))?;
        }

        for (_, link) in &self.links {
            if link.from() == from_node && link.to() == to_node
                || link.from() == to_node && link.to() == from_node
            {
                Err(format!(
                    "A link {} similar to {} already exists (multiple edges are not allowed)",
                    link.name(),
                    name
                ))?;
            }
        }

        Ok(())
    }

    /// Activates or desactivates the `svg_automatic_animation` option (affects only SVG rendering).
    /// When this option is activated, each change in the structure of the graph, and each change in the property of the representation of the node or of the link causes a graphic animation (the animations occur one after the other).
    /// If this option is desactivated, then all the pending animations occur during the same period (i.e. not one after the other) when you manually call the `svg_animate` function.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.svg_automatic_animation(false);
    /// graph.node_add_fixed('C', 0, 60, None);
    /// graph.node_add_fixed('D', 0, -60, None);
    /// graph.link_add('β', 'A', 'C', true, Some(20));
    /// graph.link_add('γ', 'D', 'C', true, Some(30));
    /// graph.svg_animate(5000);
    /// graph.svg_automatic_animation(true);
    /// graph.svg_layout();
    ///
    /// let html = graph.svg_render_animation_html("svg_automatic_animation example");
    /// write!(File::create("example-svg_automatic_animation.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_automatic_animation(&mut self, automatic: bool) {
        self.svg_automatic_animation = automatic;
    }

    /// Creates SVG animations with SMIL language (affects only SVG rendering).
    /// Each change in the structure of the graph, and each change in the property of the representation of the node or of the link causes an animation.
    /// The animation show the evolution between the last represented state and the current state.
    /// The value of the duration_ms parameter can not be less than 1 ms.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add_fixed('A', 0, 0, None);
    /// graph.node_add_fixed('B', 100, 0, None);
    /// graph.node_add_fixed('C', 50, -50, None);
    /// graph.link_add('α', 'A', 'B', true, None);
    /// graph.svg_node_move('A', 50, 0);
    /// graph.svg_node_move('A', 50, 50);
    /// graph.svg_node_move('A', 0, 50);
    /// graph.svg_automatic_animation(false);
    /// graph.svg_node_move('B', 150, 0);
    /// graph.svg_node_move('B', 150, 50);
    /// graph.svg_node_move('B', 100, 50);
    /// graph.svg_animate(1000);
    /// graph.svg_automatic_animation(true);
    /// graph.nodes_exchange('A', 'B');
    /// graph.svg_automatic_animation(false);
    /// graph.svg_node_color('A', 255, 0, 0);
    /// graph.svg_node_color('B', 0, 255, 0);
    /// graph.nodes_exchange('A', 'B');
    /// graph.nodes_exchange('A', 'B');
    /// graph.svg_node_color('C', 0, 0, 255);
    /// graph.svg_animate(1000);
    ///
    /// let html = graph.svg_render_animation_html("svg_animate example");
    /// write!(File::create("example-svg_animate.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_animate(&mut self, duration_ms: u32) {
        let duration_ms = match duration_ms {
            0 => 1,
            d => d,
        };
        self.renderer.animate(duration_ms);
    }

    /// Activates or desactivates the `svg_automatic_layout` option (affects only SVG rendering).
    /// Activating this option is revelant only if the `svg_automatic_animation` option is activated.
    /// When this option is activated, the nodes are layouted as they are created. When this option is desactivated, you must explicitly call the svg_layout or svg_layout_nodes function to layout the created nodes.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.svg_automatic_layout(false);
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.svg_layout();
    ///
    /// let html = graph.svg_render_animation_html("svg_automatic_layout example");
    /// write!(File::create("example-svg_automatic_layout.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_automatic_layout(&mut self, automatic: bool) {
        self.svg_automatic_layout = automatic;
    }

    /// Layouts the listed nodes (affects only SVG rendering).
    /// Calling this function is revelant only if the option `svg_automatic_layout` is desactivated.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, None);
    /// graph.svg_automatic_layout(false);
    /// graph.node_add('C', None);
    /// graph.node_add('D', None);
    /// graph.link_add('β', 'A', 'C', true, None);
    /// graph.link_add('γ', 'B', 'D', true, None);
    /// graph.svg_layout_nodes(vec!['C', 'D']);
    ///
    /// let html = graph.svg_render_animation_html("svg_layout_nodes example");
    /// write!(File::create("example-svg_layout_nodes.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_layout_nodes(&mut self, nodes_names: Vec<char>) -> Result<(), String> {
        for name in &nodes_names {
            self.node_check_exists(*name)?;
        }

        let adjacencies = self.adjacency_list();
        self.renderer.layout(&nodes_names, adjacencies);
        self.svg_animate(self.renderer.p_duration_move);

        Ok(())
    }

    /// Layouts all the nodes (affects only SVG rendering).
    /// Calling this function is revelant only if the `svg_automatic_layout` option is desactivated.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.svg_automatic_layout(false);
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, None);
    /// graph.node_add('C', None);
    /// graph.node_add('D', None);
    /// graph.link_add('β', 'A', 'C', true, None);
    /// graph.link_add('γ', 'B', 'D', true, None);
    /// graph.svg_layout();
    ///
    /// let html = graph.svg_render_animation_html("svg_layout example");
    /// write!(File::create("example-svg_layout.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_layout(&mut self) {
        self.svg_layout_nodes(self.nodes_list()).unwrap()
    }

    fn svg_render_animation(&self) -> String {
        self.renderer.animation()
    }

    /// Exports the animations in SVG format into a HTML page.
    /// The HTML page contains a little javascript so that the animation can be paused and resumed by clickig on the image.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.node_add('C', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.link_add('β', 'B', 'C', true, Some(20));
    /// graph.link_add('γ', 'C', 'A', true, Some(30));
    ///
    /// let html_file_content = graph.svg_render_animation_html("svg_render_animation_html example");       
    /// let mut html_file = File::create("example-svg_render_animation_html.html").unwrap();
    /// write!(html_file, "{}", html_file_content);
    /// ```
    pub fn svg_render_animation_html(&self, title: &str) -> String {
        Html::render(title, self.svg_render_animation())
    }

    /// Returns the current x,y coords of the node in the SVG graphic context.
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.node_add('A', None);
    /// let (x,y) : (i32,i32) = graph.svg_node_position('A').unwrap();
    /// ```
    pub fn svg_node_position(&self, name: char) -> Result<(i32, i32), String> {
        self.node_check_exists(name)?;

        let point = self.renderer.node_center(name);
        Ok((point.x(), point.y()))
    }

    fn svg_bulk_changes(
        &mut self,
        nodes_selected: Option<(Vec<char>, bool)>, // ( nodes_names, selected )
        links_selected: Option<(Vec<char>, bool)>, // ( links_names, selected )
        nodes_move: Option<Vec<(char, (i32, i32))>>, // Vec<(node_name, (cx: i32, cy: i32))>
        nodes_color: Option<(Vec<char>, (u8, u8, u8))>, // ( nodes_names, (red, green, blue) )
        duration_ms: u32,
    ) -> Result<(), String> {
        match nodes_selected {
            Some((names, selected)) => {
                for name in names {
                    self.node_check_exists(name)?;
                    self.renderer.node_selected(name, selected);
                }
            }
            None => {}
        }
        match links_selected {
            Some((names, selected)) => {
                for name in names {
                    self.link_check_exists(name)?;
                    self.renderer.link_selected(name, selected);
                }
            }
            None => {}
        }
        match nodes_move {
            Some(moves) => {
                for (name, (cx, cy)) in moves {
                    self.node_check_exists(name)?;
                    self.renderer.node_move(name, Point::new(cx, cy));
                }
            }
            None => {}
        }
        match nodes_color {
            Some((names, (red, green, blue))) => {
                for name in names {
                    self.node_check_exists(name)?;
                    self.renderer.node_color(name, (red, green, blue));
                }
            }
            None => {}
        }

        if self.svg_automatic_animation {
            self.svg_animate(duration_ms);
        }

        Ok(())
    }

    /// Changes the selected status of a node (affects only SVG rendering).
    /// The selected status affects the stroke color of the node.
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.svg_automatic_animation(false);
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.node_add('C', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.link_add('β', 'B', 'C', true, Some(20));
    /// graph.link_add('γ', 'C', 'A', true, Some(30));
    /// graph.svg_automatic_animation(true);
    /// graph.svg_layout();
    /// graph.svg_node_selected('A', true);
    /// graph.svg_node_selected('B', true);
    /// graph.svg_node_selected('C', true);
    ///
    /// let html = graph.svg_render_animation_html("svg_node_selected example");
    /// write!(File::create("example-svg_node_selected.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_node_selected(&mut self, name: char, selected: bool) -> Result<(), String> {
        self.svg_bulk_changes(
            Some((vec![name], selected)),
            None,
            None,
            None,
            self.renderer.p_duration_select,
        )
    }

    /// Changes the selected status of a link (affects only SVG rendering).
    /// The selected status affects the stroke color of the link.
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.svg_automatic_animation(false);
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.node_add('C', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.link_add('β', 'B', 'C', true, Some(20));
    /// graph.link_add('γ', 'C', 'A', true, Some(30));
    /// graph.svg_automatic_animation(true);
    /// graph.svg_layout();
    /// graph.svg_link_selected('α', true);
    /// graph.svg_link_selected('β', true);
    /// graph.svg_link_selected('γ', true);
    ///
    /// let html = graph.svg_render_animation_html("svg_link_selected example");
    /// write!(File::create("example-svg_link_selected.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_link_selected(&mut self, name: char, selected: bool) -> Result<(), String> {
        self.svg_bulk_changes(
            None,
            Some((vec![name], selected)),
            None,
            None,
            self.renderer.p_duration_select,
        )
    }

    /// Changes the x,y coords of the node SVG representation (affects only SVG rendering).
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.node_add_fixed('A', -50, 0, None);
    /// graph.node_add_fixed('C', 450, 400, None);
    /// graph.node_add('B', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.link_add('β', 'B', 'C', true, Some(20));
    /// graph.svg_node_move('B', 0, 0);
    /// graph.svg_node_move('B', 0, 400);
    /// graph.svg_node_move('B', 400, 400);
    /// graph.svg_node_move('B', 400, 0);
    /// graph.svg_node_move('B', 0, 0);
    ///
    /// let html = graph.svg_render_animation_html("svg_node_move example");
    /// write!(File::create("example-svg_node_move.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_node_move(&mut self, name: char, cx: i32, cy: i32) -> Result<(), String> {
        self.svg_bulk_changes(
            None,
            None,
            Some(vec![(name, (cx, cy))]),
            None,
            self.renderer.p_duration_move,
        )
    }

    /// Changes the fill color of the node SVG representation (affects only SVG rendering).
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// let mut graph = Graph::new();
    /// graph.svg_automatic_animation(false);
    /// graph.node_add('A', None);
    /// graph.node_add('B', None);
    /// graph.node_add('C', None);
    /// graph.link_add('α', 'A', 'B', true, Some(10));
    /// graph.link_add('β', 'B', 'C', true, Some(20));
    /// graph.link_add('γ', 'C', 'A', true, Some(30));
    /// graph.svg_automatic_animation(true);
    /// graph.svg_layout();
    /// graph.svg_node_color('A', 255, 0, 0);
    /// graph.svg_node_color('B', 0, 255, 0);
    /// graph.svg_node_color('C', 0, 0, 255);
    /// graph.svg_node_color('A', 0, 255, 0);
    /// graph.svg_node_color('B', 0, 0, 255);
    /// graph.svg_node_color('C', 255, 0, 0);
    ///
    /// let html = graph.svg_render_animation_html("svg_node_color example");
    /// write!(File::create("example-svg_node_color.html").unwrap(), "{}", html);
    /// ```
    pub fn svg_node_color(
        &mut self,
        name: char,
        red: u8,
        green: u8,
        blue: u8,
    ) -> Result<(), String> {
        self.svg_bulk_changes(
            None,
            None,
            None,
            Some((vec![name], (red, green, blue))),
            self.renderer.p_duration_color,
        )
    }

    /// Changes the value of the duration_add parameter (affects only SVG rendering).
    /// This parameter affects the duration of the animation when a node or a link is added.
    /// Default value is 1000 ms
    pub fn svg_param_duration_add(&mut self, duration_ms: u32) {
        self.renderer.p_duration_add = duration_ms;
    }

    /// Changes the value of the duration_delete parameter (affects only SVG rendering).
    /// This parameter affects the duration of the animation when a node or a link is deleted.
    /// Default value is 1000 ms
    pub fn svg_param_duration_delete(&mut self, duration_ms: u32) {
        self.renderer.p_duration_delete = duration_ms;
    }

    /// Changes the value of the duration_move parameter (affects only SVG rendering).
    /// This parameter affects the duration of the animation when a node is moved.
    /// Default value is 1000 ms
    pub fn svg_param_duration_move(&mut self, duration_ms: u32) {
        self.renderer.p_duration_move = duration_ms;
    }

    /// Changes the value of the duration_select parameter (affects only SVG rendering).
    /// This parameter affects the duration of the animation when a node or a link is selected or deselected.
    /// Default value is 1000 ms
    pub fn svg_param_duration_select(&mut self, duration_ms: u32) {
        self.renderer.p_duration_select = duration_ms;
    }

    /// Changes the value of the duration_color parameter (affects only SVG rendering).
    /// This parameter affects the duration of the animation when a node is colored.
    /// Default value is 1000 ms
    pub fn svg_param_duration_color(&mut self, duration_ms: u32) {
        self.renderer.p_duration_color = duration_ms;
    }

    /// Changes the value of the color_tag_created parameter (affects only SVG rendering).
    /// This parameter affects the stroke color of a node or a link when just created.
    /// Default rgb  value is (0, 0, 255)
    pub fn svg_param_color_tag_created(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.renderer.p_color_tag_created = Color::new(red, green, blue);
    }

    /// Changes the value of the color_tag_selected parameter (affects only SVG rendering).
    /// This parameter affects the stroke color of a node or a link when selected.
    /// Default rgb  value is (191, 255, 0)
    pub fn svg_param_color_tag_selected(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.renderer.p_color_tag_selected = Color::new(red, green, blue);
    }

    /// Changes the value of the color_tag_deleted parameter (affects only SVG rendering).
    /// This parameter affects the color of a node or a link when deleted.
    /// Default rgb  value is (255, 0, 0)
    pub fn svg_param_color_tag_deleted(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.renderer.p_color_tag_deleted = Color::new(red, green, blue);
    }

    /// Changes the value of the color_node_fill parameter (affects only SVG rendering).
    /// This parameter affects the default fill color of a node.
    /// Default rgb  value is (255, 255, 255)
    pub fn svg_param_color_node_fill(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.renderer.p_color_node_fill = Color::new(red, green, blue);
    }

    /// Changes the value of the color_node_stroke parameter (affects only SVG rendering).
    /// This parameter affects the default stroke color of a node.
    /// Default rgb  value is (128, 139, 150)
    pub fn svg_param_color_node_stroke(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.renderer.p_color_node_stroke = Color::new(red, green, blue);
    }

    /// Changes the value of the color_link_stroke parameter (affects only SVG rendering).
    /// This parameter affects the stroke color of a link.
    /// Default rgb  value is (128, 139, 150)
    pub fn svg_param_color_link_stroke(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.renderer.p_color_link_stroke = Color::new(red, green, blue);
    }

    /// Changes the value of the color_text parameter (affects only SVG rendering).
    /// This parameter affects the font color.
    /// Default rgb  value is (0, 0, 0)
    pub fn svg_param_color_text(&mut self, (red, green, blue): (u8, u8, u8)) {
        self.renderer.p_color_text = Color::new(red, green, blue);
    }

    /// Changes the value of the display_node_label parameter (affects only SVG rendering).
    /// This parameter indicates whether the node label is displayed or not.
    /// Default value is True
    pub fn svg_param_display_node_label(&mut self, display: bool) {
        self.renderer.p_display_node_label(display);
    }

    /// Changes the value of the display_node_value parameter (affects only SVG rendering).
    /// This parameter indicates whether the node value is displayed or not.
    /// Default value is False
    pub fn svg_param_display_node_value(&mut self, display: bool) {
        self.renderer.p_display_node_value(display);
    }

    /// Changes the value of the display_link_label parameter (affects only SVG rendering).
    /// This parameter indicates whether the link label is displayed or not.
    /// Default value is False
    pub fn svg_param_display_link_label(&mut self, display: bool) {
        self.renderer.p_display_link_label(display);
    }

    /// Changes the value of the display_link_value parameter (affects only SVG rendering).
    /// This parameter indicates whether the link value is displayed or not.
    /// Default value is True
    pub fn svg_param_display_link_value(&mut self, display: bool) {
        self.renderer.p_display_link_value(display);
    }

    /// Changes the value of the stroke_width_node parameter (affects only SVG rendering).
    /// This parameter affects the stroke width of the nodes.
    /// Default value is 2
    pub fn svg_param_stroke_width_node(&mut self, width: u32) {
        self.renderer.p_stroke_width_node(width);
    }

    /// Changes the value of the stroke_width_link parameter (affects only SVG rendering).
    /// This parameter affects the stroke width of the links.
    /// Default value is 2
    pub fn svg_param_stroke_width_link(&mut self, width: u32) {
        self.renderer.p_stroke_width_link(width);
    }

    /// Changes the value of the radius_node parameter (affects only SVG rendering).
    /// This parameter affects the radius of the nodes.
    /// Default value is 20
    pub fn svg_param_radius_node(&mut self, radius: u32) {
        self.renderer.p_radius_node(radius);
    }
}
