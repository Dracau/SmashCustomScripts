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

unsafe extern "C" fn donkey_frame_sizemanagement(agent: &mut L2CAgentBase) {

    if StatusModule::status_kind(agent.module_accessor) == FIGHTER_STATUS_KIND_DEAD{
        return;
    }

    let currentSize = PostureModule::scale(agent.module_accessor);

    let maxSize = 1.45;
    let minSize = 0.65;

    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
        PostureModule::set_scale(agent.module_accessor, (currentSize*1.04).clamp(currentSize, maxSize), false)
    }
    else if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
        PostureModule::set_scale(agent.module_accessor, (currentSize/1.04).clamp(minSize, currentSize), false)
    }
}

pub fn install() {
    Agent::new("donkey")
        .on_line(Main, donkey_frame_sizemanagement)
        .install();
}
