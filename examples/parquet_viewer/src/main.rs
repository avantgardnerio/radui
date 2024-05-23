use radui::app::App;

mod app_window;

fn main() {
    let win = crate::app_window::ParquetViewerWindow::new();

    App::run(win);
}
