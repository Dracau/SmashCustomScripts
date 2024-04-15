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

//mod Everyone;
mod Ganondorf;
mod Hero;
mod Incineroar;
mod Mario;
mod DRMario;
mod Greninja;
mod DonkeyKong;
mod Wolf;
mod Mewtwo;
mod Ridley;
//mod DonkeyKongBarrel;


#[skyline::main(name = "smashline_test")]
pub fn main() {
	//Everyone::install();
    Ganondorf::install();
	Hero::install();
	Incineroar::install();
	Mario::install();
	DRMario::install();
	Greninja::install();
	DonkeyKong::install();
	Wolf::install();
	Mewtwo::install();
	Ridley::install();
	//DonkeyKongBarrel::install();
}