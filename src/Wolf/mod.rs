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

static mut WOLF_NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
pub const FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT : i32 = 0x200000e0;

unsafe extern "C" fn wolf_game_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);

    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 270, 90, 0, 6, 5.0, 0.0, 6.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 270, 90, 0, 6, 7.0, 0.0, 2.0, 0.5, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


#[skyline::hook(offset = WOLF_NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn wolf_notify_log_event_collision_hit_replace(fighter_manager: *mut smash::app::FighterManager, attacker_id: u32, defender_id: u32, move_type: f32, arg5: i32, move_type_again: bool, fighter: &mut L2CAgentBase) -> u64 {
    
    println!("Wolf Hook !");
    
    let attacker_boma = sv_battle_object::module_accessor(attacker_id);
    let defender_boma = sv_battle_object::module_accessor(defender_id);
    let attacker_kind = sv_battle_object::kind(attacker_id);
    let defender_kind = sv_battle_object::kind(defender_id);
    
    if attacker_kind == FIGHTER_KIND_WOLF{
        // if search_hit flag is on
        if WorkModule::is_flag(attacker_boma, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT) {
            println!("Wolf hook : Found cancel flag");
            // add velocity
            //KineticModule::add_speed(attacker_boma, &Vector3f{ x: 0.0, y: 2.5, z: 0.0 });
            StatusModule::change_status_request_from_script(attacker_boma, FIGHTER_STATUS_KIND_FALL.into(), false.into());

            // disable flag
            WorkModule::off_flag(attacker_boma, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
            // if thing being hit is a fighter
            if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                // give fighter being hit sticky bomb
                //ItemModule::have_item(defender_boma, smash::app::ItemKind(*ITEM_KIND_CHEWING), 0, 0, false, false);
            }
        }
    }
    
    original!()(fighter_manager, attacker_id, defender_id, move_type, arg5, move_type_again, fighter)
}

unsafe extern "C" fn wolf_frame(fighter: &mut L2CFighterCommon) {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("attack_air_lw") {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_WOLF_INSTANCE_WORK_ID_FLAG_SEARCH_HIT);
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

static OFFSET_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1, //.text:0000007100675A20                 SUB             SP, SP, #0xC0
    0xe8, 0x2b, 0x00, 0xfd, //.text:0000007100675A24                 STR             D8, [SP,#0xB0+var_60]
    0xfc, 0x6f, 0x06, 0xa9, //.text:0000007100675A28                 STP             X28, X27, [SP,#0xB0+var_50]
    0xfa, 0x67, 0x07, 0xa9, //.text:0000007100675A2C                 STP             X26, X25, [SP,#0xB0+var_40]
    0xf8, 0x5f, 0x08, 0xa9, //.text:0000007100675A30                 STP             X24, X23, [SP,#0xB0+var_30]
    0xf6, 0x57, 0x09, 0xa9, //.text:0000007100675A34                 STP             X22, X21, [SP,#0xB0+var_20]
    0xf4, 0x4f, 0x0a, 0xa9, //.text:0000007100675A38                 STP             X20, X19, [SP,#0xB0+var_10]
    0xfd, 0x7b, 0x0b, 0xa9, //.text:0000007100675A3C                 STP             X29, X30, [SP,#0xB0+var_s0]
    0xfd, 0xc3, 0x02, 0x91, //.text:0000007100675A40                 ADD             X29, SP, #0xB0
    0xfb, 0x03, 0x00, 0xaa  //.text:0000007100675A44                 MOV             X27, X0
];

unsafe extern "C" fn wolf_game_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 55, 0, 0, 40, 2.5, 0.0, 9.0, 2.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 55, 0, 0, 40, 2.8, 0.0, 9.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.0, 55, 0, 0, 40, 4.0, 0.0, 9.0, 11.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);

        //if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK){
            StatusModule::change_status_request_from_script(agent.module_accessor, FIGHTER_STATUS_KIND_ATTACK_S3.into(), false.into());
        //}
    }
}

pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, OFFSET_SEARCH_CODE) {
            WOLF_NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
    }

    Agent::new("wolf")
    .game_acmd("game_attackairlw", wolf_game_attackairlw)
    .game_acmd("game_attack13", wolf_game_attack13)
    .on_line(Main, wolf_frame)
    .install();

    skyline::install_hook!(
        wolf_notify_log_event_collision_hit_replace
    );
}
