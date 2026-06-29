// ============================================================
// 模块声明：引入项目的其他源文件
// ============================================================
mod ast; // AST.rs -> ast.rs（抽象语法树，定义表达式结构）
mod lexer; // lexer.rs（词法分析器，把字符串切成 token）
mod parser; // parser.rs（语法分析器，把 token 组合成表达式树）
mod token; // token.rs（定义 Token 枚举，如数字、加减乘除等）

// 从自己项目里导入需要用到的类型
use crate::{
    ast::{Expr, Op}, // Expr = 表达式树节点，Op = 运算符
    lexer::Lexer,    // 词法分析器
    parser::Parser,  // 语法分析器
};
// 导入 eframe 库（egui 的桌面窗口框架）
use eframe::egui;

// ============================================================
// eval 函数：递归计算表达式树的值
// ============================================================
// 参数 expr 是一棵二叉树，叶子是数字，节点是运算符
// 返回计算结果（f64 浮点数）
fn eval(expr: &Expr) -> f64 {
    match expr {
        // 情况1：叶子节点就是数字，直接返回这个数字
        Expr::Number(n) => *n,

        // 情况2：二叉树节点，包含 运算符 + 左子树 + 右子树
        Expr::Binary { op, left, right } => {
            // 先递归计算左右子树的值
            let left_val = eval(left);
            let right_val = eval(right);
            // 再根据运算符做计算
            match op {
                Op::Add => left_val + right_val, // 加法
                Op::Sub => left_val - right_val, // 减法
                Op::Mul => left_val * right_val, // 乘法
                Op::Div => {
                    // 除法要检查除数是否为 0
                    if right_val == 0.0 {
                        return f64::NAN; // NAN = Not A Number，代表"错误"
                    }
                    left_val / right_val
                }
            }
        }
    }
}

// ============================================================
// CalculatorApp 结构体：保存计算器的状态
// ============================================================
// display：用户输入的表达式字符串（如 "1+2*3"）
// result：实时计算的结果（如 "7"）
struct CalculatorApp {
    display: String, // 用户正在输入的表达式
    result: String,  // 下方的计算结果
}

impl CalculatorApp {
    // ----- 构造函数：创建一个全新的计算器状态 -----
    fn new() -> Self {
        Self {
            display: String::new(),    // 表达式初始为空
            result: String::from("0"), // 结果初始显示 "0"
        }
    }

    // ----- 用户点击了一个按钮（数字/运算符/括号）-----
    // s 是按钮上的文字，比如 "5" 或 "+" 或 "("
    fn input(&mut self, s: &str) {
        self.display.push_str(s); // 把字符追加到表达式末尾
        self.try_eval(); // 输入后立即尝试计算并更新结果
    }

    // ----- 清空按钮 C：重置所有状态 -----
    fn clear(&mut self) {
        self.display.clear(); // 清空表达式
        self.result = String::from("0"); // 结果重置为 "0"
    }

    // ----- 退格按钮 DEL：删除表达式最后一个字符 -----
    fn backspace(&mut self) {
        self.display.pop(); // pop() 会删除最后一个字符
        self.try_eval(); // 删除后重新计算结果
    }

    // ----- 核心：解析表达式并计算结果 -----
    fn try_eval(&mut self) {
        // 如果表达式为空，直接显示 "0"
        if self.display.is_empty() {
            self.result = String::from("0");
            return;
        }

        // 克隆一份表达式字符串（因为下面要传进闭包）
        let input = self.display.clone();

        // catch_unwind：捕获 panic，防止表达式不完整时程序崩溃
        // 比如用户输入 "1+" 还没写完，解析器会 panic，我们捕获它
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // 1. 词法分析：把字符串切成 token 序列
            //    比如 "1+2" -> [Number(1), Plus, Number(2)]
            let lexer = Lexer::new(&input);

            // 2. 语法分析：把 token 序列构建成表达式树
            //    比如 [Number(1), Plus, Number(2)] -> Binary{ op:Add, left:1, right:2 }
            let mut parser = Parser::new(lexer);
            let expr = parser.parse_expr();

            // 3. 执行计算
            eval(&expr)
        }));

        // 根据捕获的结果更新显示
        match result {
            // 计算成功，但结果是 NAN（除零错误）
            Ok(val) if val.is_nan() => {
                self.result = String::from("错误：除零");
            }
            // 计算成功，正常显示结果
            Ok(val) => {
                // 如果是整数（如 7.0），就不显示小数点后面的 .0
                if val == (val as i64) as f64 && val.is_finite() {
                    self.result = format!("{}", val as i64);
                } else {
                    self.result = format!("{}", val);
                }
            }
            // 计算失败（表达式不完整或语法错误），结果保持不变
            Err(_) => {
                // 什么都不做，保留上一次的结果
            }
        }
    }
}

// ============================================================
// 实现 eframe::App 接口 —— 这是 egui 框架必须的
// ============================================================
// egui 是一个"即时模式 GUI 库"：
//   每一帧（每秒钟 60 次）都会调用 update 函数
//   我们在 update 里"描述"界面的样子
impl eframe::App for CalculatorApp {
    // update 每帧调用一次，负责绘制整个界面
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                        egui::RichText::new(&self.result)
                            .size(32.0) // 字号 32
                            .strong(), // 加粗
                    ));
                },
            );

            // 第2行：显示用户输入的表达式（灰色小字）
            let expr_text = if self.display.is_empty() {
                "".to_string() // 空表达式不显示任何东西
            } else {
                self.display.clone()
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
                        self.clear();
                    }
                    ui.add_space(spacing);

                    // ( 按钮 - 左括号
                    if ui.add_sized(button_size, egui::Button::new("(")).clicked() {
                        self.input("(");
                    }
                    ui.add_space(spacing);

                    // ) 按钮 - 右括号
                    if ui.add_sized(button_size, egui::Button::new(")")).clicked() {
                        self.input(")");
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
                        self.input("/");
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
                            self.input(label);
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
                            self.input(label);
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
                            self.input(label);
                        }
                        ui.add_space(spacing);
                    }
                });

                ui.add_space(spacing);

                // ---- 第5行：0  .  DEL  = ----
                ui.horizontal(|ui| {
                    // 0 按钮
                    if ui.add_sized(button_size, egui::Button::new("0")).clicked() {
                        self.input("0");
                    }
                    ui.add_space(spacing);

                    // . 按钮（小数点）
                    if ui.add_sized(button_size, egui::Button::new(".")).clicked() {
                        self.input(".");
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
                        self.backspace();
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
                        self.try_eval();
                    }
                });
            });
        });
    }
}

// ============================================================
// 程序入口：main 函数
// ============================================================
fn main() -> eframe::Result<()> {
    // 配置窗口选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 460.0]) // 窗口内尺寸：宽300，高460
            .with_resizable(false), // 不允许用户拖拽改变窗口大小
        ..Default::default() // 其他选项用默认值
    };

    // 启动 egui 桌面应用
    eframe::run_native(
        "Rust Calculator", // 窗口标题
        options,           // 窗口配置
        // 这个闭包在应用启动时调用，创建 CalculatorApp 实例
        Box::new(|_cc| Ok(Box::new(CalculatorApp::new()))),
    )
}
