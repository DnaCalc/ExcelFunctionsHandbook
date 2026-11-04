mod checks;
mod markdown;
mod site;
mod util;

use std::path::PathBuf;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "build".into());
    if matches!(command.as_str(), "--help" | "-h" | "help") {
        println!("efh-site build [--repo PATH] [--out PATH]\nefh-site check|check-links|check-claims [--out PATH]\n\nDefault output: <repo>/site/build. Build checks output before replacing an earlier generated folder.\nNo C#, OxFunc checkout, network access, or browser is needed to build. No publication occurs.");
        return Ok(());
    }
    let mut repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut out = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--repo" => repo = PathBuf::from(args.next().ok_or("--repo needs a path")?),
            "--out" => out = Some(PathBuf::from(args.next().ok_or("--out needs a path")?)),
            _ => return Err(format!("unknown argument {arg:?}; use --help")),
        }
    }
    let repo = repo
        .canonicalize()
        .map_err(|e| format!("repository: {e}"))?;
    let out = out.unwrap_or_else(|| repo.join("site/build"));
    match command.as_str() {
        "build" => site::build(&repo, &out),
        "check" => checks::check(&out, true, true),
        "check-links" => checks::check(&out, true, false),
        "check-claims" => checks::check(&out, false, true),
        _ => Err(format!("unknown command {command:?}; use --help")),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("efh-site: {e}");
        std::process::exit(1);
    }
}
