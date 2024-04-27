//FUN_710000a720


/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_710000a720(L2CValue *param_1,void *boma,L2CValue *param_3)

{
  if GroundModule::is_touch(boma, param_3){
    stack160 = GroundModule::get_touch_normal(boma, param_3)
    param_2.Vector2__create(0xa0, 0x90);
    fVar10 = alStack128[0x18cdc1683];
    fVar11 = alStack128[0x1fbdb2615];
    fVar8 = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fVar9 = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
    angle = vec2_angle(fVar8,fVar9,fVar10,fVar11);
    fVar8 = 360.0;
    auStack256 = fVar8 + 90.0;
    math_rad(auStack256,plVar6);
    if auStack256 < fVar8{
        alStack320 = *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3;
        alStack336 = false;
        StatusModule::change_status_request_from_script(agent.module_accessor, 0xc0.into(), 0xb0.into());
        param1 = true;
    }
    if GroundModule::is_touch(boma, param_3){
        return;
    }
    param1 = false;
    return;
  }

    
    
      lib::L2CValue::L2CValue(param_1,true);
    }
    if (bVar1) {
      return;
    }
  }
  lib::L2CValue::L2CValue(param_1,false);
  return;
}

