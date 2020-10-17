use yaml_rust::Yaml;
use abstract_pipeline_runner::*;
use std::collections::HashMap;

pub mod parsers;


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ParserNodeType {
    ParserNodeTypeSeries,
    ParserNodeTypeParallel,
    ParserNodeTypeTask,
    ParserNodeTypeKnown,
}
pub use ParserNodeType::*;

/// This parser trait is to be implemented
/// by the user to be able to parse potentially any
/// file format into a hierarchy of nodes.
/// this parser is the generic parser that is used
/// by the more specific parsers included in this library
pub trait Parser<T: Send + Sync + Clone> {
    // these are methods you must implement as a user
    fn get_node_type(&self) -> ParserNodeType;
    fn create_task_node<'a, U: Task<T> + Clone>(&'a self, task: &'a U) -> Option<Node<T, U>>;
    fn collect_node_vec<'a, U: Task<T> + Clone>(&'a self, task: &'a U, node_type: ParserNodeType) -> Vec<Node<T, U>>;

    // these are methods you can implement if you
    // want to customize the behavior a little bit
    fn kwd_name(&self) -> &str { "name" }
    fn kwd_series(&self) -> &str { "series" }
    fn kwd_parallel(&self) -> &str { "parallel" }
    fn kwd_task(&self) -> &str { "task" }
    fn get_node_name<'a>(&'a self) -> Option<&'a str> { None }

    // this is a method you should only implement if you want really
    // specific behavior. this default should work well in most cases
    fn make_node<'a, U: Task<T> + Clone>(&'a self, task: &'a U) -> Option<Node<T, U>> {
        let node_type = self.get_node_type();
        if node_type == ParserNodeTypeParallel || node_type == ParserNodeTypeSeries {
            let mut node = Node {
                name: None,
                is_root_node: false,
                ntype: NodeTypeTask,
                task: None,
                properties: HashMap::new(),
                continue_on_fail: false,
            };
            node.name = self.get_node_name();
            let node_vec = self.collect_node_vec(task, node_type);
            node.ntype = if node_type == ParserNodeTypeParallel {
                NodeTypeParallel(node_vec)
            } else {
                // otherwhise its series
                NodeTypeSeries(node_vec)
            };
            return Some(node);
        }

        if node_type == ParserNodeTypeTask {
            return self.create_task_node(task);
        }

        // above we handled the case of series, paralle, and task nodes
        // but there can also be known nodes, which we do not parse here.
        // Instead, use a seperate convenience method for collecting
        // known nodes. This is because known nodes do not get put into
        // the node hierarchy, but are rather stored seperately in a global
        // context, to be accessed by any node in the hiearchy as needed
        None
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
