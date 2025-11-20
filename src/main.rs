use eframe::egui;

#[derive(Default)]
struct MyApp {
    show_window_a: bool,
    show_window_b: bool,
    counter: i32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top bar with toggles
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Toggle Window A").clicked() {
                    self.show_window_a = !self.show_window_a;
                }
                if ui.button("Toggle Window B").clicked() {
                    self.show_window_b = !self.show_window_b;
                }
            });
        });

        // Window A
        egui::Window::new("Window A")
            .open(&mut self.show_window_a)
            .show(ctx, |ui| {
                ui.label("This is Window A.");
                if ui.button("Increment counter").clicked() {
                    self.counter += 1;
                }
                ui.label(format!("Counter: {}", self.counter));
            });

        // Window B
        egui::Window::new("Window B")
            .open(&mut self.show_window_b)
            .show(ctx, |ui| {
                ui.label("This is Window B.");
                if ui.button("Reset counter").clicked() {
                    self.counter = 0;
                }
                ui.label(format!("Counter: {}", self.counter));
            });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui windows example");
            ui.label("Use the top buttons to open/close windows.");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Windows Example",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}