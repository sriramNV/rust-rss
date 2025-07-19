use eframe::egui::{self, CentralPanel, Context, FontFamily, FontId, TextStyle, TopBottomPanel};

#[derive(Default)]
struct App{
    subs:Vec<(String, String)>,
    name_input: String,
    url_input: String,
}

impl eframe::App for App{
    fn update(
            &mut self, 
            ctx: &eframe::egui::Context,
            _frame: &mut eframe::Frame,
    ) {
        set_styles(ctx);
        show_top_bar(ctx);     
        CentralPanel::default().show(ctx, |ui|{
            ui.collapsing("New RSS", |ui|{
                ui.vertical_centered_justified(|ui|{
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.name_input);
                    ui.label("RSS URL");
                    ui.text_edit_singleline(&mut self.url_input);
                    ui.horizontal(|ui|{
                        if ui.button("Submit").clicked(){
                            self.subs.push((
                                self.name_input.clone(),
                                self.url_input.clone(), 
                            ));
                            self.name_input.clear();
                            self.url_input.clear();
                        }
                        if ui.button("Clear").clicked(){
                            self.name_input.clear();
                            self.url_input.clear();
                        }
                    });
                });
            });

            for sub in &self.subs{
                ui.heading(&sub.0);
                ui.heading(&sub.1);
            }
        });
        
    }
}

fn show_top_bar(ctx: &Context) {
    TopBottomPanel::top("menu_bar").show(ctx, |ui|{
                egui::menu::bar(ui, |ui|{
                    ui.menu_button("File", |ui|{
                        if ui.button("Exit").clicked(){
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            }); 
}


fn main()  -> Result<(), eframe::Error> {
    println!("Hello, world!");
    let options = eframe::NativeOptions{
        viewport: eframe::egui::ViewportBuilder::default().with_resizable(true).with_inner_size([320.0,240.0]),
        ..Default::default()
    };
    eframe::run_native("Rustdev", options, Box::new(|_cc|Ok(Box::<App>::default())))
}

fn set_styles(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, FontFamily::Monospace)),
        (TextStyle::Body, FontId::new(18.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(20.0, FontFamily::Monospace)),
        (TextStyle::Small, FontId::new(16.0, FontFamily::Monospace)),
    ].into();

    ctx.set_style(style)
}