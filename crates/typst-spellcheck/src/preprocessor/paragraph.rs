use std::borrow::Cow;

use typst_syntax::{Span, SyntaxNode};

#[derive(Debug, Clone)]
pub struct Paragraph<'a> {
    pub nodes: Vec<Cow<'a, SyntaxNode>>,
}

#[derive(Debug, Clone)]
pub struct NodeContribution {
    pub span: Span,
    pub offset: usize,
    pub length: usize,
}

impl<'a> Paragraph<'a> {
    pub fn get_text(&self) -> (String, Vec<NodeContribution>) {
        let mut output = String::new();
        let mut node_contributions = vec![];

        for node in self.nodes.iter() {
            let text = node.text().as_str();
            let offset = output.len();
            let length = text.len();

            output.push_str(text);

            let node_contribution = NodeContribution {
                span: node.span(),
                offset,
                length,
            };

            node_contributions.push(node_contribution);
        }

        (output, node_contributions)
    }
}
