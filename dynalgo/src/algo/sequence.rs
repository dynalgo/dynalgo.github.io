//! Use of the sequence of a graph

use crate::algo::utils::Names;
use crate::graph::Graph;

pub struct Sequence {}

impl Sequence {
    /// Indicates whether a graph exists that has this sequence, or not
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::algo::sequence::Sequence;
    /// let is_valid = Sequence::is_valid(vec![1,1,2]);
    /// assert!(is_valid);
    /// ```
    pub fn is_valid(sequence: Vec<usize>) -> bool {
        if sequence.iter().sum::<usize>() % 2 != 0 {
            return false;
        }

        match Self::to_graph(sequence, None) {
            Err(_) => false,
            Ok(_) => true,
        }
    }

    /// Returns the sequence of a complete graph of a specified order
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    /// use dynalgo::algo::sequence::Sequence;
    /// let sequence = Sequence::complete(5);
    /// assert_eq!(sequence , vec![4, 4, 4, 4, 4]);
    /// let graph = Sequence::to_graph(sequence, Some(vec!['V', 'W', 'X', 'Y', 'Z'])).unwrap();
    ///
    /// let html = graph.svg_render_animation_html("sequence-complete example");
    /// write!(File::create("example-sequence-complete.html").unwrap(), "{}", html);
    /// ```
    pub fn complete(graph_order: usize) -> Vec<usize> {
        let mut sequence = Vec::new();
        for _ in 0..graph_order {
            sequence.push(graph_order - 1);
        }
        sequence
    }

    /// creates a Graph structure from a given sequence
    ///
    /// # Example:
    ///
    /// ```
    /// use dynalgo::graph::Graph;
    /// use std::fs::File;
    /// use std::io::Write;
    /// use dynalgo::algo::sequence::Sequence;
    /// let graph = Sequence::to_graph(vec![1,1,2], None).unwrap();
    ///
    /// let html = graph.svg_render_animation_html("sequence-to_graph example");
    /// write!(File::create("example-sequence-to_graph.html").unwrap(), "{}", html);
    /// ```
    pub fn to_graph(sequence: Vec<usize>, node_names: Option<Vec<char>>) -> Result<Graph, String> {
        let node_names = match node_names {
            Some(names) => names,
            None => Names::latin(sequence.len())?,
        };
        let mut link_names = Names::greek(sequence.iter().sum())?;

        let mut sequence: Vec<(char, usize)> =
            node_names.into_iter().zip(sequence.into_iter()).collect();
        let mut graph = Graph::new();
        graph.svg_automatic_animation(false);
        graph.svg_automatic_layout(false);
        while !sequence.is_empty() {
            sequence.sort_by(|(_, d1), (_, d2)| d2.cmp(d1));
            let (node, degree) = sequence.remove(0);
            if !graph.nodes_list().contains(&node) {
                graph.node_add(node, None)?;
            }
            if degree == 0 {
                continue;
            }
            if sequence.len() < degree {
                Err("Sequence is not valid")?;
            }
            for (n, d) in sequence.iter_mut().take(degree) {
                *d -= 1;
                if !graph.nodes_list().contains(n) {
                    graph.node_add(*n, None)?;
                }
                graph.link_add(link_names.pop().unwrap(), node, *n, true, None)?;
            }
        }
        graph.svg_layout();
        graph.svg_animate(500);

        Ok(graph)
    }
}
