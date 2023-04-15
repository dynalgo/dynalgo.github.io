pub mod html;
mod link;
mod node;
pub mod params;
pub mod point;
mod svg;
mod tag;

use link::Link;
use node::Node;
use params::Color;
use params::Params;
use point::Point;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::f64::consts::PI;
use svg::Svg;
use tag::Tag;

pub struct Renderer {
    id_seq: u32,
    nodes: HashMap<char, Node>,
    links: HashMap<char, Link>,
    previous_nodes: HashMap<char, Node>,
    previous_links: HashMap<char, Link>,
    initial_nodes: HashMap<char, Node>,
    initial_links: HashMap<char, Link>,
    animation: String,
    svg_renderer: Svg,
    pub params: Params,
    total_duration: u32,
    svg_first_lign: String,
}

impl Renderer {
    pub fn new() -> Renderer {
        let params = Params::new();
        let svg_renderer = Svg::new(
            params.display_node_label,
            params.display_node_value,
            params.display_link_label,
            params.display_link_value,
            params.stroke_width_node,
            params.stroke_width_link,
            params.radius_node,
        );
        Renderer {
            id_seq: 0,
            nodes: HashMap::new(),
            links: HashMap::new(),
            previous_nodes: HashMap::new(),
            previous_links: HashMap::new(),
            initial_nodes: HashMap::new(),
            initial_links: HashMap::new(),
            animation: String::new(),
            svg_renderer,
            params,
            total_duration: 0,
            svg_first_lign: String::new(),
        }
    }

    fn id_seq(&mut self) -> u32 {
        self.id_seq += 1;
        self.id_seq
    }

    pub fn node_add(&mut self, name: char, center: Point, value: u8) {
        let mut node = Node::new(
            self.id_seq(),
            name,
            center,
            value,
            self.params.radius_node,
            self.params.color_node_fill,
            self.params.color_node_border,
            self.params.color_text,
            self.params.stroke_width_node,
        );
        node.tag(Some(Tag::Created(self.params.color_tag_created)));

        self.nodes.insert(node.name(), node.clone());
        self.previous_nodes.insert(node.name(), node.clone());
        self.initial_nodes.insert(node.name(), node);

        self.animation
            .push_str(&self.svg_renderer.instantiate_node(&node));
    }

    pub fn node_delete(&mut self, name: char) {
        self.nodes
            .get_mut(&name)
            .unwrap()
            .tag(Some(Tag::Deleted(self.params.color_tag_deleted)));
    }

    pub fn node_selected(&mut self, name: char, selected: bool) {
        let tag = match selected {
            true => Some(Tag::Selected(self.params.color_tag_selected)),
            false => None,
        };
        self.nodes.get_mut(&name).unwrap().tag(tag);
    }

    pub fn link_add(&mut self, name: char, from: char, to: char, bidirect: bool, value: u8) {
        let id_seq = self.id_seq();
        let from = self.nodes.get(&from).unwrap();
        let to = self.nodes.get(&to).unwrap();

        let mut link = Link::new(
            id_seq,
            name,
            from.name(),
            to.name(),
            from.center().clone(),
            to.center().clone(),
            bidirect,
            value,
            self.params.color_link_border,
            self.params.color_text,
            self.params.stroke_width_link,
        );
        link.tag(Some(Tag::Created(self.params.color_tag_created)));

        self.links.insert(link.name(), link.clone());
        self.previous_links.insert(link.name(), link.clone());
        self.initial_links.insert(link.name(), link);

        self.animation
            .insert_str(0, &self.svg_renderer.instantiate_link(&link));
    }

    pub fn link_delete(&mut self, name: char) {
        self.links
            .get_mut(&name)
            .unwrap()
            .tag(Some(Tag::Deleted(self.params.color_tag_deleted)));
    }

    pub fn link_selected(&mut self, name: char, selected: bool) {
        let tag = match selected {
            true => Some(Tag::Selected(self.params.color_tag_selected)),
            false => None,
        };
        self.links.get_mut(&name).unwrap().tag(tag);
    }

    pub fn layout(
        &mut self,
        nodes_names: &Vec<char>,
        adjacencies: HashMap<char, HashMap<char, u8>>,
    ) {
        if nodes_names.is_empty() {
            return;
        }

        let mut fixed_center_x: i32 = 0;
        let mut fixed_center_y: i32 = 0;
        for name in nodes_names {
            let center = self.node_center(*name);
            fixed_center_x += center.x();
            fixed_center_y += center.y();
        }
        fixed_center_x = fixed_center_x / nodes_names.len() as i32;
        fixed_center_y = fixed_center_y / nodes_names.len() as i32;

        let radius = ((2 * self.params.radius_node * nodes_names.len() as u32 * 3) as f64
            / (2. * PI)) as f64;
        let angle = 2. * PI / nodes_names.len() as f64;
        for (i, name) in nodes_names.iter().enumerate() {
            let x = radius * (i as f64 * angle).cos() + fixed_center_x as f64;
            let y = radius * (i as f64 * angle).sin() + fixed_center_y as f64;
            let center = Point::new(x as i32, y as i32);
            self.node_move(*name, center);
        }
        let max_x = fixed_center_x + radius as i32;
        let max_y = fixed_center_y + radius as i32;

        for _i in 1..1000 {
            for node in nodes_names {
                let c_node = self.node_center(*node);
                let mut sum_forces_x = 0;
                let mut sum_forces_y = 0;
                for other in self.nodes.keys() {
                    let c_other = if *node == *other {
                        Point::new(fixed_center_x, fixed_center_y)
                    } else {
                        *self.node_center(*other)
                    };
                    let dx = (c_other.x() - c_node.x()) as i32;
                    let dy = (c_other.y() - c_node.y()) as i32;
                    let distance = ((dx * dx + dy * dy) as f64).sqrt() as u32;
                    let (spring_length, k) = match adjacencies.get(node).unwrap().get(other) {
                        Some(_) => ((self.params.radius_node * 10) as i32, 0.6),
                        None => ((self.params.radius_node * 25) as i32, 0.1),
                    };
                    let force = (distance as i32 - spring_length) as f64 * k;
                    let force_x = (force * (dx as f64 / distance as f64)) as i32;
                    let force_y = (force * (dy as f64 / distance as f64)) as i32;
                    sum_forces_x += force_x;
                    sum_forces_y += force_y;
                }

                let c_node = self.node_center(*node);
                let mut x_next = c_node.x() + sum_forces_x;
                x_next = min(x_next, max_x);
                x_next = max(x_next, -max_x);
                let mut y_next = c_node.y() + sum_forces_y;
                y_next = min(y_next, max_y);
                y_next = max(y_next, -max_y);

                for other in self.nodes.keys() {
                    if *node == *other {
                        continue;
                    }
                    let c_other = self.node_center(*other);
                    let dx = (c_other.x() - x_next) as i32;
                    let dy = (c_other.y() - y_next) as i32;
                    if dx.abs() < (self.params.radius_node * 2) as i32
                        && dy.abs() < (self.params.radius_node * 2) as i32
                    {
                        if sum_forces_x >= 0 {
                            x_next = c_other.x() + (self.params.radius_node * 2) as i32;
                        } else {
                            x_next = c_other.x() - (self.params.radius_node * 2) as i32;
                        }
                        if sum_forces_y >= 0 {
                            y_next = c_other.y() + (self.params.radius_node * 2) as i32;
                        } else {
                            y_next = c_other.y() - (self.params.radius_node * 2) as i32;
                        }
                        break;
                    }
                }

                self.node_move(*node, Point::new(x_next, y_next));
            }
        }
    }

    pub fn node_move(&mut self, name: char, center: Point) {
        let node = self.nodes.get_mut(&name).unwrap();
        node.move_to(center);

        for (_, link) in &mut self.links {
            if link.from() == node.name() {
                link.update_from_center(node.center().clone());
            }
            if link.to() == node.name() {
                link.update_to_center(node.center().clone());
            }
        }
    }

    pub fn node_color(&mut self, name: char, (red, green, blue): (u8, u8, u8)) {
        self.nodes.get_mut(&name).unwrap().fill_color = Color::new(red, green, blue);
    }

    pub fn node_center(&self, name: char) -> &Point {
        self.nodes.get(&name).unwrap().center()
    }

    fn animate_viewbox(&mut self, duration: u32) -> String {
        let mut x_min_curr = i32::MAX;
        let mut x_max_curr = i32::MIN;
        let mut y_min_curr = i32::MAX;
        let mut y_max_curr = i32::MIN;
        for (_, node) in &self.previous_nodes {
            if node.tag_created() {
                if self.nodes.get(&node.name()).unwrap().tag_created() {
                    continue;
                }
            }
            x_min_curr = min(x_min_curr, node.center().x());
            x_max_curr = max(x_max_curr, node.center().x());
            y_min_curr = min(y_min_curr, node.center().y());
            y_max_curr = max(y_max_curr, node.center().y());
        }

        let mut x_min_next = i32::MAX;
        let mut x_max_next = i32::MIN;
        let mut y_min_next = i32::MAX;
        let mut y_max_next = i32::MIN;
        for (_, node) in &self.nodes {
            if node.tag_deleted() {
                continue;
            }
            x_min_next = min(x_min_next, node.center().x());
            x_max_next = max(x_max_next, node.center().x());
            y_min_next = min(y_min_next, node.center().y());
            y_max_next = max(y_max_next, node.center().y());
        }

        if x_min_curr == i32::MAX {
            x_min_curr = x_min_next;
            x_max_curr = x_max_next;
            y_min_curr = y_min_next;
            y_max_curr = y_max_next;
        }

        if self.svg_first_lign.is_empty() {
            self.svg_first_lign = self
                .svg_renderer
                .instanciate_viewbox(x_min_curr, x_max_curr, y_min_curr, y_max_curr);

            String::new()
        } else {
            self.svg_renderer.animate_viewbox(
                x_min_curr,
                x_max_curr,
                y_min_curr,
                y_max_curr,
                x_min_next,
                x_max_next,
                y_min_next,
                y_max_next,
                duration,
                self.total_duration,
            )
        }
    }

    pub fn animate(&mut self, duration: u32) {
        let mut svg = String::new();

        svg.push_str(&self.animate_viewbox(duration));

        for (name, current_link) in self.links.iter() {
            let initial_link = self.initial_links.get(name).unwrap();
            let previous_link = self.previous_links.get(name).unwrap();
            svg.push_str(&self.svg_renderer.animate_link(
                &current_link,
                &initial_link,
                &previous_link,
                duration,
                self.total_duration,
            ));
        }
        for (name, current_node) in self.nodes.iter() {
            let initial_node = self.initial_nodes.get(name).unwrap();
            let previous_node = self.previous_nodes.get(name).unwrap();
            svg.push_str(&self.svg_renderer.animate_node(
                &current_node,
                &initial_node,
                &previous_node,
                duration,
                self.total_duration,
            ));
        }
        self.animation.push_str(&svg);

        self.total_duration += duration;
        self.previous_nodes = self.nodes.clone();
        self.previous_links = self.links.clone();

        // clean
        let mut untag_created = Vec::new();
        let mut useless = Vec::new();
        for (name, link) in self.links.iter() {
            if link.tag_created() {
                untag_created.push(name.clone());
            }
            if link.tag_deleted() {
                useless.push(name.clone());
            }
        }
        for name in untag_created {
            self.links.get_mut(&name).unwrap().tag(None);
        }
        for name in useless {
            self.links.remove(&name);
            self.initial_links.remove(&name);
            self.previous_links.remove(&name);
        }

        let mut untag_created = Vec::new();
        let mut useless = Vec::new();
        for (name, node) in self.nodes.iter() {
            if node.tag_created() {
                untag_created.push(name.clone());
            }
            if node.tag_deleted() {
                useless.push(name.clone());
            }
        }
        for name in untag_created {
            self.nodes.get_mut(&name).unwrap().tag(None);
        }
        for name in useless {
            self.nodes.remove(&name);
            self.initial_nodes.remove(&name);
            self.previous_nodes.remove(&name);
        }
    }

    pub fn animation(&self) -> String {
        let mut svg = String::new();

        if self.svg_first_lign.is_empty() {
            return svg;
        }

        svg.push_str(&self.svg_first_lign);
        svg.push_str(&self.animation);
        svg.push_str("</svg>");

        svg
    }
}
