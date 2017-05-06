#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate rustc_plugin;
use rustc_plugin::Registry;

extern crate api_stats_lint;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    api_stats_lint::register_plugins(reg);
}
