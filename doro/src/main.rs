mod app;

fn main() {
    let mut app =  app::App::new();
    app.ui.add(ui_elements::BackgroundSquare::default()
        .set_x(5)
        .set_y(10)
        .set_width(10)
        .set_height(5)
        .set_color(ui::Background::Red)
    );
    app.start()
}