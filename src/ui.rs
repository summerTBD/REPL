// ============================================================
// ui.rs — egui 图形界面布局
// ============================================================
// 这个文件只负责"画界面"：
//   - 显示区域（结果 + 表达式）
//   - 按钮网格（5行 × 4列）
// 按钮被点击时调用 CalculatorApp 的方法（在 app.rs 里定义）
// 不涉及任何计算逻辑

use crate::app::CalculatorApp;
use eframe::egui;

/// 绘制整个计算器界面
///
/// 布局从上到下：
///   1. 结果行（大号加粗，右对齐）—— 显示计算结果
///   2. 表达式行（灰色小字，右对齐）—— 显示用户输入
///   3. 按钮网格（5行 × 4列）—— C ( ) / | 7 8 9 * | 4 5 6 - | 1 2 3 + | 0 . DEL =
pub fn draw(app: &mut CalculatorApp, ctx: &egui::Context) {
    // CentralPanel：占据整个窗口的主面板
    egui::CentralPanel::default().show(ctx, |ui| {
        // ---------- 定义按钮大小和间距 ----------
        let button_size = egui::vec2(60.0, 45.0); // 宽60像素，高45像素
        let spacing = 6.0; // 按钮之间间隔 6 像素

        // ===================================================
        // 上半部分：显示区域
        // ===================================================

        ui.add_space(10.0); // 顶部留白 10 像素

        // 第1行：显示计算结果（大号加粗字体）
        // allocate_ui_with_layout：创建一个矩形区域，内容右对齐
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 50.0), // 区域大小：宽=窗口宽，高=50
            egui::Layout::right_to_left(egui::Align::Center), // 右对齐
            |ui| {
                ui.add(egui::Label::new(
                    // RichText 可以设置文字样式
                    egui::RichText::new(&app.result)
                        .size(32.0) // 字号 32
                        .strong(), // 加粗
                ));
            },
        );

        // 第2行：显示用户输入的表达式（灰色小字）
        let expr_text = if app.display.is_empty() {
            "".to_string() // 空表达式不显示任何东西
        } else {
            app.display.clone()
        };
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 30.0), // 高30像素
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(expr_text)
                        .size(18.0) // 字号 18
                        .color(egui::Color32::GRAY), // 灰色
                ));
            },
        );

        ui.add_space(10.0); // 显示区域和按钮之间留白

        // ===================================================
        // 下半部分：按钮区域
        // ===================================================

        // vertical_centered：让按钮区域在窗口中水平居中
        ui.vertical_centered(|ui| {
            let _grid_width = 4.0 * button_size.x + 3.0 * spacing;

            // ---- 第1行：C  (  )  / ----
            ui.horizontal(|ui| {
                // horizontal：这一行从左到右排列
                // C 按钮 - 清空
                if ui.add_sized(button_size, egui::Button::new("C")).clicked() {
                    app.clear();
                }
                ui.add_space(spacing);

                // ( 按钮 - 左括号
                if ui.add_sized(button_size, egui::Button::new("(")).clicked() {
                    app.input("(");
                }
                ui.add_space(spacing);

                // ) 按钮 - 右括号
                if ui.add_sized(button_size, egui::Button::new(")")).clicked() {
                    app.input(")");
                }
                ui.add_space(spacing);

                // / 按钮 - 除号
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("/").size(22.0).strong()),
                    )
                    .clicked()
                {
                    app.input("/");
                }
            });

            ui.add_space(spacing); // 行间距

            // ---- 第2行：7  8  9  * ----
            ui.horizontal(|ui| {
                // 用循环简化重复代码
                for &label in &["7", "8", "9", "*"] {
                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(egui::RichText::new(label).size(20.0)),
                        )
                        .clicked()
                    {
                        app.input(label);
                    }
                    ui.add_space(spacing);
                }
            });

            ui.add_space(spacing);

            // ---- 第3行：4  5  6  - ----
            ui.horizontal(|ui| {
                for &label in &["4", "5", "6", "-"] {
                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(egui::RichText::new(label).size(20.0)),
                        )
                        .clicked()
                    {
                        app.input(label);
                    }
                    ui.add_space(spacing);
                }
            });

            ui.add_space(spacing);

            // ---- 第4行：1  2  3  + ----
            ui.horizontal(|ui| {
                for &label in &["1", "2", "3", "+"] {
                    if ui
                        .add_sized(
                            button_size,
                            egui::Button::new(egui::RichText::new(label).size(20.0)),
                        )
                        .clicked()
                    {
                        app.input(label);
                    }
                    ui.add_space(spacing);
                }
            });

            ui.add_space(spacing);

            // ---- 第5行：0  .  DEL  = ----
            ui.horizontal(|ui| {
                // 0 按钮
                if ui.add_sized(button_size, egui::Button::new("0")).clicked() {
                    app.input("0");
                }
                ui.add_space(spacing);

                // . 按钮（小数点）
                if ui.add_sized(button_size, egui::Button::new(".")).clicked() {
                    app.input(".");
                }
                ui.add_space(spacing);

                // DEL 按钮（退格）
                if ui
                    .add_sized(
                        button_size,
                        egui::Button::new(egui::RichText::new("DEL").size(16.0)),
                    )
                    .clicked()
                {
                    app.backspace();
                }
                ui.add_space(spacing);

                // = 按钮（等号，蓝色背景白色文字，特殊样式）
                let eq_response = ui.add_sized(
                    button_size,
                    egui::Button::new(
                        egui::RichText::new("=")
                            .size(22.0)
                            .strong()
                            .color(egui::Color32::WHITE), // 白色文字
                    )
                    .fill(egui::Color32::from_rgb(0, 120, 215)), // 蓝色背景
                );
                if eq_response.clicked() {
                    app.try_eval();
                }
            });
        });
    });
}
