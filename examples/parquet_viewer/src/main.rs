use radui::app::App;

mod app_window;

fn main() {
    let win = crate::app_window::AppWindow::new();

    App::run(win);
}
