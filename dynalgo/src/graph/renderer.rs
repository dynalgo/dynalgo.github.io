pub mod color;
pub mod html;
mod link;
mod node;
pub mod point;
mod svg;
mod tag;

use color::Color;
use link::Link;
use node::Node;
use point::Point;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Mutex;
use svg::Svg;
use tag::Tag;

static SEQ: Mutex<u32> = Mutex::new(0);

pub struct Renderer {
    //id_seq: u32,
    nodes: HashMap<char, Node>,
    links: HashMap<char, Link>,
    previous_nodes: HashMap<char, Node>,
    previous_links: HashMap<char, Link>,
    initial_nodes: HashMap<char, Node>,
    initial_links: HashMap<char, Link>,
    animation: String,
    svg: Svg,
    pub p_duration_add: u32,
    pub p_duration_delete: u32,
    pub p_duration_move: u32,
    pub p_duration_select: u32,
    pub p_duration_color: u32,
    pub p_color_tag_created: Color,
    pub p_color_tag_selected: Color,
    pub p_color_tag_deleted: Color,
    pub p_color_node_fill: Color,
    pub p_color_node_stroke: Color,
    pub p_color_link_stroke: Color,
    pub p_color_text: Color,
    total_duration: u32,
    svg_first_lign: String,
}

impl Renderer {
    pub fn new() -> Renderer {
        let p_display_node_label = true;
        let p_display_node_value = false;
        let p_display_link_label = false;
        let p_display_link_value = true;
        let p_stroke_width_node = 2;
        let p_stroke_width_link = 2;
        let p_radius_node = 20;
        let svg = Svg::new(
            p_display_node_label,
            p_display_node_value,
            p_display_link_label,
            p_display_link_value,
            p_stroke_width_node,
            p_stroke_width_link,
            p_radius_node,
        );
        Renderer {
            //id_seq: 0,
            nodes: HashMap::new(),
            links: HashMap::new(),
            previous_nodes: HashMap::new(),
            previous_links: HashMap::new(),
            initial_nodes: HashMap::new(),
            initial_links: HashMap::new(),
            animation: String::new(),
            svg,
            p_duration_add: 500,
            p_duration_delete: 500,
            p_duration_move: 500,
            p_duration_select: 500,
            p_duration_color: 500,
            p_color_tag_created: Color::new(0, 0, 255),
            p_color_tag_selected: Color::new(191, 255, 0),
            p_color_tag_deleted: Color::new(255, 0, 0),
            p_color_node_fill: Color::new(255, 255, 255),
            p_color_node_stroke: Color::new(128, 139, 150),
            p_color_link_stroke: Color::new(128, 139, 150),
            p_color_text: Color::new(0, 0, 0),
            total_duration: 0,
            svg_first_lign: String::new(),
        }
    }

    fn id_seq(&mut self) -> u32 {
        let mut seq = SEQ.lock().unwrap();
        *seq += 1;
        //self.id_seq += 1;
        //self.id_seq
        *seq
    }

    pub fn node_add(&mut self, name: char, center: Option<Point>, value: Option<u8>) {
        let mut node = Node::new(
            self.id_seq(),
            name,
            center,
            value,
            self.svg.p_radius_node,
            self.p_color_node_fill,
            self.p_color_node_stroke,
            self.p_color_text,
            self.svg.p_stroke_width_node,
        );
        node.tag(Some(Tag::Created(self.p_color_tag_created)));

        self.nodes.insert(node.name(), node.clone());
        self.previous_nodes.insert(node.name(), node.clone());
        self.initial_nodes.insert(node.name(), node);

        self.animation.push_str(&self.svg.instantiate_node(&node));
    }

    pub fn node_delete(&mut self, name: char) {
        self.nodes
            .get_mut(&name)
            .unwrap()
            .tag(Some(Tag::Deleted(self.p_color_tag_deleted)));
    }

    pub fn node_selected(&mut self, name: char, selected: bool) {
        let tag = match selected {
            true => Some(Tag::Selected(self.p_color_tag_selected)),
            false => None,
        };
        self.nodes.get_mut(&name).unwrap().tag(tag);
    }

    pub fn link_add(
        &mut self,
        name: char,
        from: char,
        to: char,
        bidirect: bool,
        value: Option<u8>,
    ) {
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
            self.p_color_link_stroke,
            self.p_color_text,
            self.svg.p_stroke_width_link,
        );
        link.tag(Some(Tag::Created(self.p_color_tag_created)));

        self.links.insert(link.name(), link.clone());
        self.previous_links.insert(link.name(), link.clone());
        self.initial_links.insert(link.name(), link);

        self.animation
            .insert_str(0, &self.svg.instantiate_link(&link));
    }

    pub fn link_delete(&mut self, name: char) {
        self.links
            .get_mut(&name)
            .unwrap()
            .tag(Some(Tag::Deleted(self.p_color_tag_deleted)));
    }

    pub fn link_selected(&mut self, name: char, selected: bool) {
        let tag = match selected {
            true => Some(Tag::Selected(self.p_color_tag_selected)),
            false => None,
        };
        self.links.get_mut(&name).unwrap().tag(tag);
    }

    pub fn layout(
        &mut self,
        nodes_names: &Vec<char>,
        adjacencies: HashMap<char, HashMap<char, (char, Option<u8>)>>,
    ) {
        if nodes_names.is_empty() {
            return;
        }

        let all_nodes: Vec<char> = self.nodes.keys().cloned().collect();

        let mut fixed_center_x: i32 = 0;
        let mut fixed_center_y: i32 = 0;
        for name in &all_nodes {
            let center = self.node_center(*name);
            fixed_center_x += center.x();
            fixed_center_y += center.y();
        }
        fixed_center_x = fixed_center_x / self.nodes.len() as i32;
        fixed_center_y = fixed_center_y / self.nodes.len() as i32;

        let radius =
            ((2 * self.svg.p_radius_node * self.nodes.len() as u32 * 4) as f64 / (2. * PI)) as f64;
        let angle = 2. * PI / self.nodes.len() as f64;

        for (i, name) in nodes_names.iter().enumerate() {
            if self.node_center_fixed(*name) {
                continue;
            }
            if nodes_names.len() == 1 {
                let square_size = (self.nodes.len() as f64).sqrt().ceil() as u32;
                let dx = ((self.nodes.len() as u32) % square_size) as i32 * 10;
                let dy = ((self.nodes.len() as u32) / square_size) as i32 * 10;
                self.node_move(*name, Point::new(fixed_center_x + dx, fixed_center_y + dy));
            } else {
                let x = radius * (i as f64 * angle).cos() + fixed_center_x as f64;
                let y = radius * (i as f64 * angle).sin() + fixed_center_y as f64;
                self.node_move(*name, Point::new(x as i32, y as i32));
            }
        }

        let density = if (self.links.len() as f64 / self.nodes.len() as f64) < 1.2 {
            1.2
        } else {
            self.links.len() as f64 / self.nodes.len() as f64
        };
        let max_x = fixed_center_x + (density * radius) as i32;
        let max_y = fixed_center_y + (density * radius) as i32;

        for _i in 1..(200. * density) as u32 {
            for node in &all_nodes {
                if self.node_center_fixed(*node) {
                    continue;
                }
                let c_node = self.node_center(*node);
                let mut sum_forces_x = 0;
                let mut sum_forces_y = 0;

                for other in &all_nodes {
                    let c_other = if *node == *other {
                        continue;
                    } else {
                        *self.node_center(*other)
                    };
                    let mut dx = (c_other.x() - c_node.x()) as i32;
                    let mut dy = (c_other.y() - c_node.y()) as i32;
                    let mut distance = ((dx * dx + dy * dy) as f64).sqrt() as u32;
                    if dx == 0 && dy == 0 {
                        dx = 1;
                        dy = 1;
                        distance = 1;
                    }
                    let (spring_length, k) = if adjacencies.get(node).unwrap().get(other).is_some()
                        || adjacencies.get(other).unwrap().get(node).is_some()
                    {
                        (
                            (self.svg.p_radius_node * (10. * density) as u32) as i32,
                            0.6,
                        )
                    } else {
                        (
                            (self.svg.p_radius_node * (25. * density) as u32) as i32,
                            0.1,
                        )
                    };
                    let force = (distance as i32 - spring_length) as f64 * k;
                    let force_x = (force * (dx as f64 / distance as f64)) as i32;
                    let force_y = (force * (dy as f64 / distance as f64)) as i32;
                    sum_forces_x += force_x;
                    sum_forces_y += force_y;
                }

                let mut x_next = c_node.x() + sum_forces_x;
                let mut y_next = c_node.y() + sum_forces_y;
                x_next = min(x_next, max_x);
                x_next = max(x_next, -max_x);
                y_next = min(y_next, max_y);
                y_next = max(y_next, -max_y);

                let mut collision = true;
                while collision && (x_next.abs() <= max_x) && (y_next.abs() <= max_y) {
                    for other in &all_nodes {
                        collision = false;
                        if *node == *other {
                            continue;
                        }
                        let c_other = self.node_center(*other);
                        let dx = (c_other.x() - x_next) as i32;
                        let dy = (c_other.y() - y_next) as i32;
                        if dx.abs() < (self.svg.p_radius_node * 2) as i32
                            && dy.abs() < (self.svg.p_radius_node * 2) as i32
                        {
                            if sum_forces_x == 0 && sum_forces_y == 0 {
                                break;
                            }
                            collision = true;

                            if sum_forces_x > 0 {
                                x_next = c_other.x() + (self.svg.p_radius_node * 3) as i32;
                            } else if sum_forces_x < 0 {
                                x_next = c_other.x() - (self.svg.p_radius_node * 3) as i32;
                            }
                            if sum_forces_y > 0 {
                                y_next = c_other.y() + (self.svg.p_radius_node * 3) as i32;
                            } else if sum_forces_y < 0 {
                                y_next = c_other.y() - (self.svg.p_radius_node * 3) as i32;
                            }
                        }
                    }
                }

                self.node_move(*node, Point::new(x_next, y_next));
            }
        }
    }

    pub fn node_move(&mut self, name: char, center: Point) {
        let node = self.nodes.get_mut(&name).unwrap();

        let center_curr = node.center();
        if center_curr.x() == center.x() && center_curr.y() == center.y() {
            return;
        }

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

    pub fn node_center_fixed(&self, name: char) -> bool {
        self.nodes.get(&name).unwrap().center_fixed()
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

        if x_min_next == i32::MAX {
            x_min_next = -500;
            x_max_next = 500;
            y_min_next = -500;
            y_max_next = 500;
        }

        if x_min_curr == i32::MAX {
            x_min_curr = x_min_next;
            x_max_curr = x_max_next;
            y_min_curr = y_min_next;
            y_max_curr = y_max_next;
        }

        if self.svg_first_lign.is_empty() {
            self.svg_first_lign = self
                .svg
                .instanciate_viewbox(x_min_curr, x_max_curr, y_min_curr, y_max_curr);

            String::new()
        } else {
            self.svg.animate_viewbox(
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

    /*
        pub fn sleep(&mut self, duration: u32) {
            self.total_duration += duration;
        }
    */

    pub fn animate(&mut self, duration: u32) {
        let mut svg = String::new();

        svg.push_str(&self.animate_viewbox(duration));

        for (name, current_link) in self.links.iter() {
            let initial_link = self.initial_links.get(name).unwrap();
            let previous_link = self.previous_links.get(name).unwrap();
            svg.push_str(&self.svg.animate_link(
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
            svg.push_str(&self.svg.animate_node(
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

    pub fn p_display_node_label(&mut self, display: bool) {
        self.svg.p_display_node_label = display;
    }

    pub fn p_display_node_value(&mut self, display: bool) {
        self.svg.p_display_node_value = display;
    }

    pub fn p_display_link_label(&mut self, display: bool) {
        self.svg.p_display_link_label = display;
    }

    pub fn p_display_link_value(&mut self, display: bool) {
        self.svg.p_display_link_value = display;
    }

    pub fn p_stroke_width_node(&mut self, width: u32) {
        self.svg.p_stroke_width_node = width;
    }

    pub fn p_stroke_width_link(&mut self, width: u32) {
        self.svg.p_stroke_width_link = width;
    }

    pub fn p_radius_node(&mut self, radius: u32) {
        self.svg.p_radius_node = radius;
    }
}
