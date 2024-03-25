#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

mod Ganondorf;
mod Hero;
mod Incineroar;
mod DRMario;
mod Greninja;
mod DonkeyKong;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    Ganondorf::install();
	Hero::install();
	Incineroar::install();
	DRMario::install();
	Greninja::install();
	DonkeyKong::install();
}