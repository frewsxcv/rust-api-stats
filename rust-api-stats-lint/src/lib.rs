#![feature(plugin_registrar)]
#![feature(rustc_private)]

#[macro_use] extern crate rustc;
extern crate rustc_plugin;

use rustc::lint as rustc_lint;
use rustc::lint::LintArray;
use rustc::hir as rustc_hir;

pub fn register_plugins(reg: &mut rustc_plugin::Registry) {
    reg.register_late_lint_pass(Box::new(BitMask));
}

declare_lint! {
    pub BAD_BIT_MASK,
    Warn,
    "expressions of the form `_ & mask == select` that will only ever return `true` or `false`"
}

struct BitMask;

impl rustc_lint::LintPass for BitMask {
    fn get_lints(&self) -> rustc_lint::LintArray {
        lint_array![BAD_BIT_MASK]
    }
}

impl<'a, 'tcx> rustc_lint::LateLintPass<'a, 'tcx> for BitMask {
    /*
    fn check_expr(&mut self, _cx: &rustc_lint::LateContext<'a, 'tcx>, e: &'tcx rustc_hir::Expr) {
        println!("{:?}", e);
    }
    */

    // check for unsafe blocks
    fn check_block(&mut self, ctx: &rustc_lint::LateContext<'a, 'tcx>, block: &'tcx rustc_hir::Block) {
        if is_unsafe_block(block) {
            println!("crate: {:?}", ctx.krate);
            println!("block: {:?}", block);
            println!("encountered unsafe block");
        }
    }
}

fn is_unsafe_block(block: &rustc_hir::Block) -> bool {
    block.rules == rustc_hir::BlockCheckMode::UnsafeBlock(rustc_hir::UnsafeSource::CompilerGenerated)
}
