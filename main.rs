use eframe::egui;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;

const SAVE_FILE: &str = "words_data.json";

// Screens
#[derive(PartialEq)]
enum Screen {
    AddWords,
    Game,
    End,
}

// Word model
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Word {
    foreign: String,
    translation: String,
    level: u8, // 1..=5
}

impl Word {
    fn new(foreign: String, translation: String) -> Self {
        Self {
            foreign,
            translation,
            level: 1,
        }
    }
}

// App state
struct App {
    screen: Screen,
    words: Vec<Word>,

    // Add form
    new_foreign: String,
    new_translation: String,

    // Game
    current_word_index: usize,
    user_answer: String,
    feedback_message: String,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            screen: Screen::AddWords,
            words: Vec::new(),
            new_foreign: String::new(),
            new_translation: String::new(),
            current_word_index: 0,
            user_answer: String::new(),
            feedback_message: String::new(),
        };
        app.load();
        app
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "woro 📚",
        options,
    Box::new(|_cc| Box::new(App::default())),

    )
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.screen {
            Screen::AddWords => self.add_words_screen(ui),
            Screen::Game => self.game_screen(ui),
            Screen::End => self.end_screen(ui),
        });
    }
}

// ------------------- Persistence -------------------
impl App {
    fn save(&self) {
        match serde_json::to_string_pretty(&self.words) {
            Ok(json) => {
                if let Err(e) = fs::write(SAVE_FILE, json) {
                    eprintln!("Error saving to {}: {}", SAVE_FILE, e);
                }
            }
            Err(e) => eprintln!("Error serializing words: {}", e),
        }
    }

    fn load(&mut self) {
        match fs::read_to_string(SAVE_FILE) {
            Ok(data) => match serde_json::from_str::<Vec<Word>>(&data) {
                Ok(vec) => {
                    self.words = vec;
                    if !self.words.is_empty() {
                        self.current_word_index = 0;
                    }
                }
                Err(e) => eprintln!("Error parsing {}: {}", SAVE_FILE, e),
            },
            Err(_e) => { /* first run: ignore */ }
        }
    }
}

// ------------------- UI Screens -------------------
impl App {
    fn add_words_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Add New Words");
        ui.add_space(10.0);

        // TXT import
        if ui.button("📁 Import from TXT").clicked() {
            self.import_from_txt();
        }

        ui.add_space(10.0);
        egui::Grid::new("add_word_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.label("Foreign word:");
                ui.text_edit_singleline(&mut self.new_foreign);
                ui.end_row();

                ui.label("Translation:");
                let response = ui.text_edit_singleline(&mut self.new_translation);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.add_word();
                }
                ui.end_row();
            });

        ui.add_space(8.0);
        if ui.button("➕ Add Word").clicked() {
            self.add_word();
        }

        if !self.words.is_empty() {
            if ui.button("🎮 Go to Game").clicked() {
                self.feedback_message.clear();
                self.screen = Screen::Game;
                self.pick_random_word();
            }
        }

        ui.separator();
        ui.heading("Your Words");

        if self.words.is_empty() {
            ui.label("No words yet. Add some words to start learning!");
        } else {
            egui::ScrollArea::vertical()
                .max_height(320.0)
                .show(ui, |ui| {
                    let mut to_delete: Option<usize> = None;

                    for (i, word) in self.words.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("🔹 {} = {}", word.foreign, word.translation));
                            ui.label(format!("(Level {})", word.level));
                            if ui.button("🗑 Delete").clicked() {
                                to_delete = Some(i);
                            }
                        });
                    }

                    if let Some(index) = to_delete {
                        self.words.remove(index);
                        self.save();
                    }
                });
        }
    }

    fn game_screen(&mut self, ui: &mut egui::Ui) {
        if self.words.is_empty() {
            ui.heading("No words yet!");
            ui.label("Go to 'Add Words' and add some words first.");
            return;
        }

        ui.heading("🎮 Game Mode");
        ui.add_space(6.0);

        // Progress
        let mastered = self.words.iter().filter(|w| w.level >= 5).count();
        let total = self.words.len();
        let progress = mastered as f32 / (total as f32).max(1.0);
        ui.horizontal(|ui| {
            ui.label("Mastery:");
            ui.add(
                egui::ProgressBar::new(progress)
                    .text(format!("{}/{} mastered", mastered, total))
                    .desired_width(220.0),
            );
        });

        ui.separator();
        ui.add_space(10.0);

        let word = &self.words[self.current_word_index];
        ui.label("What is the translation of this word?");
        ui.label(
            egui::RichText::new(&word.foreign)
                .size(48.0)
                .strong(),
        );
        ui.label(format!("Level: {}", word.level));

        ui.add_space(12.0);
        ui.label("Your answer:");
        let response = ui.text_edit_singleline(&mut self.user_answer);

        // Clear feedback on input change
        if response.changed() {
            self.feedback_message.clear();
        }

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.check_answer();
        }

        if ui.button("✓ Check").clicked() {
            self.check_answer();
        }

        ui.add_space(10.0);
        if !self.feedback_message.is_empty() {
            ui.label(&self.feedback_message);
        }
    }

    fn end_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(egui::RichText::new("🎉🎉🎉").size(72.0));
            ui.add_space(12.0);

            ui.label(
                egui::RichText::new("CONGRATULATIONS!")
                    .size(40.0)
                    .strong()
                    .color(egui::Color32::GOLD),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("ALL WORDS MASTERED!")
                    .size(26.0)
                    .color(egui::Color32::GREEN),
            );

            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(format!("You mastered {} words!", self.words.len()))
                    .size(18.0),
            );

            ui.add_space(30.0);
            if ui.button(egui::RichText::new("🔄 Play Again").size(18.0)).clicked() {
                for w in &mut self.words {
                    w.level = 1;
                }
                self.save();
                self.screen = Screen::Game;
                self.pick_random_word();
            }

            if ui.button(egui::RichText::new("➕ Add More Words").size(18.0)).clicked() {
                self.screen = Screen::AddWords;
            }

            ui.add_space(10.0);
            ui.label("Thanks for using woro!");
        });
    }
}

// ------------------- Core logic -------------------
impl App {
    fn add_word(&mut self) {
        if !self.new_foreign.trim().is_empty() && !self.new_translation.trim().is_empty() {
            self.words.push(Word::new(
                self.new_foreign.trim().to_string(),
                self.new_translation.trim().to_string(),
            ));
            self.new_foreign.clear();
            self.new_translation.clear();
            self.save();
        }
    }

    fn import_from_txt(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text Files", &["txt"])
            .set_title("Select word list")
            .pick_file()
        {
            match fs::read_to_string(path) {
                Ok(content) => self.parse_txt_content(&content),
                Err(e) => eprintln!("Error reading file: {}", e),
            }
        }
    }

    fn parse_txt_content(&mut self, content: &str) {
        let mut added = 0usize;
        let mut skipped = 0usize;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let foreign = parts[0].to_string();
                let translation = parts[1..].join(" ");
                self.words.push(Word::new(foreign, translation));
                added += 1;
            } else {
                skipped += 1;
            }
        }

        if added > 0 {
            self.save();
        }
        println!("✅ Added {} words, skipped {} invalid lines", added, skipped);
    }

    fn pick_random_word(&mut self) {
        if self.words.is_empty() {
            return;
        }
        let mut rng = rand::thread_rng();
        self.current_word_index = rng.gen_range(0..self.words.len());
    }

    fn check_answer(&mut self) {
        let idx = self.current_word_index;
        let correct_translation = self.words[idx].translation.clone();
        let old_level = self.words[idx].level;
        let user = self.user_answer.trim().to_lowercase();
        let right = correct_translation.to_lowercase();

        if user == right {
            let w = &mut self.words[idx];
            if w.level < 5 {
                w.level += 1;
                self.feedback_message =
                    format!("✅ CORRECT! Level: {} → {}", old_level, w.level);
            } else {
                self.feedback_message = "✅ CORRECT! Already mastered!".to_string();
            }
        } else {
            let w = &mut self.words[idx];
            if w.level > 1 {
                w.level -= 1;
            }
            self.feedback_message = format!(
                "❌ WRONG! Correct answer: {} (Level: {} → {})",
                correct_translation, old_level, w.level
            );
        }

        // Save persistent progress
        self.save();

        // Move to next word
        self.pick_random_word();

        // All mastered?
        if self.all_words_mastered() {
            self.screen = Screen::End;
            self.feedback_message.clear();
        }

        self.user_answer.clear();
    }

    fn all_words_mastered(&self) -> bool {
        !self.words.is_empty() && self.words.iter().all(|w| w.level >= 5)
    }
}
