# abstract-pipeline-parsers

This library contains a collection of parsers (at this time it only has a parser for `yaml_rust`) that are implemented for certain data types.

For example, the yaml parser can be used as such:

```rs
use yaml_rust::Yaml;

// this imports the Parser trait
use abstract_pipeline_parsers::*;
// this imports the Parser for Yaml implementation
// as well as a Property enum that the implementation
// uses
use abstract_pipeline_parsers::parsers::yaml::*;
// this contains the Node struct which is what root_node is
use abstract_pipeline_runner::*;

fn main() {
    let task = SomeCustomTask {};
    let my_yaml_str = get_yaml_string();
    let my_yaml_vec = load_from_str(my_yaml_str);
    let my_yaml_obj: &Yaml = &my_yaml_obj[0];
    let mut root_node = my_yaml_obj.make_node(&task);
}

```


