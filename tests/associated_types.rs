use std::fmt;

trait Graph<N, E> {
    fn has_edge(&self, &N, &N) -> bool;
    fn edges(&self, &N) -> Vec<E>;
}

fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {
    0
}

trait Graph_v2 {
    type N;
    type E;

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

fn distance_v2<G: Graph_v2>(graph: &G, start: &G::N, end: &G::N) -> u32 {
    0
}

struct Node;
struct Edge;
struct MyGraph;

impl Graph_v2 for MyGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        true
    }

    fn edges(&self, n: &Node) -> Vec<Edge> {
        Vec::new()
    }
}

// won't compile. Don't know associated types
//#[test]
//fn test_failing_trait_object() {
//    let graph = MyGraph;
//    let obj = Box::new(graph) as Box<Graph_v2>;
//}

#[test]
fn test_associated_type_trait_object() {
    let graph = MyGraph;
    let obj = Box::new(graph) as Box<Graph_v2<N=Node, E=Edge>>;
}

