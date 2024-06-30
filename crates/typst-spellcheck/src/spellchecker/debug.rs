use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
};

use languagetool_rust::CheckResponse;
use typst_syntax::SyntaxNode;

use crate::{
    preprocessor::paragraph::{NodeContribution, Paragraph},
    problem::Problem,
};

const FILE_NAME: &str = "typst-spellcheck.debug.log";

pub fn setup_debug_file() {
    let _file = File::create(FILE_NAME).unwrap();
}

pub fn debug_syntax_tree(root: &SyntaxNode) {
    let file = OpenOptions::new().append(true).open(FILE_NAME).unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "---- Syntax Tree ----").unwrap();

    writeln!(&mut writer, "{:#?}", root).unwrap();

    writeln!(&mut writer).unwrap();
    writer.flush().unwrap();
}

pub fn debug_paragraphs(paragraphs: &[Paragraph]) {
    let file = OpenOptions::new().append(true).open(FILE_NAME).unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "---- Paragraphs ----").unwrap();

    writeln!(&mut writer, "{:#?}", paragraphs).unwrap();

    writeln!(&mut writer).unwrap();
    writer.flush().unwrap();
}

pub fn debug_response(
    response: &CheckResponse,
    paragraph: &Paragraph,
    text: &str,
    node_contributions: &[NodeContribution],
) {
    let file = OpenOptions::new().append(true).open(FILE_NAME).unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "---- Response ----").unwrap();

    writeln!(&mut writer, "Response:\n{:#?}", response).unwrap();
    writeln!(&mut writer, "Paragraph:\n{:#?}", paragraph).unwrap();
    writeln!(&mut writer, "Text:\n{}", text).unwrap();
    writeln!(&mut writer, "Contributions:\n{:#?}", node_contributions).unwrap();

    writeln!(&mut writer).unwrap();
    writer.flush().unwrap();
}

pub fn debug_problems(problems: &[Problem]) {
    let file = OpenOptions::new().append(true).open(FILE_NAME).unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(&mut writer, "---- Problems ----").unwrap();

    writeln!(&mut writer, "{:#?}", problems).unwrap();

    writeln!(&mut writer).unwrap();
    writer.flush().unwrap();
}
