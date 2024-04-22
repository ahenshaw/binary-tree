// use std::collections::HashMap;

// pub type NodeKey = (usize, usize);
// pub type Nodes = HashMap<NodeKey, Node>;
// pub type Edges = HashMap<(NodeKey, NodeKey), Edge>;

// pub enum NodeState {
//     Default,
//     Disabled,
//     Highlighted,
// }

// pub enum EdgeState {
//     Default,
//     Disabled,
//     Highlighted,
//     Animated(f32),
// }

// pub struct Node {
//     pub x: f32,
//     pub y: f32,
//     pub label: String,
//     pub state: NodeState,
// }
// impl Node {
//     fn new() -> Self {
//         Self {
//             x: 0.,
//             y: 0.,
//             label: String::new(),
//             state: NodeState::Default,
//         }
//     }
// }

// pub struct Edge {
//     pub state: EdgeState,
// }

// pub fn populate(num_vars: usize) -> (Nodes, Edges) {
//     let mut nodes = Nodes::new();
//     let mut edges = Edges::new();

//     for var in 0..num_vars {
//         let num_nodes = 2usize.pow(var as u32);
//         for index in 0..num_nodes {
//             nodes.insert((var, index), Node::new());
//         }
//     }
//     for &(var, index) in nodes.keys() {
//         let child0 = (var + 1, index * 2);
//         let child1 = (var + 1, index * 2 + 1);
//         if nodes.contains_key(&child0) {
//             edges.insert(
//                 ((var, index), child0),
//                 Edge {
//                     state: EdgeState::Default,
//                 },
//             );
//         }
//         if nodes.contains_key(&child1) {
//             edges.insert(
//                 ((var, index), child1),
//                 Edge {
//                     state: EdgeState::Default,
//                 },
//             );
//         }
//     }
//     (nodes, edges)
// }
