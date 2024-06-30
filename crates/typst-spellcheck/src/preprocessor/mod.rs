pub mod merge_short;
pub mod paragraph;

use std::borrow::Cow;

use paragraph::Paragraph;
use typst_syntax::{SyntaxKind, SyntaxNode};

/// Preprocesses a typst syntax tree to remove and modify parts
/// that languagetool disagrees with.
///
/// Returns a set of paragraphs of text with their associated nodes.
///
/// Paragraphs are separated by `Parbreak` nodes.
///
/// `Raw`, `Equation`, and `FieldAccess` nodes are replaced with placeholder
/// text so lanugagetool doesn't flag them while still maintaining context.
///
/// `Hash`, `Label`, `ModuleImport`, `ModuleInclude`, `LineComment`, `BlockComment`, `Ident`, `Underscore`. `Star` nodes are ignored.
///
/// Inside a `FuncCall`, `ShowRule`, `SetRule`, `LetBinding` node, only the contents of
/// `Markdown` nodes are recorded.
pub fn preprocess(root: &SyntaxNode) -> Vec<Paragraph<'_>> {
    let (mut paragraphs, partial_paragraph) =
        recursively_build_paragraphs(root, Paragraph { nodes: vec![] }, false);

    if !partial_paragraph.nodes.is_empty() {
        paragraphs.push(partial_paragraph)
    }

    paragraphs
}

fn recursively_build_paragraphs<'a>(
    node: &'a SyntaxNode,
    current_paragraph: Paragraph<'a>,
    code_mode: bool,
) -> (Vec<Paragraph<'a>>, Paragraph<'a>) {
    let mut paragraphs = vec![];
    let mut current_paragraph = current_paragraph;
    let mut code_mode = code_mode;

    let node_kind = node.kind();

    // If in code mode, only a markup block will exit it.
    // All other nodes should be ignored in code mode.
    if code_mode {
        if node_kind == SyntaxKind::Markup {
            code_mode = false;
            /* if !current_paragraph.nodes.is_empty() {
                paragraphs.push(current_paragraph);
                current_paragraph = Paragraph { nodes: vec![] };
            } */
        }
    } else {
        match node_kind {
            // Terminate the paragraph
            SyntaxKind::Parbreak => {
                if !current_paragraph.nodes.is_empty() {
                    paragraphs.push(current_paragraph);
                    current_paragraph = Paragraph { nodes: vec![] };
                }
            }

            // If there are other nodes in the paragraph, to maintain context
            // a fake node is appended in place of the real node.
            SyntaxKind::Raw | SyntaxKind::Equation | SyntaxKind::FieldAccess => {
                if !current_paragraph.nodes.is_empty() {
                    let node_text = format!("`{}`", node_kind.name());
                    let mut fake_node = SyntaxNode::leaf(SyntaxKind::Text, node_text);
                    fake_node.synthesize(node.span());

                    current_paragraph.nodes.push(Cow::Owned(fake_node));
                }

                return (paragraphs, current_paragraph);
            }

            // Hash and label nodes are ignored
            SyntaxKind::Hash
            | SyntaxKind::Label
            | SyntaxKind::ModuleImport
            | SyntaxKind::ModuleInclude
            | SyntaxKind::LineComment
            | SyntaxKind::BlockComment
            | SyntaxKind::Ident
            | SyntaxKind::Underscore
            | SyntaxKind::Star => return (paragraphs, current_paragraph),

            // Toggle code mode for code nodes
            SyntaxKind::FuncCall
            | SyntaxKind::ShowRule
            | SyntaxKind::SetRule
            | SyntaxKind::LetBinding => code_mode = true,

            // To reduce load on languagetool, headings may be ignored
            // SyntaxKind::Heading => 'heading: {
            //     if !node.text().is_empty() {
            //         break 'heading;
            //     }

            //     if spellcheck_config.ignore_headings {
            //         return (paragraphs, current_paragraph);
            //     } else {
            //         current_paragraph.nodes.push(Cow::Borrowed(node));
            //     }
            // }

            // Space nodes should not be appended to empty paragraphs
            SyntaxKind::Space => 'space: {
                if node.text().is_empty() {
                    break 'space;
                }

                if current_paragraph.nodes.is_empty() {
                    return (vec![], current_paragraph);
                } else {
                    current_paragraph.nodes.push(Cow::Borrowed(node));
                }
            }

            // Other nodes should be recorded if they have text content
            _ => 'other: {
                if node.text().is_empty() {
                    break 'other;
                }

                current_paragraph.nodes.push(Cow::Borrowed(node));
            }
        }
    }

    // Go through this node's children and add their paragraphs
    for child in node.children() {
        let (mut child_groups, new_current_group) =
            recursively_build_paragraphs(child, current_paragraph, code_mode);

        if !child_groups.is_empty() {
            paragraphs.append(&mut child_groups);
        }

        current_paragraph = new_current_group;
    }

    (paragraphs, current_paragraph)
}
