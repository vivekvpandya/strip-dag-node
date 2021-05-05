extern crate clap;
mod graph;

use crate::graph::Graph;
use clap::{App, Arg};
use std::str::FromStr;

fn main() {
    let app = App::new("strip-dag-node")
                .version("0.1.0")
                .about("Reads graph with --in argument and node to be deleted with --strip. Create a graph print the graph and reprint graph after deleting the node specified in --strip")
                .author("Vivek Pandya")
                .arg(Arg::with_name("input")
                            .short("i")
                            .long("in")
                            .value_name("in_str")
                            .required(true)
    //                        .index(1)
                            .help("Provide input graph , exmple a-b,b-c,c-d here a->b , b->c and c->d and -> represent directed edge between nodes. A single char is unconnected node")
                            .takes_value(true))
                .arg(Arg::with_name("strip")
                            .short("s")
                            .long("strip")
                            .value_name("strip_node")
                            .required(true)
      //                      .index(2)
                            .help("Provide node to be stripped")
                            .takes_value(true));
    let matches = app.get_matches();
    let input_graph = matches.value_of("input").unwrap();
    let strip_node = matches.value_of("strip").unwrap();

    //println!("{:?} {:?}", input_graph, strip_node);
    let g = Graph::from_str(input_graph);
    match g {
        Ok(mut graph) => {
            let graph_ref = &mut graph;
            if graph_ref.delete_node(strip_node.chars().next().unwrap()) {
                println!("{}", graph.to_string());
            } else {
                println!(
                    "Error deleteing node {} from {}",
                    strip_node,
                    graph.to_string()
                );
            }
        }
        Err(e) => {
            println!(
                "Error : {},  Could not create graph from input {}",
                e.to_string(),
                input_graph
            );
        }
    }
}
