#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    _dummy: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self { _dummy: false }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("egui_table inside ScrollArea bug repro");
            ui.separator();

            // Outer ScrollArea — the "canvas" that is larger than the viewport.
            // The table sits inside this and should move when the canvas scrolls.
            egui::ScrollArea::both()
                .id_salt("outer_canvas")
                .show(ui, |ui| {
                    // Force the canvas to be larger than the viewport so it scrolls.
                    ui.set_min_size(egui::vec2(1200.0, 1000.0));

                    ui.label("Content above the table");
                    ui.add_space(40.0);

                    // Constrain the table so it doesn't fill the entire canvas.
                    let table_max_size = egui::vec2(500.0, 250.0);
                    egui::Frame::new()
                        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                        .corner_radius(4.0)
                        .inner_margin(4.0)
                        .show(ui, |ui| {
                            ui.allocate_ui(table_max_size, |ui| {
                                let mut delegate = DemoTableDelegate;
                                const NUM_ROWS: u64 = 20;
                                const NUM_COLS: usize = 10;

                                egui_table::Table::new()
                                    .id_salt("demo_table")
                                    .num_rows(NUM_ROWS)
                                    .columns(vec![
                                        egui_table::Column::new(100.0)
                                            .range(50.0..=300.0)
                                            .resizable(true);
                                        NUM_COLS
                                    ])
                                    .num_sticky_cols(1)
                                    .headers([egui_table::HeaderRow::new(24.0)])
                                    .auto_size_mode(egui_table::AutoSizeMode::OnParentResize)
                                    .show(ui, &mut delegate);
                            });
                        });

                    ui.add_space(40.0);
                    ui.label("Content below the table");
                });
        });
    }
}

struct DemoTableDelegate;

impl egui_table::TableDelegate for DemoTableDelegate {
    fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::HeaderCellInfo) {
        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(4, 0))
            .show(ui, |ui| {
                ui.strong(format!("Col {}", cell.col_range.start));
            });
    }

    fn cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::CellInfo) {
        if cell.row_nr % 2 == 1 {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }

        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(4, 0))
            .show(ui, |ui| {
                ui.label(format!("R{} C{}", cell.row_nr, cell.col_nr));
            });
    }
}
