use colored::Colorize;
use std::time::Duration;

fn log_logo() {
    let logo = "TRUTH-RS".green().bold();
    let version = "v0.0.1".dimmed();
    println!("\n  {logo} {version}\n");
}

pub fn log_url(url: &str, duration: Duration) {
    log_logo();
    let url = url.cyan().underline();
    println!(
        "  {}{}: {url}\t{} {}\n",
        "✓ ".green().bold(),
        "[Finished]".dimmed(),
        duration.as_millis().to_string().white().bold(),
        "ms".dimmed()
    );
}
