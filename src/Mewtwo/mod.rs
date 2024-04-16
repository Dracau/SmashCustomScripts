use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::*,
    std::ptr
};

static mut mewtwoPosX: [f32; 8] = [0.0; 8];
static mut mewtwoPosY: [f32; 8] = [0.0; 8];
static mut mewtwoPosZ: [f32; 8] = [0.0; 8];

static mut mewtwoSetPos: [bool; 8] = [false; 8];

pub static mut SPOT_FX_DELAY : u64 = 15;
static mut SPOT_FX_CURRENT: [u64; 8] = [0; 8];

static mut pos : Vector3f = Vector3f{x:0.0,y:0.0,z:0.0};

unsafe extern "C" fn game_appeallwl(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 2.0);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        mewtwoPosX[entry_id] = PostureModule::pos_x(agent.module_accessor);
        mewtwoPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
        mewtwoPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);
        mewtwoSetPos[entry_id] = true;
    }
}

unsafe extern "C" fn game_appeallwr(agent: &mut L2CAgentBase) {
    MotionModule::set_rate(agent.module_accessor, 2.0);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent){
        let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        mewtwoPosX[entry_id] = PostureModule::pos_x(agent.module_accessor);
        mewtwoPosY[entry_id] = PostureModule::pos_y(agent.module_accessor);
        mewtwoPosZ[entry_id] = PostureModule::pos_z(agent.module_accessor);
        mewtwoSetPos[entry_id] = true;
    }
}
unsafe extern "C" fn game_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) && mewtwoSetPos[entry_id] {
        StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        let correct_kind = smash::app::GroundCorrectKind(GroundModule::get_correct(agent.module_accessor));
        
        GroundModule::set_correct(agent.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
        PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: mewtwoPosX[entry_id], y: mewtwoPosY[entry_id], z: mewtwoPosZ[entry_id] });
        GroundModule::set_correct(agent.module_accessor, correct_kind);

        mewtwoSetPos[entry_id] = false;
    }
}

unsafe extern "C" fn game_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(agent) && mewtwoSetPos[entry_id] {
        StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        let correct_kind = smash::app::GroundCorrectKind(GroundModule::get_correct(agent.module_accessor));
        
        GroundModule::set_correct(agent.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
        PostureModule::set_pos(agent.module_accessor, &Vector3f{ x: mewtwoPosX[entry_id], y: mewtwoPosY[entry_id], z: mewtwoPosZ[entry_id] });
        GroundModule::set_correct(agent.module_accessor, correct_kind);

        mewtwoSetPos[entry_id] = false;
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 160.0, 55, 118, 0, 20, 7.5, 0.0, 4.0, 13.7, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 160.0, 55, 118, 0, 20, 11.0, 0.0, 4.0, 13.7, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        
        if mewtwoSetPos[entry_id]{

            let mutPos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};

            ModelModule::joint_global_position(
            agent.module_accessor,
        Hash40::new("top"),
        mutPos,
        true
            );

            let mewtwoPos = *PostureModule::pos(agent.module_accessor);
            pos = *mutPos;

            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 16.0, 55, 118, 0,
        20, 30, -pos.x + mewtwoPos.x, -pos.y + mewtwoPos.y, -pos.z, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF,
    *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0,
false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn mewtwo_frame(agent: &mut L2CAgentBase) {

    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if StatusModule::status_kind(agent.module_accessor) == FIGHTER_STATUS_KIND_ENTRY || StatusModule::status_kind(agent.module_accessor) == FIGHTER_STATUS_KIND_DEAD || StatusModule::status_kind(agent.module_accessor) == FIGHTER_STATUS_KIND_REBIRTH{
        mewtwoSetPos[entry_id] = false;
    }
    
    SPOT_FX_CURRENT[entry_id] = SPOT_FX_CURRENT[entry_id] + 1;
    if SPOT_FX_CURRENT[entry_id] >= SPOT_FX_DELAY && mewtwoSetPos[entry_id]
    {
        SPOT_FX_CURRENT[entry_id] = 0;
        EffectModule::req(agent.module_accessor, Hash40::new("mewtwo_pk_attack_g"), &mut Vector3f{x:mewtwoPosX[entry_id], y:mewtwoPosY[entry_id] + 7.5, z:mewtwoPosZ[entry_id]}, &mut Vector3f{x:0.0, y:0.0, z:0.0}, 0.66, 0, 1, false, 0);
        EffectModule::req(agent.module_accessor, Hash40::new("mewtwo_pk_attack_g"), &mut Vector3f{x:pos.x, y:pos.y, z:pos.z}, &mut Vector3f{x:0.0, y:0.0, z:0.0}, 0.66, 0, 1, false, 0);
    }

}

pub fn install() {
    Agent::new("mewtwo")
    .game_acmd("game_appeallwl", game_appeallwl)
    .game_acmd("game_appealhil", game_appealhil)
    .game_acmd("game_appeallwr", game_appeallwr)
    .game_acmd("game_appealhir", game_appealhir)
    .game_acmd("game_attacklw4", game_attacklw4)
    .on_line(Main, mewtwo_frame)
        .install();
}
