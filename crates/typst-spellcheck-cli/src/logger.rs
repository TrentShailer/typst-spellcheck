use log::LevelFilter;

pub fn init_fern(debug: bool) -> Result<(), fern::InitError> {
    let log_level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let message = message.to_string();
            let level = record.level();
            let target = record.target();

            out.finish(format_args!("[{level}] [{target}]\n{message}\n",))
        })
        .level(LevelFilter::Warn)
        .level_for("typst_spellcheck_cli", log_level)
        .level_for("typst_spellcheck", log_level)
        .chain(std::io::stderr())
        .apply()?;
    Ok(())
}
