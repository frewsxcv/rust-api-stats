#![feature(plugin_registrar)]
#![feature(rustc_private)]

#[macro_use] extern crate rustc;
extern crate rustc_plugin;

use rustc::lint as rustc_lint;
use rustc::lint::LintArray;
use rustc::hir as rustc_hir;

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
    fn check_expr(&mut self, cx: &rustc_lint::LateContext<'a, 'tcx>, e: &'tcx rustc_hir::Expr) {
        println!("{:?}", e);
    }
}
