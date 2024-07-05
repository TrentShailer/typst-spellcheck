use std::io::{stderr, BufWriter, Write};

use owo_colors::{OwoColorize, Style};
use typst_spellcheck::{problem::Problem, spellchecker::metadata::Metadata};

pub fn display_problems(
    file_path: &str,
    problems: Vec<Problem>,
    metadata: Metadata,
) -> Result<(), std::io::Error> {
    let handle = stderr().lock();
    let mut writer = BufWriter::new(handle);

    let emph = Style::new().yellow().bold();
    let sub = Style::new().bright_black().bold();

    for problem in problems.iter() {
        let title_sub = format!("{}: `{}`", problem.short_message, problem.match_string);
        writeln!(
            &mut writer,
            "{}{} {}",
            "Problem".style(emph),
            ":".style(sub),
            title_sub.style(sub)
        )?;

        writeln!(&mut writer, "{}, {}", file_path, problem.range)?;

        writeln!(&mut writer, "   |")?;
        writeln!(&mut writer, "   | {}", problem.context.as_str())?;
        writeln!(&mut writer, "   |")?;

        writeln!(
            &mut writer,
            "   {} {}: {}",
            "=".bold(),
            "Detail".style(sub),
            problem.message
        )?;
        writeln!(
            &mut writer,
            "   {} {}: {}",
            "=".bold(),
            "Category".style(sub),
            problem.rule_category
        )?;
        writeln!(
            &mut writer,
            "   {} {}: {}",
            "=".bold(),
            "Rule ID".style(sub),
            problem.rule_id
        )?;

        if !problem.replacements.is_empty() {
            writeln!(&mut writer, "{}:", "Did you mean".style(emph))?;

            for (index, replacement) in problem.replacements.iter().enumerate() {
                let number = format!("{}.", index + 1);

                writeln!(&mut writer, "   {} {}", number.style(sub), replacement)?;
            }
        }

        writeln!(&mut writer)?;
    }

    writeln!(
        &mut writer,
        "{}: processed {} chunks ({} words) and found {} problem(s) in {:.2}s",
        "Finished".green().bold(),
        metadata.paragraph_count.bold(),
        metadata.word_count,
        problems.len().bold(),
        metadata.languagetool_request_time.as_secs_f32().bold()
    )?;

    writer.flush()?;

    Ok(())
}
