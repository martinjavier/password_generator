use eframe::egui;
use rand::Rng; // Para generar la contraseña

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 450.0])
            .with_transparent(false),

        renderer: eframe::Renderer::Glow,
        
        depth_buffer: 0,
        stencil_buffer: 0,
        multisampling: 0,

        ..Default::default()
    };
   
    eframe::run_native(
        "Generador de Contraseñas Seguras",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            // 👇 CAMBIO CLAVE: Retornamos el Box directamente sin Ok() ni cast
            Box::new(PasswordApp::default())
        }),
    )
}

// Definimos los idiomas disponibles
#[derive(PartialEq)]
enum Idioma { Es, En, Fr, De }

struct PasswordApp {
    longitud: f32,
    mayusculas: bool,
    minusculas: bool,
    numeros: bool,
    simbolos: bool,
    password_generada: String,
    idioma_actual: Idioma,
}

impl Default for PasswordApp {
    fn default() -> Self {
        Self {
            longitud: 32.0,
            mayusculas: true,
            minusculas: true,
            numeros: true,
            simbolos: false,
            password_generada: "".to_string(),
            idioma_actual: Idioma::Es,
        }
    }
}

impl eframe::App for PasswordApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut visuals = egui::Visuals::light();

        visuals.panel_fill = egui::Color32::WHITE;

        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {

        // --- SECCIÓN DE IDIOMA ---
        ui.horizontal(|ui| {
            // Reemplazamos egui::ImageButton::new(...) por egui::Button::image(...)
            
            // Botón para Español
            let image_es = egui::include_image!("español.png");
            if ui.add(egui::Button::image(image_es).selected(self.idioma_actual == Idioma::Es)).clicked() {
                self.idioma_actual = Idioma::Es;
            }

            // Botón para Inglés
            let image_en = egui::include_image!("inglés.png");
            if ui.add(egui::Button::image(image_en).selected(self.idioma_actual == Idioma::En)).clicked() {
                self.idioma_actual = Idioma::En;
            }

            // Botón para Francés
            let image_fr = egui::include_image!("francés.png");
            if ui.add(egui::Button::image(image_fr).selected(self.idioma_actual == Idioma::Fr)).clicked() {
                self.idioma_actual = Idioma::Fr;
            }

            // Botón para Alemán
            let image_de = egui::include_image!("alemán.png");
            if ui.add(egui::Button::image(image_de).selected(self.idioma_actual == Idioma::De)).clicked() {
                self.idioma_actual = Idioma::De;
            }
        });

            ui.separator();

            // --- TEXTOS SEGÚN IDIOMA ---
            let (titulo, txt_mayus, txt_minus, txt_num, txt_sim, btn_gen, btn_copy, txt_long) = match self.idioma_actual {
                Idioma::Es => ("Generador de Contraseñas", "Mayúsculas", "Minúsculas", "Números", "Símbolos", "GENERAR", "Copiar", "Longitud"),
                Idioma::En => ("Password Generator", "Uppercase", "Lowercase", "Numbers", "Symbols", "GENERATE", "Copy", "Length"),
                Idioma::Fr => ("Générateur de Mots de Passe", "Majuscules", "Minuscules", "Chiffres", "Symboles", "GÉNÉRER", "Copier", "Longueur"),
                Idioma::De => ("Passwort-Generator", "Großbuchstaben", "Kleinbuchstaben", "Zahlen", "Symbole", "GENERIEREN", "Kopieren", "Länge"),
            };

            ui.heading(egui::RichText::new(titulo).color(egui::Color32::BLACK).size(30.0));
            ui.add(egui::Slider::new(&mut self.longitud, 4.0..=128.0).text(txt_long));

            // --- CHECKBOXES ---
            ui.checkbox(&mut self.mayusculas, txt_mayus);
            ui.checkbox(&mut self.minusculas, txt_minus);
            ui.checkbox(&mut self.numeros, txt_num);
            ui.checkbox(&mut self.simbolos, txt_sim);

            ui.separator();

            egui::Frame::canvas(ui.style())
                .fill(egui::Color32::BLACK)
                .stroke(egui::Stroke::new(2.0, egui::Color32::BLUE))
                .inner_margin(5.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.password_generada)
                            .font(egui::TextStyle::Monospace)
                            .text_color(egui::Color32::WHITE)
                            .desired_rows(4)
                            .desired_width(f32::INFINITY)
                            .frame(false)
                    );
                });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button(btn_gen).clicked() {
                    self.password_generada = self.generar_password();
                }

                if ui.button(btn_copy).clicked() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        let _ = clipboard.set_text(self.password_generada.clone());
                    }
                }
            });
        });
    }
}

impl PasswordApp {
    fn generar_password(&self) -> String {
        let mut charset = String::new();
        if self.mayusculas { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
        if self.minusculas { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
        if self.numeros { charset.push_str("0123456789"); }
        if self.simbolos { charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?"); }

        if charset.is_empty() { return "Selecciona algo!".to_string(); }

        let mut rng = rand::thread_rng();
        (0..self.longitud as usize)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect()
    }
}