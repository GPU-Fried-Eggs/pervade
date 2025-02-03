use std::collections::HashMap;

use oxc::{
    ast::{ast::{JSXAttributeItem, JSXElementName, JSXOpeningElement}, visit, Visit},
    span::Span
};

pub struct NetworkConfig {
    protocol: String,
}

pub struct NetworkNode {
    id: String,
    config: NetworkConfig,
    code_span: Span,
}

pub struct NetworkAnalyzer {
    node: petgraph::Graph<ComponentNode, DependencyEdge>,
}

impl ComponentAnalyzer {
    pub fn new(filters: Vec<String>) -> Self {
        Self {
            dependency_graph: Default::default(),
            custom_tags: Default::default(),
        }
    }

    fn handle_custom_tag(&mut self, node: &JSXOpeningElement) {
        // 提取配置属性
        let config_attr = node.attributes.iter().find_map(|attr| {
            if let JSXAttributeItem::Attribute(attr) = attr {
                if attr.name == "config" {
                    return attr.value.as_ref().and_then(|v| serde_json::from_str(&v.to_string()).ok());
                }
            }
            None
        });

        // 构建组件节点
        let parent = self.current_component_node;
        let child_node = self.dependency_graph.add_node(ComponentNode {
            id: generate_component_id(),
            config: config_attr.unwrap_or_default(),
            code_span: node.span,
        });

        self.dependency_graph.add_edge(parent, child_node, DependencyEdge {
            relation_type: RelationType::CustomTag,
        });
    }
}

impl Visit<'_> for ComponentAnalyzer {
    fn visit_jsx_opening_element(&mut self, node: &JSXOpeningElement) {
        if let JSXElementName::Identifier(ident) = &node.name {
            if self.custom_tags.contains_key(&ident.name) {
                self.handle_custom_tag(node);
            }
        }

        visit::walk::walk_jsx_opening_element(self, node);
    }
}
