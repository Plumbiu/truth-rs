use colored::Colorize;

fn log_logo() {
    let logo = "TRUTH-RS".green().bold();
    let version = "v0.0.1".dimmed();
    println!("\n  {logo} {version}\n");
}

pub fn log_url(url: &str, duration: u128) {
    log_logo();
    let url = url.cyan().underline();
    println!(
        "  {}{}: {url}\t{} {}\n",
        "✓ ".green().bold(),
        "[Finished]".dimmed(),
        duration,
        "ms".dimmed()
    );
}
