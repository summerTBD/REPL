// ============================================================
// app.rs — 计算器状态管理
// ============================================================
// 这个文件负责：
//   1. 定义 CalculatorApp 结构体（保存表达式和结果）
//   2. 按钮操作逻辑（input/clear/backspace）
//   3. 核心计算流程：词法分析 → 语法分析 → 执行计算
// 但不会画界面——界面代码在 ui.rs

use crate::eval::eval; // 纯计算逻辑
use crate::lexer::Lexer; // 词法分析器
use crate::parser::Parser; // 语法分析器

/// 计算器应用的状态
///
/// display：用户正在输入的表达式字符串（如 "1+2*3"）
/// result ：实时计算的结果（如 "7"）
pub struct CalculatorApp {
    pub display: String, // 用户正在输入的表达式
    pub result: String,  // 下方的计算结果
}

impl CalculatorApp {
    // ----- 构造函数：创建一个全新的计算器状态 -----
    pub fn new() -> Self {
        Self {
            display: String::new(),    // 表达式初始为空
            result: String::from("0"), // 结果初始显示 "0"
        }
    }

    // ----- 用户点击了一个按钮（数字/运算符/括号）-----
    // s 是按钮上的文字，比如 "5" 或 "+" 或 "("
    pub fn input(&mut self, s: &str) {
        self.display.push_str(s); // 把字符追加到表达式末尾
        self.try_eval(); // 输入后立即尝试计算并更新结果
    }

    // ----- 清空按钮 C：重置所有状态 -----
    pub fn clear(&mut self) {
        self.display.clear(); // 清空表达式
        self.result = String::from("0"); // 结果重置为 "0"
    }

    // ----- 退格按钮 DEL：删除表达式最后一个字符 -----
    pub fn backspace(&mut self) {
        self.display.pop(); // pop() 会删除最后一个字符
        self.try_eval(); // 删除后重新计算结果
    }

    // ----- 核心：解析表达式并计算结果 -----
    /// 流程：词法分析 → 语法分析 → 执行计算
    /// 用 catch_unwind 防止表达式不完整时 panic 导致程序崩溃
    pub fn try_eval(&mut self) {
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
