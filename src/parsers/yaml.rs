use yaml_rust::Yaml;
use abstract_pipeline_runner::*;
use std::collections::HashMap;

use super::super::Parser;
use super::super::ParserNodeType;
use super::super::ParserNodeType::*;

const KWD_TASK: &str = "run";

fn create_property_from_yaml_hash(yaml: &Yaml) -> Property {
    if let Yaml::Hash(h) = yaml {
        let mut hashmap = HashMap::new();
        for (k, v) in h {
            if let Some(s) = k.as_str() {
                hashmap.insert(s.into(), create_property_from_yaml_hash(v));
            }
        }
        Property::Map(hashmap)
    } else {
        Property::Simple(get_yaml_key_as_string(yaml))
    }
}

fn get_yaml_key_as_string(yaml: &Yaml) -> String {
    match yaml {
        Yaml::Real(s) => s.into(),
        Yaml::Integer(i) => i.to_string(),
        Yaml::String(s) => s.into(),
        Yaml::Boolean(b) => b.to_string(),
        Yaml::Null => "null".into(),

        // TODO:
        // Yaml::Array(_) => {}
        // Yaml::Hash(_) => {}
        // Yaml::Alias(_) => {}
        // Yaml::BadValue => {}
        _ => "".into(),
    }
}

fn yaml_hash_has_key(yaml: &Yaml, key: &str) -> bool {
    match yaml {
        Yaml::Hash(h) => h.keys().any(|k| k.as_str() == Some(key)),
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub enum Property {
    Simple(String),
    Map(HashMap<String, Property>)
}
impl Parser<Property> for Yaml {
    // modify the constants at the top
    // of this file, if you want to customize
    // the semantics of your pipeline file
    fn kwd_task(&self) -> &str { KWD_TASK }
    // TODO:
    // potentially overwrite the other keywords


    fn get_node_type(&self) -> ParserNodeType {
        if yaml_hash_has_key(self, self.kwd_series()) {
            ParserNodeTypeSeries
        } else if yaml_hash_has_key(self, self.kwd_parallel()) {
            ParserNodeTypeParallel
        } else if yaml_hash_has_key(self, self.kwd_task()) {
            ParserNodeTypeTask
        } else if let Yaml::String(_) = self {
            // if its just a single string, it's probably
            // a task
            ParserNodeTypeTask
        } else {
            ParserNodeTypeKnown
        }
    }

    fn get_node_name<'a>(&'a self) -> Option<&'a str> {
        if let Yaml::Hash(h) = self {
            if yaml_hash_has_key(self, self.kwd_name()) {
                for (k, v) in h {
                    match (k.as_str(), v.as_str()) {
                        (Some(key), Some(value)) => {
                            if key == self.kwd_name() {
                                return Some(value);
                            }
                        },
                        _ => (),
                    }
                }
            }
        }
        None
    }

    fn collect_node_vec<'a, U: Task<Property> + Clone>(&'a self, task: &'a U, node_type: ParserNodeType) -> Vec<Node<Property, U>> {
        let kwd = if node_type == ParserNodeTypeParallel {
            self.kwd_parallel()
        } else if node_type == ParserNodeTypeSeries {
            self.kwd_series()
        } else {
            panic!("unsupported usage")
        };

        let mut node_vec = vec![];
        if let Yaml::Array(yaml_array) = &self[kwd] {
            for yaml_obj in yaml_array {
                let node_obj = yaml_obj.make_node(task);
                if node_obj.is_some() {
                    node_vec.push(node_obj.unwrap());
                }
            }
        }
        node_vec
    }
    fn create_task_node<'a, U: Task<Property> + Clone>(&'a self, task: &'a U) -> Option<Node<Property, U>> {
        if let Yaml::Hash(h) = self {
            let mut node = Node {
                name: None,
                is_root_node: false,
                ntype: NodeTypeTask,
                task: None,
                properties: HashMap::new(),
                continue_on_fail: false,
            };
            node.ntype = NodeTypeTask;
            for (k, v) in h {
                if let Some(s) = k.as_str() {
                    if s == self.kwd_name() && v.as_str().is_some() {
                        node.name = Some(v.as_str().unwrap());
                    }
                    let property = create_property_from_yaml_hash(v);
                    node.properties.insert(s.into(), property);
                }
            }
            node.task = Some(task);
            return Some(node);
        } else if let Yaml::String(s) = self {
            let mut node = Node {
                name: None,
                is_root_node: false,
                ntype: NodeTypeTask,
                task: None,
                properties: HashMap::new(),
                continue_on_fail: false,
            };
            node.ntype = NodeTypeTask;
            node.properties.insert(self.kwd_task(), Property::Simple(s.into()));
            node.task = Some(task);
            return Some(node);
        }
        None
    }
}
