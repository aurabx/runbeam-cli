use clap::CommandFactory;

/// List available commands (from clap) in a table
pub fn list_commands() -> anyhow::Result<()> {
    let cmd = crate::cli::Cli::command();
    let subs: Vec<_> = cmd.get_subcommands().collect();

    if subs.is_empty() {
        println!("No commands available.");
        return Ok(());
    }

    // Compute widths
    let mut w_name = "NAME".len();
    let mut w_desc = "DESCRIPTION".len();
    for sc in &subs {
        let name = sc.get_name();
        if name.len() > w_name {
            w_name = name.len();
        }
        let about = sc.get_about().map(|s| s.to_string()).unwrap_or_default();
        if about.len() > w_desc {
            w_desc = about.len();
        }
    }

    // Header
    println!(
        "{name:<name_w$} | {desc:<desc_w$}",
        name = "NAME",
        desc = "DESCRIPTION",
        name_w = w_name,
        desc_w = w_desc,
    );
    // Separator
    println!(
        "{n:-<name_w$}-+-{d:-<desc_w$}",
        n = "",
        d = "",
        name_w = w_name,
        desc_w = w_desc,
    );

    // Rows
    for sc in subs {
        let name = sc.get_name();
        let about = sc.get_about().map(|s| s.to_string()).unwrap_or_default();
        println!(
            "{name:<name_w$} | {desc:<desc_w$}",
            name = name,
            desc = about,
            name_w = w_name,
            desc_w = w_desc,
        );
    }

    Ok(())
}
