use super::link::Link;
use super::node::Node;

pub struct Svg {
    pub p_display_node_label: bool,
    pub p_display_node_value: bool,
    pub p_display_link_label: bool,
    pub p_display_link_value: bool,
    pub p_stroke_width_node: u32,
    pub p_stroke_width_link: u32,
    pub p_radius_node: u32,
}

impl Svg {
    pub fn new(
        p_display_node_label: bool,
        p_display_node_value: bool,
        p_display_link_label: bool,
        p_display_link_value: bool,
        p_stroke_width_node: u32,
        p_stroke_width_link: u32,
        p_radius_node: u32,
    ) -> Svg {
        Svg {
            p_display_node_label,
            p_display_node_value,
            p_display_link_label,
            p_display_link_value,
            p_stroke_width_node,
            p_stroke_width_link,
            p_radius_node,
        }
    }

    pub fn instantiate_node(&self, node: &Node) -> String {
        let mut svg = String::new();

        svg.push_str(&format!("<g id=\"{}\" opacity=\"{}\">\n", node.id(), 0));
        svg.push_str(&format!(
            "  <circle id=\"c{}\" cx=\"{}\" cy=\"{}\" r=\"{}\" ",
            node.id(),
            node.center().x(),
            node.center().y(),
            self.p_radius_node
        ));
        svg.push_str(&format!(
            "fill=\"rgb({},{},{})\" ",
            node.fill_color.r(),
            node.fill_color.g(),
            node.fill_color.b()
        ));
        svg.push_str(&format!(
            "stroke=\"rgb({},{},{})\" stroke-width=\"{}\"></circle>\n",
            node.stroke_color_tagged().r(),
            node.stroke_color_tagged().g(),
            node.stroke_color_tagged().b(),
            self.p_stroke_width_node
        ));

        if self.p_display_node_label {
            let (dx, dy) = match self.p_display_node_value {
                false => (-6, 6),
                true => (-6, -2),
            };
            svg.push_str(&format!(
                "  <text id=\"co{}\" x=\"{}\" y=\"{}\" dx=\"{}\" dy=\"{}\" ",
                node.id(),
                node.center().x(),
                node.center().y(),
                dx,
                dy
            ));
            svg.push_str(&format!(
                "fill=\"rgb({},{},{})\">{}</text>\n",
                node.text_color.r(),
                node.text_color.g(),
                node.text_color.b(),
                node.name()
            ));
        }
        if self.p_display_node_value && node.value().is_some() {
            let (dx, dy) = match self.p_display_node_label {
                false => (-5, -5),
                true => (-12, 13),
            };
            svg.push_str(&format!(
                "  <text id=\"m{}\" x=\"{}\" y=\"{}\" dx=\"{}\" dy=\"{}\" ",
                node.id(),
                node.center().x(),
                node.center().y(),
                dx,
                dy
            ));
            svg.push_str(&format!(
                "fill=\"rgb({},{},{})\">{}</text>\n",
                node.text_color.r(),
                node.text_color.g(),
                node.text_color.b(),
                node.value().unwrap()
            ));
        }
        svg.push_str("</g>\n");

        svg
    }

    pub fn instantiate_link(&self, link: &Link) -> String {
        let mut svg = String::new();

        svg.push_str(&format!(
            "<path id=\"{}\" stroke-width=\"{}\" opacity=\"{}\"",
            link.id(),
            self.p_stroke_width_link,
            0
        ));
        svg.push_str(&format!(
            " stroke=\"rgb({},{},{})\" d=\"M{} {} L{} {} Z\" />\n",
            link.stroke_color_tagged().r(),
            link.stroke_color_tagged().g(),
            link.stroke_color_tagged().b(),
            link.from_center().x(),
            link.from_center().y(),
            link.to_center().x(),
            link.to_center().y()
        ));

        svg.push_str(&format!("<g id=\"lib{}\" opacity=\"{}\">\n", link.id(), 0));

        if self.p_display_link_label {
            let (dx, dy) = match self.p_display_link_value {
                false => (-5, -5),
                true => (-5, -3),
            };
            svg.push_str(&format!(
                "  <text id=\"co{}\" x=\"{}\" y=\"{}\" dx=\"{}\" dy=\"{}\"",
                link.id(),
                (link.from_center().x() + link.to_center().x()) / 2 as i32,
                (link.from_center().y() + link.to_center().y()) / 2 as i32,
                dx,
                dy
            ));
            svg.push_str(&format!(
                " fill=\"rgb({},{},{})\">{}</text>\n",
                link.text_color.r(),
                link.text_color.g(),
                link.text_color.b(),
                link.name()
            ));
        }

        if self.p_display_link_value && link.value().is_some() {
            let (dx, dy) = match self.p_display_link_label {
                false => (-5, -5),
                true => (-12, 14),
            };
            svg.push_str(&format!(
                "  <text id=\"m{}\" x=\"{}\" y=\"{}\" dx=\"{}\" dy=\"{}\" ",
                link.id(),
                (link.from_center().x() + link.to_center().x()) / 2 as i32,
                (link.from_center().y() + link.to_center().y()) / 2 as i32,
                dx,
                dy
            ));
            svg.push_str(&format!(
                " fill=\"rgb({},{},{})\">{}</text>\n",
                link.text_color.r(),
                link.text_color.g(),
                link.text_color.b(),
                link.value().unwrap()
            ));
        }
        svg.push_str("</g>\n");

        if !link.bidirect() {
            svg.push_str(&format!(
                "<text id=\"bi{}\" fill=\"rgb({},{},{})\" opacity=\"{}\">\n",
                link.id(),
                link.text_color.r(),
                link.text_color.g(),
                link.text_color.b(),
                0
            ));
            svg.push_str(&format!(
                "<textpath startOffset=\"{}\" href=\"#{}\">⇒</textpath>\n",
                self.p_radius_node,
                link.id()
            ));
            svg.push_str("</text>\n");
        }

        svg
    }

    pub fn instanciate_viewbox(
        &self,
        x_min_init: i32,
        x_max_init: i32,
        y_min_init: i32,
        y_max_init: i32,
    ) -> String {
        let mut svg = String::new();

        svg.push_str(&format!(
            "\n<svg class=\"svg_dynalgo\" onclick=\"pause(this)\" viewBox=\"{} {} {} {}\" preserveAspectRatio=\"xMidYMid meet\">\n",
            x_min_init - 2 * self.p_radius_node as i32,
            y_min_init - 2 * self.p_radius_node as i32,
            x_max_init - x_min_init + 4 * self.p_radius_node as i32,
            y_max_init - y_min_init + 4 * self.p_radius_node as i32
        ));

        svg
    }

    pub fn animate_viewbox(
        &self,
        x_min_curr: i32,
        x_max_curr: i32,
        y_min_curr: i32,
        y_max_curr: i32,
        x_min_next: i32,
        x_max_next: i32,
        y_min_next: i32,
        y_max_next: i32,
        duration: u32,
        start_time: u32,
    ) -> String {
        let mut svg = String::new();

        if x_min_curr == x_min_next
            && y_min_curr == y_min_next
            && x_max_curr == x_max_next
            && y_max_curr == y_max_next
        {
            return svg;
        }

        svg.push_str(&format!(
            "<animate attributeName=\"viewBox\" from=\"{} {} {} {}\" ",
            x_min_curr - 2 * self.p_radius_node as i32,
            y_min_curr - 2 * self.p_radius_node as i32,
            x_max_curr - x_min_curr + 4 * self.p_radius_node as i32,
            y_max_curr - y_min_curr + 4 * self.p_radius_node as i32
        ));
        svg.push_str(&format!(
            "to=\"{} {} {} {}\" begin=\"{}ms\" dur=\"{}ms\" fill=\"freeze\" />\n",
            x_min_next - 2 * self.p_radius_node as i32,
            y_min_next - 2 * self.p_radius_node as i32,
            x_max_next - x_min_next + 4 * self.p_radius_node as i32,
            y_max_next - y_min_next + 4 * self.p_radius_node as i32,
            start_time,
            duration
        ));

        svg
    }

    pub fn animate_node(
        &self,
        current: &Node,
        initial: &Node,
        previous: &Node,
        duration: u32,
        start_time: u32,
    ) -> String {
        let mut svg = String::new();

        if previous.center().x() != current.center().x()
            || previous.center().y() != current.center().y()
        {
            let dx_curr = previous.center().x() - initial.center().x();
            let dy_curr = previous.center().y() - initial.center().y();

            let dx_next = current.center().x() - previous.center().x();
            let dy_next = current.center().y() - previous.center().y();

            svg.push_str(&format!(
                "<animateMotion href=\"#{}\" begin=\"{}ms\" dur=\"{}ms\"
                    fill=\"freeze\" path=\"m {} {} l {} {}\" />\n",
                current.id(),
                start_time,
                duration,
                dx_curr,
                dy_curr,
                dx_next,
                dy_next
            ));
        }

        if current.tag_created() || current.tag_deleted() {
            let (opacity_curr, opacity_next) = match current.tag_created() {
                true => (0, 1),
                false => (1, 0),
            };
            svg.push_str(&format!(
                "<animate href=\"#{}\" attributeName=\"opacity\" ",
                current.id()
            ));
            svg.push_str(&format!(
                "from=\"{}\" to=\"{}\" ",
                opacity_curr, opacity_next
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        if !current.tag_created() && previous.tag_created() {
            let color_curr = previous.stroke_color_tagged();
            let color_next = current.stroke_color_tagged();
            svg.push_str(&format!(
                "<animate href=\"#c{}\" attributeName=\"stroke\" ",
                current.id()
            ));
            svg.push_str(&format!(
                "from=\"rgb({},{},{})\" to=\"rgb({},{},{})\" ",
                color_curr.r(),
                color_curr.g(),
                color_curr.b(),
                color_next.r(),
                color_next.g(),
                color_next.b()
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        if current.tag_deleted() && !previous.tag_deleted() {
            let color_curr = previous.stroke_color_tagged();
            let color_next = current.stroke_color_tagged();
            svg.push_str(&format!(
                "<animate href=\"#c{}\" attributeName=\"stroke\" ",
                current.id()
            ));
            svg.push_str(&format!(
                "from=\"rgb({},{},{})\" to=\"rgb({},{},{})\" ",
                color_curr.r(),
                color_curr.g(),
                color_curr.b(),
                color_next.r(),
                color_next.g(),
                color_next.b()
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        if current.tag_selected() != previous.tag_selected() {
            let (width_curr, width_next) = match current.tag_selected() {
                true => (current.stroke_width, 2 * current.stroke_width),
                false => (2 * current.stroke_width, current.stroke_width),
            };
            svg.push_str(&format!(
                "<animate href=\"#c{}\" attributeName=\"stroke-width\" ",
                current.id()
            ));
            svg.push_str(&format!("from=\"{}\" to=\"{}\" ", width_curr, width_next));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));

            let color_curr = previous.stroke_color_tagged();
            let color_next = current.stroke_color_tagged();
            svg.push_str(&format!(
                "<animate href=\"#c{}\" attributeName=\"stroke\" ",
                current.id()
            ));
            svg.push_str(&format!(
                "from=\"rgb({},{},{})\" to=\"rgb({},{},{})\" ",
                color_curr.r(),
                color_curr.g(),
                color_curr.b(),
                color_next.r(),
                color_next.g(),
                color_next.b()
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        if current.fill_color != previous.fill_color {
            let color_curr = previous.fill_color;
            let color_next = current.fill_color;
            svg.push_str(&format!(
                "<animate href=\"#c{}\" attributeName=\"fill\" ",
                current.id(),
            ));
            svg.push_str(&format!(
                "from=\"rgb({},{},{})\" to=\"rgb({},{},{})\" ",
                color_curr.r(),
                color_curr.g(),
                color_curr.b(),
                color_next.r(),
                color_next.g(),
                color_next.b()
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        svg
    }

    pub fn animate_link(
        &self,
        current: &Link,
        initial: &Link,
        previous: &Link,
        duration: u32,
        start_time: u32,
    ) -> String {
        let mut svg = String::new();

        if previous.from_center().x() != current.from_center().x()
            || previous.from_center().y() != current.from_center().y()
            || previous.to_center().x() != current.to_center().x()
            || previous.to_center().y() != current.to_center().y()
        {
            svg.push_str(&format!("<animate href=\"#{}\" ", current.id()));
            svg.push_str(&format!(
                "begin=\"{}ms\" fill=\"freeze\" attributeName=\"d\" dur=\"{}ms\" ",
                start_time, duration
            ));
            svg.push_str(&format!(
                "values=\"M{} {} L{} {} Z;M{} {} L{} {} Z\" />\n",
                previous.from_center().x(),
                previous.from_center().y(),
                previous.to_center().x(),
                previous.to_center().y(),
                current.from_center().x(),
                current.from_center().y(),
                current.to_center().x(),
                current.to_center().y(),
            ));

            let midle_next_x = (current.from_center().x() + current.to_center().x()) / 2 as i32;
            let midle_next_y = (current.from_center().y() + current.to_center().y()) / 2 as i32;
            let midle_curr_x = (previous.from_center().x() + previous.to_center().x()) / 2 as i32;
            let midle_curr_y = (previous.from_center().y() + previous.to_center().y()) / 2 as i32;
            let midle_init_x = (initial.from_center().x() + initial.to_center().x()) / 2 as i32;
            let midle_init_y = (initial.from_center().y() + initial.to_center().y()) / 2 as i32;

            let dx_curr = midle_curr_x - midle_init_x;
            let dy_curr = midle_curr_y - midle_init_y;
            let dx_next = midle_next_x - midle_curr_x;
            let dy_next = midle_next_y - midle_curr_y;

            svg.push_str(&format!(
                "<animateMotion href=\"#lib{}\" begin=\"{}ms\" dur=\"{}ms\" ",
                current.id(),
                start_time,
                duration
            ));
            svg.push_str(&format!(
                "fill=\"freeze\" path=\"m {} {} l {} {}\" />\n",
                dx_curr, dy_curr, dx_next, dy_next
            ));
        }

        if current.tag_created() || current.tag_deleted() {
            let (opacity_curr, opacity_next) = match current.tag_created() {
                true => (0, 1),
                false => (1, 0),
            };

            svg.push_str(&format!(
                "<animate href=\"#{}\" attributeName=\"opacity\" from=\"{}\" to=\"{}\" ",
                current.id(),
                opacity_curr,
                opacity_next
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));

            svg.push_str(&format!(
                "<animate href=\"#lib{}\" attributeName=\"opacity\" from=\"{}\" to=\"{}\" ",
                current.id(),
                opacity_curr,
                opacity_next
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));

            svg.push_str(&format!(
                "<animate href=\"#bi{}\" attributeName=\"opacity\" from=\"{}\" to=\"{}\" ",
                current.id(),
                opacity_curr,
                opacity_next
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        if !current.tag_created() && previous.tag_created() {
            let color_curr = previous.stroke_color_tagged();
            let color_next = current.stroke_color_tagged();
            svg.push_str(&format!(
                "<animate href=\"#{}\" attributeName=\"stroke\" ",
                current.id()
            ));
            svg.push_str(&format!(
                "from=\"rgb({},{},{})\" to=\"rgb({},{},{})\" ",
                color_curr.r(),
                color_curr.g(),
                color_curr.b(),
                color_next.r(),
                color_next.g(),
                color_next.b()
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        if current.tag_deleted() && !previous.tag_deleted() {
            let color_curr = previous.stroke_color_tagged();
            let color_next = current.stroke_color_tagged();
            svg.push_str(&format!(
                "<animate href=\"#{}\" attributeName=\"stroke\" ",
                current.id()
            ));
            svg.push_str(&format!(
                "from=\"rgb({},{},{})\" to=\"rgb({},{},{})\" ",
                color_curr.r(),
                color_curr.g(),
                color_curr.b(),
                color_next.r(),
                color_next.g(),
                color_next.b()
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        if current.tag_selected() != previous.tag_selected() {
            let (width_curr, width_next) = match current.tag_selected() {
                true => (current.stroke_width, 2 * current.stroke_width),
                false => (2 * current.stroke_width, current.stroke_width),
            };

            svg.push_str(&format!(
                "<animate href=\"#{}\" attributeName=\"stroke-width\" ",
                current.id()
            ));
            svg.push_str(&format!("from=\"{}\" to=\"{}\" ", width_curr, width_next));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));

            let color_curr = previous.stroke_color_tagged();
            let color_next = current.stroke_color_tagged();
            svg.push_str(&format!(
                "<animate href=\"#{}\" attributeName=\"stroke\" ",
                current.id()
            ));
            svg.push_str(&format!(
                "from=\"rgb({},{},{})\" to=\"rgb({},{},{})\" ",
                color_curr.r(),
                color_curr.g(),
                color_curr.b(),
                color_next.r(),
                color_next.g(),
                color_next.b()
            ));
            svg.push_str(&format!(
                "dur=\"{}ms\" begin=\"{}ms\" fill=\"freeze\"/>\n",
                duration, start_time
            ));
        }

        svg
    }
}
