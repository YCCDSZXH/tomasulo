use core::comp::{rs::Slot, Tomasulo};

use anyhow::Result;
use egui::{Color32, Context, RichText, Window};
use egui_extras::{Column, TableBody, TableBuilder};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    instructions: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: i32,
    #[serde(skip)] // This how you opt-out of serialization of a field
    tomasulo: Tomasulo,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            instructions: r#"lw x1 1 x0
mul x1 x1 x2
add x3 x1 x2
add x4 x4 x5
add x9 x4 x10
add x5 x7 x8
add x5 x9 x1
"#
            .to_owned(),
            value: 0,
            tomasulo: Tomasulo::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        // file.read_to_string(&mut contents)?;
        // let tomasulo = Tomasulo::default();
        // tomasulo.init_instruction(&contents)?;
        // tomasulo.run_to(10);

        Default::default()
    }
    fn instruction(&mut self, ctx: &Context) {
        Window::new("Instructions")
            .open(&mut true)
            .vscroll(false)
            .resizable(true)
            .title_bar(false)
            .default_size([300.0, 350.0])
            .show(ctx, |ui| {
                ui.label("Instructions");
                egui::TextEdit::multiline(&mut self.instructions)
                    .hint_text("Type something!")
                    .show(ui);

                let table = TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .min_scrolled_height(0.0);
                table
                    // .header(20.0, |mut header| {
                    //     header.col(|ui| {
                    //         ui.strong("Reg Index");
                    //     });
                    //     header.col(|ui| {
                    //         ui.strong("Value");
                    //     });
                    //     header.col(|ui| {
                    //         ui.strong("Status");
                    //     });
                    // })
                    .body(|mut body| {
                        let pc = core::comp::pc::PC.read().unwrap();
                        pc.instrutions.iter().enumerate().for_each(|(i, v)| {
                            if i as u32 == pc.index {
                                body.row(18.0, |mut row| {
                                    let v = v.to_tuple();
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(v.0)
                                                .color(Color32::from_rgb(110, 255, 110)),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(v.1)
                                                .color(Color32::from_rgb(110, 255, 110)),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(v.2)
                                                .color(Color32::from_rgb(110, 255, 110)),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(v.3)
                                                .color(Color32::from_rgb(110, 255, 110)),
                                        );
                                    });
                                });
                            } else {
                                body.row(18.0, |mut row| {
                                    let v = v.to_tuple();
                                    row.col(|ui| {
                                        ui.label(&v.0);
                                    });
                                    row.col(|ui| {
                                        ui.label(&v.1);
                                    });
                                    row.col(|ui| {
                                        ui.label(&v.2);
                                    });
                                    row.col(|ui| {
                                        ui.label(&v.3);
                                    });
                                });
                            }
                        });
                    })
            });
    }
    fn run(&mut self) -> Result<()> {
        self.tomasulo.init_instruction(&self.instructions)?;
        self.tomasulo.run_to(self.value);
        Ok(())
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //     // The central panel the region left after adding TopPanel's and SidePanel's
            //     ui.heading("eframe template");

            // ui.horizontal(|ui| {
            //     ui.label("current step");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            self.instruction(ctx);
            ui.horizontal(|ui| {
                if ui.button("prev").clicked() && self.value > 0 {
                    self.value -= 1;
                    let _ = self.run();
                }
                ui.label(self.value.to_string());
                if ui.button("next").clicked() {
                    self.value += 1;
                    let _ = self.run();
                }
            });

            //     // ui.separator();

            //     ui.add(egui::github_link_file!(
            //         "https://github.com/emilk/eframe_template/blob/master/",
            //         "Source code."
            //     ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
            //     new_windows(ctx);
        });
        rs(ctx);
        regs(ctx);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

// fn new_windows(ctx: &Context) {
//     Window::new("demo")
//         .open(&mut true)
//         .vscroll(false)
//         .resizable(false)
//         .default_size([300.0, 350.0])
//         .show(ctx, |ui| ui.label("Powered by "));
// }

fn rs(ctx: &Context) {
    Window::new("Reservation station")
        .open(&mut true)
        .title_bar(false)
        .vscroll(false)
        .resizable(false)
        .default_size([300.0, 350.0])
        .show(ctx, |ui| {
            ui.label("Reservation station");

            let table = TableBuilder::new(ui)
                // .striped(self.striped)
                // .resizable(self.resizable)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                // .column(Column::initial(100.0).range(40.0..=300.0))
                // .column(Column::initial(100.0).at_least(40.0).clip(true))
                // .column(Column::remainder())
                .min_scrolled_height(0.0);
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Busy");
                    });
                    header.col(|ui| {
                        ui.strong("Time");
                    });
                    header.col(|ui| {
                        ui.strong("Addr");
                    });
                    header.col(|ui| {
                        ui.strong("Op");
                    });
                    header.col(|ui| {
                        ui.strong("Vj");
                    });
                    header.col(|ui| {
                        ui.strong("Vk");
                    });
                    header.col(|ui| {
                        ui.strong("Qj");
                    });
                    header.col(|ui| {
                        ui.strong("Qk");
                    });
                })
                .body(|mut body| {
                    let rs = core::comp::rs::RS.read().unwrap();
                    display(&mut body, &rs.add, "add".to_owned());
                    display(&mut body, &rs.mul, "mul".to_owned());
                    display(&mut body, &rs.load, "load".to_owned());
                    display(&mut body, &rs.store, "store".to_owned());
                })
        });
}
fn regs(ctx: &Context) {
    Window::new("Reg Group")
        .open(&mut true)
        .title_bar(false)
        .vscroll(false)
        .resizable(false)
        .default_size([700.0, 750.0])
        .show(ctx, |ui| {
            ui.label("Reg Group");

            let table = TableBuilder::new(ui)
                // .striped(self.striped)
                // .resizable(self.resizable)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .min_scrolled_height(0.0);
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Reg Index");
                    });
                    header.col(|ui| {
                        ui.strong("Value");
                    });
                    header.col(|ui| {
                        ui.strong("Status");
                    });
                })
                .body(|mut body| {
                    let rg = core::comp::reg::REG_GROUP.read().unwrap();
                    rg.regs.iter().enumerate().for_each(|(i, v)| {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label("x".to_owned() + &i.to_string());
                            });
                            row.col(|ui| {
                                ui.label(v.value.to_string());
                            });
                            row.col(|ui| {
                                if let Some(state) = v.state {
                                    ui.label(format!("{}{}", state.0, state.1))
                                } else {
                                    ui.label("")
                                };
                            });
                        });
                    });
                })
        });
}

fn display(body: &mut TableBody<'_>, rows: &[Slot], label: String) {
    rows.iter().enumerate().for_each(|(i, v)| {
        body.row(18.0, |mut row| {
            // row.set_selected(self.selection.contains(&row_index));

            row.col(|ui| {
                ui.label(label.clone() + &i.to_string());
            });
            row.col(|ui| {
                ui.label(v.busy.to_string());
            });
            row.col(|ui| {
                ui.label(v.time.to_string());
            });
            row.col(|ui| {
                if let Some(addr) = v.addr {
                    ui.label(addr.to_string());
                } else {
                    ui.label("");
                }
            });
            row.col(|ui| {
                if let Some(op) = v.op {
                    ui.label(op.to_string());
                } else {
                    ui.label("");
                }
            });
            row.col(|ui| {
                if let Some(vj) = v.vj {
                    ui.label(vj.to_string());
                } else {
                    ui.label("");
                }
            });
            row.col(|ui| {
                if let Some(vk) = v.vk {
                    ui.label(vk.to_string());
                } else {
                    ui.label("");
                }
            });
            row.col(|ui| {
                if let Some(qj) = v.qj {
                    ui.label(format!("{}{}", qj.0, qj.1));
                } else {
                    ui.label("");
                }
            });
            row.col(|ui| {
                if let Some(qk) = v.qk {
                    ui.label(format!("{}{}", qk.0, qk.1));
                } else {
                    ui.label("");
                }
            });
        });
    });
}
