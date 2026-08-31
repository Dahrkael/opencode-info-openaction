mod actions;
mod render;
mod settings;
mod state;
mod usage;

use openaction::{register_action, run};

#[tokio::main]
async fn main() -> openaction::OpenActionResult<()> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(pos) = args.iter().position(|a| a == "--api-key") {
        match args.get(pos + 1) {
            Some(key) if !key.is_empty() => cli_run(key).await,
            _ => eprintln!("error: --api-key requires a non-empty value"),
        }
        return Ok(());
    }

    if let Some(pos) = args.iter().position(|a| a == "--debug-png") {
        let dir = args.get(pos + 1).map(|s| s.as_str()).unwrap_or(".");
        debug_png(dir);
        return Ok(());
    }

    if !args.iter().any(|a| a == "-port") {
        println!("opencode-info (CLI mode)");
        println!("No OpenDeck arguments detected.");
        println!("  opencode-info --api-key <KEY>   -> print Go usage (5h/week/month)");
        println!("  opencode-info --debug-png <dir> -> render sample PNGs for inspection");
        println!("  launched by OpenDeck            -> plugin mode");
        return Ok(());
    }

    register_action(actions::WindowAction::default()).await;
    register_action(actions::RotateAction::default()).await;
    register_action(actions::SummaryAction::default()).await;

    run(args).await
}

async fn cli_run(api_key: &str) {
    match usage::fetch_usage(api_key).await {
        Ok(usage) => {
            println!("OpenCode Go usage:");
            println!(
                "  5h    : {:>3}%",
                usage.rolling.as_ref().map(|w| w.percent).unwrap_or(0)
            );
            println!(
                "  week  : {:>3}%",
                usage.weekly.as_ref().map(|w| w.percent).unwrap_or(0)
            );
            println!(
                "  month : {:>3}%",
                usage.monthly.as_ref().map(|w| w.percent).unwrap_or(0)
            );
        }
        Err(e) => eprintln!("error fetching usage: {e}"),
    }
}

fn debug_png(dir: &str) {
    use render::Layout;
    std::fs::create_dir_all(dir).expect("create dir");

    // Single windows at three representative percentages
    for (window, name) in [
        ("5h", "single_5h"),
        ("week", "single_week"),
        ("month", "single_month"),
    ] {
        for pct in [0u8, 53, 100] {
            let bytes =
                render::render_png_bytes(&Layout::single(pct, window), (255, 255, 255), 50, 90);
            let path = format!("{}/{}_{}.png", dir, name, pct);
            std::fs::write(&path, &bytes).expect("write png");
            println!("wrote {path}");
        }
    }

    // Summary with all three present
    let bytes = render::render_png_bytes(
        &Layout::summary(Some(12), Some(53), Some(100)),
        (255, 255, 255),
        50,
        90,
    );
    let path = format!("{}/summary.png", dir);
    std::fs::write(&path, &bytes).expect("write png");
    println!("wrote {path}");

    // Tinting probe written to a clearly-named sample; default font colour stays white.
    let bytes = render::render_png_bytes(&Layout::single(77, "week"), (255, 120, 0), 50, 90);
    let path = format!("{}/tint_sample_orange.png", dir);
    std::fs::write(&path, &bytes).expect("write png");
    println!("wrote {path}");
}
