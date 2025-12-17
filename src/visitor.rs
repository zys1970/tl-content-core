use crate::{ContentBlock, InlineSpan};

pub struct VisitContext<'a> {
    pub block_stack: &'a [ContentBlock],//当前所有父节点
}

pub trait ContentVisitor {
    fn visit_block(&mut self, block: &ContentBlock);
    fn enter_block(&mut self, block: &ContentBlock, ctx:&VisitContext);
    fn exit_block(&mut self, block: &ContentBlock, ctx:&VisitContext);
    fn visit_inline(&mut self, span: &InlineSpan);
}
