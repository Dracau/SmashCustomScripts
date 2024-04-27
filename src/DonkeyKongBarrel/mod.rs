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

pub static mut DK_MAX_BARREL_COUNT: u64 = 2;
pub static mut DK_TAUNT_BARREL_APPEAR_FRAME : f32 = 15.0;

unsafe extern "C" fn game_appealsl(agent: &mut L2CAgentBase) {
    if !max_barrel_count_reached() { 
        frame(agent.lua_state_agent, DK_TAUNT_BARREL_APPEAR_FRAME);
        if macros::is_excute(agent) && !max_barrel_count_reached() {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP,true);
        }
    }
}

unsafe extern "C" fn game_appealsr(agent: &mut L2CAgentBase) {
    if !max_barrel_count_reached() { 
        frame(agent.lua_state_agent, DK_TAUNT_BARREL_APPEAR_FRAME);
        if macros::is_excute(agent) && !max_barrel_count_reached() {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
            StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP,true);
        }
    }
}

pub unsafe fn max_barrel_count_reached() -> bool {
    if smash::app::lua_bind::ItemManager::get_num_of_active_item(*ITEM_KIND_BARREL) >= 1 * DK_MAX_BARREL_COUNT {
        return true;
    }
    return false;
}

pub fn install() {
    Agent::new("donkey")
        .game_acmd("game_appealsl", game_appealsl)
        .game_acmd("game_appealsr", game_appealsr)
        .install();
}
