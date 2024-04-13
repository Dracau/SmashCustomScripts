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

static mut mewtwoPosX: [f32; 8] = [0.0; 8];
static mut mewtwoPosY: [f32; 8] = [0.0; 8];
static mut mewtwoPosZ: [f32; 8] = [0.0; 8];

unsafe extern "C" fn game_appeallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        mewtwoPosX[entry_id] = PostureModule::pos_x(agent.module_accessor);
        mewtwoPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
        mewtwoPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);
    }
}
unsafe extern "C" fn game_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: mewtwoPosX[entry_id], y: mewtwoPosY[entry_id], z: mewtwoPosZ[entry_id] });
        StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
}

unsafe extern "C" fn game_appeallwr(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        mewtwoPosX[entry_id] = PostureModule::pos_x(agent.module_accessor);
        mewtwoPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
        mewtwoPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);
    }
}
unsafe extern "C" fn game_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        KineticModule::set_consider_ground_friction(agent.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: mewtwoPosX[entry_id], y: mewtwoPosY[entry_id], z: mewtwoPosZ[entry_id] });
        StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .game_acmd("game_appeallwl", game_appeallwl)
    .game_acmd("game_appealhil", game_appealhil)
    .game_acmd("game_appeallwr", game_appeallwr)
    .game_acmd("game_appealhir", game_appealhir)
        .install();
}
