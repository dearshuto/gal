use std::fs::File;

use eframe::egui;

#[derive(Debug, PartialEq, Eq)]
enum Primitive {
    Teapot(usize, usize), // div, subdiv
    #[allow(unused)]
    Sphere,
    #[allow(unused)]
    Cube,
}

pub struct App {
    primitive: Primitive,
}

impl App {
    pub fn run() {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };
        eframe::run_native(
            "Girly",
            options,
            Box::new(|_cc| {
                // This gives us image support:
                // egui_extras::install_image_loaders(&cc.egui_ctx);

                Ok(Box::new(App::new()))
            }),
        )
        .unwrap()
    }

    fn new() -> Self {
        Self {
            primitive: Primitive::Teapot(1, 1),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Teapot");
            match &mut self.primitive {
                Primitive::Teapot(div, sub_div) => {
                    ui.add(egui::Slider::new(div, 1..=100).text("div"));
                    ui.add(egui::Slider::new(sub_div, 1..=100).text("suv_div"));
                    self.primitive = Primitive::Teapot(*div, *sub_div);
                }
                Primitive::Sphere => {}
                Primitive::Cube => {}
            }

            if ui.button("Save").clicked() {
                let (mesh, file_name) = match self.primitive {
                    Primitive::Teapot(div, sub_div) => (
                        gal::prim::Builder::teapot()
                            .with_params(|_| (div, sub_div))
                            .triangulated(),
                        "teapot.obj",
                    ),
                    Primitive::Sphere => {
                        (gal::prim::Builder::sphere().triangulated(), "sphere.pbj")
                    }
                    Primitive::Cube => (gal::prim::Builder::cube().triangulated(), "cube.obj"),
                };
                let file = File::create(file_name).unwrap();
                gal::serialize::WavefrontObj::serialize(file, &mesh).unwrap();
            }
        });
    }
}
