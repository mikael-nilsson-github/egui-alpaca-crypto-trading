use std::sync::{Arc, Mutex};

use egui::plot::{Line, Plot};
use egui::Color32;

pub struct App {
    pub tick: Arc<Mutex<f64>>,
    pub security1: Arc<Mutex<f64>>,
    pub security2: Arc<Mutex<f64>>,
    pub security3: Arc<Mutex<f64>>,
    pub security4: Arc<Mutex<f64>>,
    pub security5: Arc<Mutex<f64>>,
    pub security6: Arc<Mutex<f64>>,
    pub data_security1: Arc<Mutex<Vec<[f64; 2]>>>,
    pub data_security2: Arc<Mutex<Vec<[f64; 2]>>>,
    pub data_security3: Arc<Mutex<Vec<[f64; 2]>>>,
    pub data_security4: Arc<Mutex<Vec<[f64; 2]>>>,
    pub data_security5: Arc<Mutex<Vec<[f64; 2]>>>,
    pub data_security6: Arc<Mutex<Vec<[f64; 2]>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            tick: Arc::new(Mutex::new(0.0)),
            security1: Arc::new(Mutex::new(0.0)),
            security2: Arc::new(Mutex::new(0.0)),
            security3: Arc::new(Mutex::new(0.0)),
            security4: Arc::new(Mutex::new(0.0)),
            security5: Arc::new(Mutex::new(0.0)),
            security6: Arc::new(Mutex::new(0.0)),
            data_security1: Arc::new(Mutex::new(Vec::new())),
            data_security2: Arc::new(Mutex::new(Vec::new())),
            data_security3: Arc::new(Mutex::new(Vec::new())),
            data_security4: Arc::new(Mutex::new(Vec::new())),
            data_security5: Arc::new(Mutex::new(Vec::new())),
            data_security6: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
            //println!("{}", self.tick.lock().unwrap());
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Administration", |ui| {
                    if ui.button("Log Message").clicked() {
                        println!("> Log Message.");
                    }
                    if ui.button("Run Loop").clicked() {
                        for i in 0..10 {
                            println!("> Run Loop {}.", i);
                        }
                    }
                    if ui.button("Quit").clicked() {
                        println!("> Application Quit...");
                        _frame.close();
                    }
                });
                ui.menu_button("Trading", |ui| {
                    if ui.button("Equities").clicked() {
                        println!("> Equities.");
                    }
                    if ui.button("Crypto").clicked() {
                        println!("> Crypto.");
                    }
                    if ui.button("Forex").clicked() {
                        println!("> Forex.");
                    }
                });
                ui.menu_button("Charts", |ui| {
                    if ui.button("Real Time").clicked() {
                        println!("> Real Time.");
                    }
                    if ui.button("Historical").clicked() {
                        println!("> Historical.");
                    }
                });
                ui.menu_button("Portfolios", |ui| {
                    if ui.button("Archimedes I").clicked() {
                        println!("> Archimedes I.");
                    }
                    if ui.button("Crypto IV").clicked() {
                        println!("> Crypto IV.");
                    }
                    if ui.button("Euler XI").clicked() {
                        println!("> Euler IV.");
                    }
                });
                ui.menu_button("Machine Learning", |ui| {
                    ui.menu_button("Supervised Learning", |ui| {
                        if ui.button("Linear Regression").clicked() {
                            println!("> Linear Regression.");
                        }
                    });
                    ui.menu_button("Unsupervised Learning", |ui| {
                        if ui.button("K-Nearest Neighbors").clicked() {
                            println!("> K-Nearest Neighbors.");
                        }
                    });
                    ui.menu_button("Reinforcement Learning", |ui| {
                        if ui.button("Q-Learning").clicked() {
                            println!("> Q-Learning.");
                        }
                    });
                });
                ui.menu_button("AI", |ui| {
                    ui.menu_button("Games", |ui| {
                        if ui.button("Tic-Tac-Toe").clicked() {
                            println!("> Tic-Tac-Toe.");
                        }
                        if ui.button("Kuhn Poker").clicked() {
                            println!("> Kuhn Poker.");
                        }
                        if ui.button("21").clicked() {
                            println!("> 21.");
                        }
                    });
                    ui.menu_button("GridWorld", |ui| {
                        if ui.button("5x5").clicked() {
                            println!("> 5x5.");
                        }
                    });
                    ui.menu_button("Labyrinth", |ui| {
                        if ui.button("9x9").clicked() {
                            println!("> 9x9.");
                        }
                    });
                });
                ui.menu_button("Risk Management", |ui| {
                    ui.menu_button("Value-at-Risk", |ui| {
                        if ui.button("Parametric").clicked() {
                            println!("> Parametric.");
                        }
                        if ui.button("Historical").clicked() {
                            println!("> Historical.");
                        }
                        if ui.button("Monte Carlo Simulation").clicked() {
                            println!("> Monte Carlo Simulation.");
                        }
                    });
                    ui.menu_button("Scenario Analysis", |ui| {
                        if ui.button("1950 - 2022").clicked() {
                            println!("> 1950 - 2022.");
                        }
                    });
                    ui.menu_button("Stress Test", |ui| {
                        if ui.button("50 Basis Points").clicked() {
                            println!("> 50 Basis Points.");
                        }
                    });
                });
            });
        });

        egui::SidePanel::left("side_panel_left").show(ctx, |ui| {
            ui.heading("C R Y P T O");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.label("2022, All Rights Reserved.");
                });
            });
        });

        egui::Window::new("Portfolio").min_height(1600.0).min_width(300.0).show(ctx, |ui| {
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.label("Symbol");
                ui.label("Price");
                ui.label("Quantity");
                ui.label("Market Value");
                ui.label("Profit / Loss");
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.separator();
                ui.separator();
                ui.separator();
                ui.end_row();

                ui.label("AAPL");
                ui.label("134.33");
                ui.label("225.00");
                ui.label("100,000");
                ui.label("+352,78");
                ui.end_row();

                ui.label("META");
                ui.label("120.10");
                ui.label("405.00");
                ui.label("550,000");
                ui.label("-33,833.33");
                ui.end_row();

                ui.label("MSFT");
                ui.label("245.23");
                ui.label("100.00");
                ui.label("300,000");
                ui.label("+12,322,78");
                ui.end_row();

                ui.label("NVDA");
                ui.label("119.00");
                ui.label("50.00");
                ui.label("75,000");
                ui.label("+3,432,46");
                ui.end_row();

                ui.label("TSLA");
                ui.label("149.20");
                ui.label("2000.00");
                ui.label("375,000");
                ui.label("-443,454,94");
                ui.end_row();
            })
        });

        egui::Window::new("Trade Order").min_height(1600.0).min_width(400.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Symbol: ");
                let mut text = "MSFT".to_string();
                ui.add(egui::TextEdit::singleline(&mut text));
            });
            ui.horizontal(|ui| {
                ui.label("Price: ");
                let mut text = "244.55".to_string();
                ui.add(egui::TextEdit::singleline(&mut text));
            });
            ui.horizontal(|ui| {
                ui.label("Type: ");
                let mut text = "Limit".to_string();
                ui.add(egui::TextEdit::singleline(&mut text));
            });
            ui.horizontal(|ui| {
                ui.label("Time: ");
                let mut text = "GTC - Good till Canceled".to_string();
                ui.add(egui::TextEdit::singleline(&mut text));
            });
            ui.horizontal(|ui| {
                ui.label("Quantity: ");
                let mut text = "125.0".to_string();
                ui.add(egui::TextEdit::singleline(&mut text));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.add(egui::Button::new("BUY"));
                ui.add(egui::Button::new("SELL"));
            });
        });

        egui::Window::new("Live Chart").show(ctx, |ui| {
            ui.label(format!("BTC/USD = {}", self.security6.lock().unwrap()));
            let line6 = Line::new(self.data_security6.lock().unwrap().clone())
                .color(Color32::from_rgb(200, 122, 65));
            let plot6 = Plot::new("chart6").view_aspect(1.6);
            plot6.show(ui, |plot_ui| {
                plot_ui.line(line6)
            });
        });

        egui::SidePanel::right("side_panel_right").min_width(350.0).show(ctx, |ui| {
            ui.label(format!("Tick -> {}s", self.tick.lock().unwrap()));

            ui.separator();
            ui.label(format!("ETH/USD = {}", self.security1.lock().unwrap()));
            let line1 = Line::new(self.data_security1.lock().unwrap().clone())
                .color(Color32::from_rgb(255, 255, 0));
            let plot1 = Plot::new("chart1").view_aspect(4.0);
            plot1.show(ui, |plot_ui| {
                plot_ui.line(line1)
            });

            ui.separator();
            ui.label(format!("SOL/USD = {}", self.security2.lock().unwrap()));
            let line2 = Line::new(self.data_security2.lock().unwrap().clone())
                .color(Color32::from_rgb(0, 0, 255));
            let plot2 = Plot::new("chart2").view_aspect(4.0);
            plot2.show(ui, |plot_ui| {
                plot_ui.line(line2)
            });

            ui.separator();
            ui.label(format!("LINK/USD = {}", self.security3.lock().unwrap()));
            let line3 = Line::new(self.data_security3.lock().unwrap().clone())
                .color(Color32::from_rgb(0, 255, 0));
            let plot3 = Plot::new("chart3").view_aspect(4.0);
            plot3.show(ui, |plot_ui| {
                plot_ui.line(line3)
            });

            ui.separator();
            ui.label(format!("LTC/USD = {}", self.security4.lock().unwrap()));
            let line4 = Line::new(self.data_security4.lock().unwrap().clone())
                .color(Color32::from_rgb(177, 0, 177));
            let plot4 = Plot::new("chart4").view_aspect(4.0);
            plot4.show(ui, |plot_ui| {
                plot_ui.line(line4)
            });

            ui.separator();
            ui.label(format!("MATIC/USD = {}", self.security5.lock().unwrap()));
            let line5 = Line::new(self.data_security5.lock().unwrap().clone())
                .color(Color32::from_rgb(83, 55, 120));
            let plot5 = Plot::new("chart5").view_aspect(4.0);
            plot5.show(ui, |plot_ui| {
                plot_ui.line(line5)
            });
        });
    }
}
