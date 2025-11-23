mod app;

use app::state::AppState;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Crystal Painter — Modular",
        options,
        Box::new(|_cc| Ok(Box::new(AppState::default()))),
    )?;

    Ok(())
}
