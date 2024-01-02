use crate::graph::Graph;

pub struct Transform;

impl Transform {
    pub fn undirect(g: &mut Graph) {
        let m = g.adjacency_matrix();
        let nodes = g.nodes();
        for node_from in nodes.iter() {
            for node_to in nodes.iter() {
                let value = m[node_from][node_to];
                if value.is_some() && (m[node_to][node_from]).is_none() {
                    g.delete_link(*node_from, *node_to);
                    g.add_link(*node_to, *node_from, true, value.unwrap());
                }
            }
        }
    }

    pub fn transpose(g: &mut Graph) {
        let m = g.adjacency_matrix();
        let nodes = g.nodes();
        for node_from in nodes.iter() {
            for node_to in nodes.iter() {
                let value = m[node_from][node_to];
                if value.is_some() && (m[node_to][node_from]).is_none() {
                    g.delete_link(*node_from, *node_to);
                    g.add_link(*node_to, *node_from, false, value.unwrap());
                }
            }
        }
    }
}
