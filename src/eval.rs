// ============================================================
// eval.rs — 纯计算逻辑：递归求值表达式树
// ============================================================
// 这个文件只做一件事：输入一棵表达式树，输出计算结果
// 不涉及任何 UI、状态管理，是纯函数

use crate::ast::{Expr, Op}; // 表达式树节点和运算符定义

/// 递归计算表达式树的值
///
/// 参数 expr 是一棵二叉树：
///   - 叶子节点：Expr::Number(数字)，直接返回值
///   - 分支节点：Expr::Binary{ op, left, right }，先递归算左右子树，再执行运算
///
/// # 返回值
/// - 正常计算结果（f64）
/// - 除零时返回 f64::NAN（Not A Number）
pub fn eval(expr: &Expr) -> f64 {
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
