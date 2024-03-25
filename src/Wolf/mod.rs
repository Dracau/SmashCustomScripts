use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*
};

use skyline::hooks::{Region, getRegionAddress};
use skyline::libc::*;

static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT : i32 = 0x20000eb;


pub fn install() {

    Agent::new("wolf")
    //.game_acmd("game_specialsend", wolf_game_specialsend)
    .install();
}
