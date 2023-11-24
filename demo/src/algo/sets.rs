use dynalgo::graph::Graph;
use dynalgo::graph::GraphError;

pub struct Sets {}

impl Sets {
    pub fn union(graph_1: &mut Graph, graph_2: &mut Graph) -> Result<Graph, GraphError> {
        graph_1.anim_pause()?;
        graph_1.param_color_tag_selected(0, 196, 0);
        graph_1.anim_nodes_select(graph_1.nodes_list(), true)?;
        graph_1.anim_links_select(graph_1.links_list(), true)?;

        let mut g_union = Graph::new();
        g_union.anim_pause()?;
        g_union.append_from_graph(&graph_1)?;

        let nodes_1 = graph_1.nodes_list();
        let nodes_2 = graph_2.nodes_list();
        graph_2.param_color_tag_selected(0, 196, 0);

        graph_2.anim_pause()?;
        for node_2 in &nodes_2 {
            if !nodes_1.contains(node_2) {
                graph_2.anim_node_select(*node_2, true)?;
                g_union.node_add(*node_2)?;
            }
        }
        let adja_2 = graph_2.adjacency_list();
        for (node, neighbors) in &adja_2 {
            for (neighbor, link_value) in neighbors {
                if g_union.node_link_to(*node, *neighbor)?.is_some() {
                    continue;
                }

                graph_2.anim_link_select(*node, *neighbor, true)?;
                let value = g_union.node_link_to(*neighbor, *node)?;
                if value.is_some() {
                    g_union.link_delete(*neighbor, *node)?;
                    g_union.link_add(*node, *neighbor, true, value.unwrap())?;
                } else {
                    let bidirect = graph_2.node_link_to(*neighbor, *node)?.is_some();
                    g_union.link_add(*node, *neighbor, bidirect, *link_value)?;
                }
            }
        }

        graph_1.anim_step(1000)?;
        graph_2.anim_step(1000)?;

        g_union.anim_step(1)?;
        g_union.param_color_tag_selected(0, 196, 0);
        g_union.anim_nodes_select(g_union.nodes_list(), true)?;
        g_union.anim_links_select(g_union.links_list(), true)?;
        g_union.anim_step(1000)?;

        graph_1.anim_resume()?;
        graph_2.anim_resume()?;
        g_union.anim_resume()?;

        Ok(g_union)
    }
}
