//! # dynalgo
//!
//! `dynalgo` is a tiny RUST library designed to produce animated SVG images that can illustrate graph algorithms in action.
//!
//! The crate offers a basic `graph` structure representation (nodes, links and adjacency list).
//! The interesting point is that each modification of the structure of the graph results in an animation in SVG with SMIL language.
//! Additionally, custom animations can be made by playing with the properties of graphical representations of nodes and links.
//! Graph's nodes are automatically layouted according to imaginary spring forces applied to nodes. `Algo` module will provide basic algorithms to apply to graph.
//!
//!
//! # Example n°1 :
//! ## (add nodes and links, and then play with their graphical representation as SVG elements. Finally, display the resulting animation as an animated SVG in a HTML page)
//!
//! ```
//! use dynalgo::graph::Graph;
//! use std::fs::File;
//! use std::io::Write;
//!
//! let mut graph = Graph::new();
//!
//! graph.svg_automatic_layout(false);
//! graph.svg_automatic_animation(false);
//!
//! graph.node_add('A', None);
//! graph.node_add('B', None);
//! graph.node_add('C', None);
//! graph.link_add('α', 'A', 'B', true, Some(10));
//! graph.link_add('β', 'B', 'C', true, Some(20));
//! graph.link_add('γ', 'C', 'A', true, Some(30));
//!
//! graph.svg_automatic_animation(true);
//! graph.svg_layout();
//! graph.svg_automatic_layout(true);
//!
//! graph.nodes_exchange('A', 'B');
//!
//! graph.svg_node_color('A', 0,128,0);
//! graph.svg_node_color('C' ,128,0,0);
//!
//! graph.link_delete('γ');
//! graph.node_delete('B');
//! graph.node_add('D', None);
//! graph.link_add('δ', 'C', 'D', false, Some(40));
//! graph.link_add('ε', 'D', 'A', false, Some(50));
//!
//! graph.svg_node_selected('D',true);
//! graph.svg_link_selected('δ',true);
//! graph.svg_link_selected('ε',true);
//!
//! let html = graph.svg_render_animation_html("This is the example n°1");
//! write!(File::create("example-1.html").unwrap(), "{}", html);
//! ```
//!
//! # Example n°2 :
//! ## (build a graph from a formatted String, and then DFS algorithm traverses it. Finally the traversed graph is layouted as an animated SVG in a HTML page)
//!
//! ```
//! use dynalgo::graph::Graph;
//! use dynalgo::algo::travers::Dfs;
//! use std::fs::File;
//! use std::io::Write;
//!
//! let mut graph = Graph::new();
//! let dyna = String::from(
//!        "N A _ _ 1
//!         N B _ _ 2
//!         N C _ _ 3
//!         N D _ _ 4
//!         N E _ _ 5
//!         N F _ _ 6
//!         N G _ _ 7
//!         N H _ _ 8
//!         N I _ _ 9
//!         N J _ _ _
//!         N K _ _ 11
//!         N L _ _ 12
//!         N M _ _ _
//!         N N _ _ 14
//!         N O _ _ 15
//!         N P _ _ 16
//!         N Q _ _ 17
//!         N R _ _ 18
//!         N S _ _ 19
//!         N T _ _ 21
//!         N U _ _ 22
//!         L a B G true 1
//!         L b F C true 2
//!         L c B C true 3
//!         L d C G true 4
//!         L e G F false 5
//!         L f F B true 6
//!         L g F E true 7
//!         L h F J true 8
//!         L i E I true 9
//!         L j I J false _
//!         L k K J true 11
//!         L l A J true 12
//!         L m I A true 13
//!         L n K G true 14
//!         L o K D false 15
//!         L p K H true 16
//!         L q K L true 17
//!         L r L M true 18
//!         L s L S true 19
//!         L t L O false _
//!         L u N O true 21
//!         L v N P true 22
//!         L w P Q true 23
//!         L x P R true 24
//!         L y P T false 25
//!         L z T U true 26"
//! );
//! graph.dyna_from(dyna);
//!
//! let dfs = Dfs::new();
//! dfs.run(&mut graph, None);
//! let html = graph.svg_render_animation_html("This is the example n°2");
//! write!(File::create("example-2.html").unwrap(), "{}", html).unwrap();   
//! ```
//!
//! # Example n°3 :
//! ## (for fun, build a graph that represents a maze, and then use DFS algorithm to traverse it to reach arrival. Finally the maze is layouted as an animated SVG in a HTML page)
//!
//! ```
//! use dynalgo::algo::fun::Maze;
//! use std::fs::File;
//! use std::io::Write;
//!
//! let maze = Maze::new();
//! let graph = maze.run(9);
//! let html = graph.svg_render_animation_html("This is the example n°3");
//! write!(File::create("example-3.html").unwrap(), "{}", html).unwrap();  
//! ```

pub mod algo;
pub mod graph;

#[cfg(test)]
mod tests {
    use crate::graph::Graph;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn it_works() -> Result<(), String> {
        let mut graph = Graph::new();

        let dyna = String::from(
            "N X _ _ 24
         N Y _ _ 25
         N Z _ _ 26
         L x X Y true 24
         L y Y Z true 25
         L z Z X true 26",
        );
        graph.dyna_from(dyna)?;

        graph.svg_automatic_animation(false);
        graph.svg_automatic_layout(false);

        graph.node_add('A', None)?;
        graph.node_add('B', None)?;
        graph.link_add('a', 'A', 'B', true, Some(10))?;

        graph.svg_automatic_animation(true);
        graph.svg_layout();
        graph.svg_automatic_layout(true);

        graph.node_add('C', None)?;
        graph.link_add('b', 'B', 'C', true, Some(20))?;
        graph.link_add('c', 'C', 'A', true, Some(30))?;
        graph.link_add('l', 'A', 'X', true, Some(30))?;
        graph.link_add('m', 'B', 'Y', true, Some(30))?;
        graph.link_add('n', 'C', 'Z', true, Some(30))?;

        graph.nodes_exchange('A', 'B')?;

        graph.svg_node_color('C', 128, 0, 0)?;

        graph.link_delete('n')?;
        graph.node_delete('A')?;
        graph.node_add('A', None)?;
        graph.link_add('a', 'A', 'B', true, Some(10))?;

        graph.svg_node_selected('A', true)?;
        graph.svg_link_selected('a', true)?;
        graph.svg_node_selected('A', false)?;
        graph.svg_link_selected('a', false)?;

        graph.node_delete('A')?;
        graph.node_add_fixed('A', 500, 500, None)?;
        graph.link_add('a', 'A', 'B', true, Some(10))?;

        graph.svg_node_move('A', -500, -500)?;

        graph.svg_layout();

        let adjacency: HashMap<char, HashMap<char, (char, Option<u8>)>> = graph.adjacency_list();
        for (node_from, adjacent) in adjacency {
            for (node_to, link) in adjacent {
                println!("Can go from node {} to node {}", node_from, node_to);
                match link {
                    (link_name, Some(value)) => println!("Link {} has value {}", link_name, value),
                    (link_name, None) => println!("Link is {}", link_name),
                }
            }
        }

        let html = graph.svg_render_animation_html("It works example");
        let mut html_file = match File::create("example-it_works.html") {
            Ok(f) => f,
            Err(e) => Err(format!("Error opening file : {}", e))?,
        };
        match write!(html_file, "{}", html) {
            Ok(_) => {}
            Err(e) => Err(format!("Error writing to file : {}", e))?,
        };

        let dyna = graph.dyna_to();
        let mut dyna_file = match File::create("example-it_works.dyna") {
            Ok(f) => f,
            Err(e) => Err(format!("Error opening file : {}", e))?,
        };
        match write!(dyna_file, "{}", dyna) {
            Ok(_) => {}
            Err(e) => Err(format!("Error writing to file : {}", e))?,
        };

        Ok(())
    }

    #[test]
    fn it_doesnt_work() {
        let mut graph = Graph::new();
        let dyna = String::from(
            "N X _ _ _
         N X _ _ _",
        );
        assert!(graph.dyna_from(dyna).is_err());

        let mut graph = Graph::new();
        let dyna = String::from(
            "N X _ _ _
         N Y _ _ _
         L x X Y true _
         L y Y X true _",
        );
        assert!(graph.dyna_from(dyna).is_err());

        assert!(graph.node_delete('A').is_err());
        assert!(graph.svg_node_color('A', 0, 0, 0).is_err());
        assert!(graph.svg_node_selected('A', true).is_err());
        assert!(graph.link_delete('a').is_err());
        assert!(graph.svg_link_selected('a', true).is_err());
    }
}
