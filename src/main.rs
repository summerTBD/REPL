// ============================================================
// main.rs — 程序入口
// ============================================================
// 这个文件只做三件事：
//   1. 声明所有模块
//   2. 给 CalculatorApp 套上 eframe::App 接口
//   3. main 函数启动桌面窗口
//
// 项目文件结构一览：
//   ast.rs    — 抽象语法树定义（Expr 枚举、Op 运算符）
//   token.rs  — Token 定义（词法单元：数字、加减乘除、括号）
//   lexer.rs  — 词法分析器（字符串 → Token 序列）
//   parser.rs — 语法分析器（Token 序列 → 表达式树）
//   eval.rs   — 纯计算逻辑（递归求值表达式树）
//   app.rs    — 计算器状态管理（表达式存储、按钮操作）
//   ui.rs     — egui 界面布局（显示区 + 按钮网格）
//   main.rs   — 程序入口（本文件）

// ----- 模块声明 -----
mod app; // app.rs（CalculatorApp 状态管理）
mod ast; // AST.rs -> ast.rs（抽象语法树，定义表达式结构）
mod eval; // eval.rs（纯计算逻辑）
mod lexer; // lexer.rs（词法分析器，把字符串切成 token）
mod parser; // parser.rs（语法分析器，把 token 组合成表达式树）
mod token; // token.rs（定义 Token 枚举，如数字、加减乘除等）
mod ui; // ui.rs（egui 图形界面布局）

use crate::app::CalculatorApp; // 计算器状态
use eframe::egui; // egui 框架
use ui::draw; // 界面绘制函数

/// 给 CalculatorApp 套上 eframe::App 接口
///
/// egui 是"即时模式 GUI"：
///   每一帧（每秒 60 次）都会调用 update，在里面重新描述界面
///   我们在 update 里调用 ui::draw() 来画整个界面
impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        draw(self, ctx); // 委托 ui.rs 负责具体绘制
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
