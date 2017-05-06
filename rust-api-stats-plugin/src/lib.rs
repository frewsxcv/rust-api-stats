#![feature(plugin_registrar)]
#![feature(rustc_private)]
#![allow(unknown_lints)]
#![allow(missing_docs_in_private_items)]

extern crate rustc_plugin;
use rustc_plugin::Registry;

extern crate clippy_lints;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    if let Ok(lint_store) = reg.sess.lint_store.try_borrow() {
        if lint_store
               .get_lint_groups()
               .iter()
               .any(|&(s, _, _)| s == "clippy") {
            reg.sess
                .struct_warn("running cargo clippy on a crate that also imports the clippy plugin")
                .emit();
            return;
        }
    }

    clippy_lints::register_plugins(reg);
}
