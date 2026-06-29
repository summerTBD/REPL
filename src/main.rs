mod ast;
mod lexer;
mod parser;
mod token;

use crate::{
    ast::{Expr, Op},
    lexer::Lexer,
    parser::Parser,
};
use eframe::egui;

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Binary { op, left, right } => {
            let left_val = eval(left);
            let right_val = eval(right);
            match op {
                Op::Add => left_val + right_val,
                Op::Sub => left_val - right_val,
                Op::Mul => left_val * right_val,
                Op::Div => {
                    if right_val == 0.0 {
                        return f64::NAN;
                    }
                    left_val / right_val
                }
            }
        }
    }
}

struct CalculatorApp {
    display: String,
    result: String,
}

impl CalculatorApp {
    fn new() -> Self {
        Self {
            display: String::new(),
            result: String::from("0"),
        }
    }

    fn input(&mut self, s: &str) {
        self.display.push_str(s);
        self.try_eval();
    }

    fn clear(&mut self) {
        self.display.clear();
        self.result = String::from("0");
    }

    fn backspace(&mut self) {
        self.display.pop();
        self.try_eval();
    }

    fn try_eval(&mut self) {
        if self.display.is_empty() {
            self.result = String::from("0");
            return;
        }
        let input = self.display.clone();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let lexer = Lexer::new(&input);
            let mut parser = Parser::new(lexer);
            let expr = parser.parse_expr();
            eval(&expr)
        }));
        match result {
            Ok(val) if val.is_nan() => {
                self.result = String::from("错误：除零");
            }
            Ok(val) => {
                // 整数不显示小数点
                if val == (val as i64) as f64 && val.is_finite() {
                    self.result = format!("{}", val as i64);
                } else {
                    self.result = format!("{}", val);
                }
            }
            Err(_) => {
                // 表达式不完整或语法错误，不更新结果
            }
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button_size = egui::vec2(60.0, 45.0);
            let spacing = 6.0;

            // ===== 显示区域 =====
            // 结果显示（大字号）
            ui.add_space(10.0);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 50.0),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(&self.result).size(32.0).strong(),
                    ));
                },
            );
            // 表达式显示（小字号灰色）
            let expr_text = if self.display.is_empty() {
                "".to_string()
            } else {
                self.display.clone()
            };
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 30.0),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(expr_text)
                            .size(18.0)
                            .color(egui::Color32::GRAY),
                    ));
                },
            );
            ui.add_space(10.0);

            // ===== 按钮区域 =====
            ui.vertical_centered(|ui| {
                let _grid_width = 4.0 * button_size.x + 3.0 * spacing;

                // 第1行：C  (  )  /
                ui.horizontal(|ui| {
                    if ui.add_sized(button_size, egui::Button::new("C")).clicked() {
                        self.clear();
                    }
                    ui.add_space(spacing);
                    if ui.add_sized(button_size, egui::Button::new("(")).clicked() {
                        self.input("(");
                    }
                    ui.add_space(spacing);
                    if ui.add_sized(button_size, egui::Button::new(")")).clicked() {
                        self.input(")");
                    }
                    ui.add_space(spacing);
                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(egui::RichText::new("/").size(22.0).strong()),
                        )
                        .clicked()
                    {
                        self.input("/");
                    }
                });

                ui.add_space(spacing);

                // 第2行：7  8  9  *
                ui.horizontal(|ui| {
                    for &label in &["7", "8", "9", "*"] {
                        if ui
                            .add_sized(
                                button_size,
                                egui::Button::new(egui::RichText::new(label).size(20.0)),
                            )
                            .clicked()
                        {
                            self.input(label);
                        }
                        ui.add_space(spacing);
                    }
                });

                ui.add_space(spacing);

                // 第3行：4  5  6  -
                ui.horizontal(|ui| {
                    for &label in &["4", "5", "6", "-"] {
                        if ui
                            .add_sized(
                                button_size,
                                egui::Button::new(egui::RichText::new(label).size(20.0)),
                            )
                            .clicked()
                        {
                            self.input(label);
                        }
                        ui.add_space(spacing);
                    }
                });

                ui.add_space(spacing);

                // 第4行：1  2  3  +
                ui.horizontal(|ui| {
                    for &label in &["1", "2", "3", "+"] {
                        if ui
                            .add_sized(
                                button_size,
                                egui::Button::new(egui::RichText::new(label).size(20.0)),
                            )
                            .clicked()
                        {
                            self.input(label);
                        }
                        ui.add_space(spacing);
                    }
                });

                ui.add_space(spacing);

                // 第5行：0  .  DEL  =
                ui.horizontal(|ui| {
                    if ui.add_sized(button_size, egui::Button::new("0")).clicked() {
                        self.input("0");
                    }
                    ui.add_space(spacing);
                    if ui.add_sized(button_size, egui::Button::new(".")).clicked() {
                        self.input(".");
                    }
                    ui.add_space(spacing);
                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(egui::RichText::new("DEL").size(16.0)),
                        )
                        .clicked()
                    {
                        self.backspace();
                    }
                    ui.add_space(spacing);
                    let eq_response = ui.add_sized(
                        button_size,
                        egui::Button::new(
                            egui::RichText::new("=")
                                .size(22.0)
                                .strong()
                                .color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(0, 120, 215)),
                    );
                    if eq_response.clicked() {
                        self.try_eval();
                    }
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 460.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::new()))),
    )
}
