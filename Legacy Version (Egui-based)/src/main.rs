// File: src/main.rs

use eframe::{egui, App, CreationContext};
use egui::{FontData, FontDefinitions, FontFamily};
use reqwest;
use scraper::{Html, Selector};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use url::Url;

struct GhaybApp {
    url: String,
    tabs: HashMap<String, (String, Vec<String>, Vec<String>)>,
    active_tab: Option<String>,
    rx: Option<mpsc::Receiver<(String, String)>>,
}

impl Default for GhaybApp {
    fn default() -> Self {
        Self {
            url: String::from("https://example.com"),
            tabs: HashMap::new(),
            active_tab: None,
            rx: None,
        }
    }
}

impl App for GhaybApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Font setup
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "FiraCode".to_owned(),
            FontData::from_static(include_bytes!("../assets/FiraCode-Regular.ttf")).into(),
        );
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "FiraCode".to_owned());
        ctx.set_fonts(fonts);

        // Dark-green theme
        let mut visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(egui::Color32::GREEN);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::BLACK;
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(20,20,20);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(40,40,40);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60,60,60);
        visuals.selection.bg_fill = egui::Color32::from_rgb(0,80,0);
        visuals.selection.stroke.color = egui::Color32::GREEN;
        ctx.set_visuals(visuals);

        // UI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("Ghayb Browser")
                        .color(egui::Color32::GREEN)
                        .font(egui::FontId::proportional(30.0))
                        .heading(),
                );
                ui.add_space(20.0);

                // URL input & validation
                let valid = Url::parse(&self.url).is_ok();
                ui.vertical_centered(|ui| {
                    ui.add_sized(
                        [450.0, 45.0],
                        egui::TextEdit::singleline(&mut self.url)
                            .hint_text("Enter URL")
                            .text_color(egui::Color32::GREEN)
                            .font(egui::FontId::monospace(22.0))
                            .background_color(egui::Color32::from_rgb(10,10,10)),
                    );
                    ui.colored_label(
                        if valid { egui::Color32::LIGHT_GREEN } else { egui::Color32::RED },
                        if valid { "URL OK" } else { "Invalid URL" },
                    );
                });

                ui.add_space(10.0);

                // Go button
                ui.vertical_centered(|ui| {
                    if ui.add_enabled(
                        valid,
                        egui::Button::new(
                            egui::RichText::new("🔍 Go")
                                .color(egui::Color32::BLACK)
                                .font(egui::FontId::monospace(24.0)),
                        )
                        .fill(if valid { egui::Color32::LIGHT_GREEN } else { egui::Color32::DARK_GRAY })
                        .stroke(egui::Stroke::new(1.5, egui::Color32::WHITE))
                        .min_size(egui::vec2(120.0, 50.0))
                        .rounding(10.0),
                    ).clicked()
                    {
                        let url = enforce_https(&self.url);
                        let (tx, rx) = mpsc::channel();
                        self.rx = Some(rx);
                        thread::spawn(move || {
                            let result = fetch_and_parse(&url);
                            let _ = tx.send((url, result));
                        });
                    }
                });

                ui.add_space(20.0);

                // Tabs
                if !self.tabs.is_empty() {
                    ui.horizontal_wrapped(|ui| {
                        for key in self.tabs.keys() {
                            if ui
                                .selectable_label(
                                    self.active_tab.as_deref() == Some(key),
                                    key.clone(),
                                )
                                .clicked()
                            {
                                self.active_tab = Some(key.clone());
                            }
                        }
                    });
                    ui.separator();
                }

                // Handle fetch result
                if let Some(rx) = &self.rx {
                    if let Ok((url, content)) = rx.try_recv() {
                        let mut parts = content.splitn(3, "\n---\n");
                        let title = parts.next().unwrap_or_default().to_string();
                        let headings: Vec<String> = parts
                            .next()
                            .unwrap_or_default()
                            .lines()
                            .map(ToString::to_string)
                            .collect();
                        let paragraphs: Vec<String> = parts
                            .next()
                            .unwrap_or_default()
                            .lines()
                            .map(ToString::to_string)
                            .collect();

                        self.tabs.insert(url.clone(), (title, headings, paragraphs));
                        self.active_tab = Some(url);
                        self.rx = None;
                    }
                }

                // Display content
                if let Some(current) = &self.active_tab {
                    if let Some((title, heads, pars)) = self.tabs.get(current) {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.group(|ui| {
                                ui.label(current);
                                ui.heading(title);
                                for h in heads {
                                    ui.label(h);
                                }
                                for p in pars {
                                    ui.label(p);
                                }
                            });
                        });
                    }
                }
            });
        });

        ctx.request_repaint();
    }
}

fn enforce_https(url: &str) -> String {
    if url.starts_with("http://") {
        format!("https://{}", &url[7..])
    } else {
        url.to_string()
    }
}

fn fetch_and_parse(url: &str) -> String {
    let html = reqwest::blocking::get(url)
        .and_then(|r| r.text())
        .unwrap_or_else(|e| format!("Error fetching: {}", e));

    let doc = Html::parse_document(&html);
    let title_sel = Selector::parse("title").unwrap();
    let h1_sel = Selector::parse("h1").unwrap();
    let p_sel = Selector::parse("p").unwrap();

    let title = doc
        .select(&title_sel)
        .next()
        .map(|e| e.inner_html())
        .unwrap_or_else(|| "(no title)".into());

    let heads: Vec<String> = doc.select(&h1_sel).map(|e| e.inner_html()).collect();
    let pars: Vec<String> = doc.select(&p_sel).take(3).map(|e| e.inner_html()).collect();

    format!("{}\n---\n{}\n---\n{}", title, heads.join("\n"), pars.join("\n"))
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Ghayb Browser",
        options,
        Box::new(|_cc: &CreationContext| Ok(Box::new(GhaybApp::default()))),
    )
}
