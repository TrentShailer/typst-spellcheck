use std::borrow::Cow;

use typst_syntax::{SyntaxKind, SyntaxNode};

use super::paragraph::Paragraph;

pub fn merge_short(paragraphs: Vec<Paragraph>, min_length: usize) -> Vec<Paragraph> {
    let mut output: Vec<Paragraph> = vec![];
    let mut latest_is_short = false;

    for paragraph in paragraphs.to_owned().iter_mut() {
        if latest_is_short {
            let latest = output.last_mut().unwrap();

            let break_node = SyntaxNode::leaf(SyntaxKind::Space, "\r\n\r\n");

            latest.nodes.push(Cow::Owned(break_node));
            latest.nodes.append(&mut paragraph.nodes);

            if latest.get_text().0.len() >= min_length {
                latest_is_short = false;
            }

            continue;
        }

        if paragraph.get_text().0.len() < min_length {
            latest_is_short = true;
        }

        output.push(paragraph.to_owned());
    }

    output
}
