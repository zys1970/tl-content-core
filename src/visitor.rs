use crate::{ContentBlock, InlineSpan};

pub struct VisitContext<'a> {
    /// 当前所有祖先块（从根到直接父，不包含当前块）
    pub ancestors: &'a [ContentBlock],

    /// 当前深度（= ancestors.len() + 1）
    pub depth: usize,

    /// 直接父块（None 表示根级）
    pub parent: Option<&'a ContentBlock>,
}

pub trait ContentVisitor {
    // 进入一个块之前调用
    fn enter_block(&mut self, block: &ContentBlock, ctx: &VisitContext);

    // 退出一个块之后调用
    fn exit_block(&mut self, block: &ContentBlock, ctx: &VisitContext);

    // 处理行内元素
    fn visit_inline(&mut self, span: &InlineSpan);
}
