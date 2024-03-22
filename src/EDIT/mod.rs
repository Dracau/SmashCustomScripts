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

// Game acmd script
unsafe extern "C" fn example_acmd_script(agent: &mut L2CAgentBase) {
    
}

// Char opff, Global opff
unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
    }
}

// Status script
unsafe extern "C" fn example_status_script(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
unsafe extern "C" fn ganon_game_attack11(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    
    damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 50);

    frame(lua_state, 3.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(lua_state, 8.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41,
         4.4,
          0.0, 12.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 5.0, 0.0, 12.0, 19.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 3.5, 0.0, 12.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);

        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR,
        0,
        Hash40::new("top"),
        20,
        0, 7, 0,
        0, 7, 8,
        3,
        5,
        135,
        false,
        1.0,
        *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    wait(lua_state, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.9);
}

unsafe extern "C" fn kirby_game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, // do not edit
        0, // id - 0
        0, // part - 1
        Hash40::new("toer"), // bone - 2
        200.0, // damage - 3
        4, // angle - 4
        5, 6, 7, // kbg, fkb, bkb - 5, 6, 7
        0,//3.7, // size - 8 
        4.3, 0.0, 0.0, // position - 9, 10, 11
        None, None, None, // position 2 - 12, 13, 14
        1.0, // hitlag - 15
        1.0, // sdi - 16
        *ATTACK_SETOFF_KIND_ON, // clang rebound - 17
        *ATTACK_LR_CHECK_POS, // facing restriction - 18
        false, // set weight - 19
        0, // shield damage - 20
        1.0, // trip chance - 21
        0, // rehit - 22
        false, // reflectable - 23
        false, // absorbable - 24
        false, // flinchless - 25
        false, // disable hitlag - 26
        true, // direct hitbox - 27
        *COLLISION_SITUATION_MASK_GA, // ground or air - 28
        *COLLISION_CATEGORY_MASK_ALL, // hitbits - 29
        *COLLISION_PART_MASK_ALL, // collision part - 30
        false, // friendly fire - 31
        Hash40::new("collision_attr_ice"), // effect - 32
        *ATTACK_SOUND_LEVEL_S, // sfx level - 33
        *COLLISION_SOUND_ATTR_KICK, // sfx type - 34
        *ATTACK_REGION_KICK); // type - 35
        
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }

    //shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);

    shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 1, Hash40::new("toer"), 30.5, 1.0, 0.0, -2.0, 1.0, 0.0, -2.0, 16.0, 4.0, false, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);

    /*shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR,
    0, //ID
    Hash40::new("toer"), //Bone
    10.0, //Size
    4.3, 0.0, 0.0, //Pos Start
    None, None, None, //Pos End
    5.0, //Damage mult
    0.1, //Speed mult
    200.0, //Max Damage
    false, // ??
    50.0, //Lifetime mult
    *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT); //Type*/

    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        //AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("ganon")
        .game_acmd("game_attack11", ganon_game_attack11).
        install();
    Agent::new("kirby")
        .game_acmd("game_attacklw3", kirby_game_attacklw3)
        .install();// Game acmd script
        //.on_line(Main, fighter_frame) // Char opff
        //.status(Main, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_CHARGE, example_status_script) // Status script
        //.install();
    /*Agent::new("fighter")
        .on_line(Main, fighter_frame) // Global opff
        .install();*/
}
