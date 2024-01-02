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
use std::collections::BTreeMap;
use std::collections::HashMap;

use std::f64::consts::PI;
use std::sync::Mutex;
use svg::Svg;
use tag::Tag;

static SEQ: Mutex<u32> = Mutex::new(0);

pub struct Renderer {
    nodes: BTreeMap<char, Node>,
    links: BTreeMap<(char, char), Link>,
    previous_nodes: BTreeMap<char, Node>,
    previous_links: BTreeMap<(char, char), Link>,
    initial_nodes: BTreeMap<char, Node>,
    initial_links: BTreeMap<(char, char), Link>,
    animation: String,
    svg: Svg,
    pub p_color_node_fill: Color,
    pub p_color_node_stroke: Color,
    pub p_color_link_stroke: Color,
    pub p_color_node_text: Color,
    pub p_color_link_text: Color,
    total_duration: u32,
    svg_first_lign: String,
}

impl Renderer {
    pub fn new(
        show_value: bool,
        show_name: bool,
        p_color_node_fill: Color,
        p_color_node_stroke: Color,
        p_color_link_stroke: Color,
        p_color_node_text: Color,
        p_color_link_text: Color,
        p_radius_node: u8,
    ) -> Renderer {
        let p_display_node_label = show_name;
        let p_display_link_value = show_value;
        let p_stroke_width_node = 2;
        let p_stroke_width_link = 2;
        let svg = Svg::new(
            p_display_node_label,
            p_display_link_value,
            p_stroke_width_node,
            p_stroke_width_link,
            p_radius_node,
        );
        Renderer {
            nodes: BTreeMap::new(),
            links: BTreeMap::new(),
            previous_nodes: BTreeMap::new(),
            previous_links: BTreeMap::new(),
            initial_nodes: BTreeMap::new(),
            initial_links: BTreeMap::new(),
            animation: String::new(),
            svg,
            p_color_node_fill,
            p_color_node_stroke,
            p_color_link_stroke,
            p_color_node_text,
            p_color_link_text,
            total_duration: 0,
            svg_first_lign: String::new(),
        }
    }

    fn id_seq(&mut self) -> u32 {
        let mut seq = SEQ.lock().unwrap();
        *seq += 1;
        *seq
    }

    pub fn sleep(&mut self, duration: u32) {
        self.total_duration += duration;
    }

    pub fn duration(&self) -> u32 {
        self.total_duration
    }

    pub fn add_node(&mut self, name: char, center: Option<Point>) {
        let mut node = Node::new(
            self.id_seq(),
            name,
            center,
            self.svg.p_radius_node,
            self.p_color_node_fill,
            self.p_color_node_stroke,
            self.p_color_node_text,
            self.svg.p_stroke_width_node,
        );
        node.tag(Some(Tag::Created));

        self.nodes.insert(node.name(), node.clone());
        self.previous_nodes.insert(node.name(), node.clone());
        self.initial_nodes.insert(node.name(), node);

        self.animation.push_str(&self.svg.instantiate_node(&node));
    }

    pub fn delete_node(&mut self, name: char) {
        self.nodes.get_mut(&name).unwrap().tag(Some(Tag::Deleted));
    }

    pub fn add_link(&mut self, from: char, to: char, bidirect: bool, value: i8) {
        let id_seq = self.id_seq();
        let from = self.nodes.get(&from).unwrap();
        let to = self.nodes.get(&to).unwrap();

        let mut link = Link::new(
            id_seq,
            from.name(),
            to.name(),
            from.center().clone(),
            to.center().clone(),
            bidirect,
            value,
            self.p_color_link_stroke,
            self.p_color_link_text,
            self.svg.p_stroke_width_link,
        );
        link.tag(Some(Tag::Created));

        self.links.insert((from.name(), to.name()), link.clone());
        self.previous_links
            .insert((from.name(), to.name()), link.clone());
        self.initial_links.insert((from.name(), to.name()), link);

        self.animation
            .insert_str(0, &self.svg.instantiate_link(&link));
    }

    pub fn delete_link(&mut self, from: char, to: char) {
        let link = match self.links.get_mut(&(from, to)) {
            None => self.links.get_mut(&(to, from)).unwrap(),
            Some(l) => l,
        };
        link.tag(Some(Tag::Deleted));
    }

    pub fn layout(&mut self, adja: BTreeMap<char, BTreeMap<char, i8>>) {
        assert!(adja.len() == self.nodes.len());

        if adja.is_empty() {
            return;
        }

        let distance = |x1: i64, y1: i64, x2: i64, y2: i64| {
            (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt() as u64
        };

        let mut avg_x: i64 = 0;
        let mut avg_y: i64 = 0;
        let mut freezed_count = 0;
        let mut radius_min = 0;
        let mut forces = BTreeMap::new();
        let mut positions = BTreeMap::new();
        for node in adja.keys() {
            let xy = self.node_center(*node);
            let freezed = self.node_center_freezed(*node);
            positions.insert(*node, (freezed, xy.x() as i64, xy.y() as i64));
            if freezed {
                avg_x += xy.x() as i64;
                avg_y += xy.y() as i64;
                freezed_count += 1;
                continue;
            }
            forces.insert(*node, (0, 0));
        }

        if freezed_count > 0 {
            if adja.keys().len() == freezed_count {
                return;
            }
            avg_x = avg_x / freezed_count as i64;
            avg_y = avg_y / freezed_count as i64;
            for (_, (freezed, x, y)) in &positions {
                if *freezed {
                    radius_min = max(radius_min, distance(*x, *y, avg_x, avg_y));
                }
            }
        }

        let diameter = 2 * self.svg.p_radius_node;
        let perimeter =
            (diameter as u32 * (positions.keys().len() - freezed_count) as u32 * 2) as f64;
        let mut radius = (perimeter / (2. * PI)) as f64;
        radius = max(radius as u32, radius_min as u32 + 2 * diameter as u32) as f64;
        let angle = 2. * PI / (positions.keys().len() - freezed_count) as f64;

        let mut i = 0;
        for (_, (freezed, x, y)) in positions.iter_mut() {
            if *freezed {
                continue;
            }
            *x = (radius * (i as f64 * angle).cos() + avg_x as f64) as i64;
            *y = (radius * (i as f64 * angle).sin() + avg_y as f64) as i64;
            i += 1;
        }

        let links_count: usize = adja
            .iter()
            .map(|(_, n)| n.len())
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>()
            / 2;

        let mut density = links_count / adja.len();
        density = max(1, density);
        let mut springs = HashMap::new();
        let length_unit = self.svg.p_radius_node as u32 * density as u32;
        let k_unit = 0.01;
        for node in positions.keys() {
            for other in positions.keys() {
                if *other == *node {
                    continue;
                }

                let (length, k) = if adja.get(node).unwrap().get(other).is_some()
                //    || adja.get(other).unwrap().get(node).is_some()
                {
                    match max(
                        adja.get(node).unwrap().len(),
                        adja.get(other).unwrap().len(),
                    ) {
                        n if n == 1 => (5 * length_unit as i64, 30. * k_unit),
                        n if n == 2 => (6 * length_unit as i64, 25. * k_unit),
                        n if n == 3 => (7 * length_unit as i64, 20. * k_unit),
                        n if n == 4 => (8 * length_unit as i64, 15. * k_unit),
                        _ => (9 * length_unit as i64, 10. * k_unit),
                    }
                } else {
                    (
                        max(24, adja.len() / 2) as i64 * length_unit as i64,
                        5. * k_unit,
                    )
                };
                springs.insert((*node, *other), (length, k));
            }
        }

        let mut not_moved = Vec::with_capacity(adja.len());
        for (node, neighbors) in &adja {
            not_moved.push((*node, neighbors.len()));
        }
        not_moved.sort_by(|(_, l1), (_, l2)| l2.cmp(l1));
        let mut not_moved: Vec<char> = not_moved.into_iter().map(|(n, _)| n).collect();

        for i in 0..(10 * adja.len()) {
            if i % adja.len() == 0 && not_moved.len() > 0 {
                for node in &not_moved {
                    let (freezed, _, _) = positions.get(node).unwrap();
                    if *freezed {
                        continue;
                    }
                    let neighbors = adja.get(&node).unwrap();
                    if neighbors.len() < 1 {
                        continue;
                    }

                    let mut try_x: i64 = 0;
                    let mut try_y: i64 = 0;
                    if neighbors.len() == 1 {
                        let (other, _) = neighbors.iter().next().unwrap();
                        let (_, o_x, o_y) = positions.get(&other).unwrap();
                        let dist = distance(*o_x, *o_y, 0, 0);
                        try_x = o_x + (1.1 * diameter as f64 * *o_x as f64 / dist as f64) as i64;
                        try_y = o_y + (1.1 * diameter as f64 * *o_y as f64 / dist as f64) as i64;
                    } else {
                        for (other, _) in neighbors {
                            let (_, o_x, o_y) = positions.get(&other).unwrap();
                            try_x += o_x;
                            try_y += o_y;
                        }
                        try_x = 10 + try_x / neighbors.len() as i64;
                        try_y = 10 + try_y / neighbors.len() as i64;
                    }

                    let mut collision = false;
                    for (other, (_, o_x, o_y)) in &positions {
                        if *other == *node {
                            continue;
                        }
                        collision = distance(try_x, try_y, *o_x, *o_y) <= diameter as u64;
                        if collision {
                            break;
                        }
                    }
                    if !collision {
                        let (_, n_x, n_y) = positions.get_mut(node).unwrap();
                        *n_x = try_x;
                        *n_y = try_y;
                    }
                }
                not_moved.clear();
            }

            let mut max_f = 0;
            for (node, (n_freezed, n_x, n_y)) in &positions {
                if *n_freezed {
                    continue;
                }

                let mut sum_f_x = 0;
                let mut sum_f_y = 0;
                for (other, (_, o_x, o_y)) in &positions {
                    if *other == *node {
                        continue;
                    }

                    let dist = distance(*n_x, *n_y, *o_x, *o_y);
                    assert!(dist >= diameter as u64);

                    let (d_x, d_y) = (o_x - n_x, o_y - n_y);
                    let (spring_length, k) = springs.get(&(*node, *other)).unwrap();
                    let force = (dist as i64 - spring_length) as f64 * k;
                    let f_x = (force * (d_x as f64 / dist as f64)) as i64;
                    let f_y = (force * (d_y as f64 / dist as f64)) as i64;
                    sum_f_x += f_x;
                    sum_f_y += f_y;
                }

                let f = distance(sum_f_x, sum_f_y, 0, 0) as i64;
                max_f = max(max_f, f);

                let (f_x, f_y) = forces.get_mut(node).unwrap();
                *f_x = sum_f_x;
                *f_y = sum_f_y;
            }

            let maxi = 6 * radius as i64 / adja.len() as i64;
            if max_f > maxi {
                let reduce = maxi as f64 / max_f as f64;
                for (_, (f_x, f_y)) in forces.iter_mut() {
                    *f_x = (*f_x as f64 * reduce) as i64;
                    *f_y = (*f_y as f64 * reduce) as i64;
                }
            }

            for (node, (f_x, f_y)) in &forces {
                let f = distance(*f_x, *f_y, 0, 0);
                if f <= length_unit as u64 / 10 {
                    continue;
                };
                let (freezed, n_x, n_y) = positions.get(node).unwrap();
                if *freezed {
                    continue;
                }
                let mut new_x = *n_x;
                let mut new_y = *n_y;
                for m in (0..=4).rev() {
                    let try_x = *n_x + ((f * m / 4) as f64 * (*f_x as f64 / f as f64)) as i64;
                    let try_y = *n_y + ((f * m / 4) as f64 * (*f_y as f64 / f as f64)) as i64;
                    let mut collision = false;
                    for (other, (_, o_x, o_y)) in &positions {
                        if *other == *node {
                            continue;
                        }
                        collision = distance(try_x, try_y, *o_x, *o_y) <= diameter as u64;
                        if collision {
                            assert!(m != 0);
                            break;
                        }
                    }
                    if !collision {
                        new_x = try_x;
                        new_y = try_y;
                        break;
                    }
                }
                if *n_x != new_x || *n_y != new_y {
                    let (_, n_x, n_y) = positions.get_mut(node).unwrap();
                    *n_x = new_x;
                    *n_y = new_y;
                } else {
                    if !not_moved.contains(node) {
                        not_moved.push(*node);
                    }
                }
            }

            if not_moved.len() == adja.len() {
                break;
            }
        }

        for (node, (freezed, n_x, n_y)) in &positions {
            if !freezed {
                self.node_move(*node, Point::new((*n_x) as i32, (*n_y) as i32), false);
            }
        }
    }

    pub fn node_move(&mut self, name: char, center: Point, freezed: bool) {
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

        self.node_freezed(name, freezed);
    }

    pub fn node_freezed(&mut self, name: char, freezed: bool) {
        self.nodes.get_mut(&name).unwrap().center_freeze(freezed);
    }

    pub fn node_fill_color(&mut self, name: char, (red, green, blue): (u8, u8, u8)) {
        self.nodes
            .get_mut(&name)
            .unwrap()
            .set_fill_color(Color::new(red, green, blue));
    }

    pub fn node_text_color(&mut self, name: char, (red, green, blue): (u8, u8, u8)) {
        self.nodes
            .get_mut(&name)
            .unwrap()
            .set_text_color(Color::new(red, green, blue));
    }

    pub fn link_text_color(
        &mut self,
        name_1: char,
        name_2: char,
        (red, green, blue): (u8, u8, u8),
    ) {
        let link = match self.links.get_mut(&(name_1, name_2)) {
            None => self.links.get_mut(&(name_2, name_1)).unwrap(),
            Some(l) => l,
        };

        link.set_text_color(Color::new(red, green, blue));
    }

    pub fn node_stroke_color(&mut self, name: char, (red, green, blue): (u8, u8, u8)) {
        self.nodes
            .get_mut(&name)
            .unwrap()
            .set_stroke_color(Color::new(red, green, blue));
    }

    pub fn link_stroke_color(
        &mut self,
        name_1: char,
        name_2: char,
        (red, green, blue): (u8, u8, u8),
    ) {
        let link = match self.links.get_mut(&(name_1, name_2)) {
            None => self.links.get_mut(&(name_2, name_1)).unwrap(),
            Some(l) => l,
        };

        link.set_stroke_color(Color::new(red, green, blue));
    }

    pub fn node_center(&self, name: char) -> &Point {
        self.nodes.get(&name).unwrap().center()
    }

    pub fn node_center_freezed(&self, name: char) -> bool {
        self.nodes.get(&name).unwrap().center_freezed()
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
}
