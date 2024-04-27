SpecialHi2_main_loop


/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CFighterPsykittytwo::status::SpecialHi2_main_loop
          (L2CFighterPsykittytwo *this,L2CValue *return_value)

{
  byte bVar1;
  int iVar2;
  uint uVar3;
  GroundCorrectKind GVar4;
  ulong uVar5;
  ulong uVar6;
  L2CValue *pLVar7;
  L2CValue *pLVar8;
  L2CValue aLStack208 [16];
  L2CValue aLStack192 [16];
  L2CValue aLStack176 [16];
  L2CValue aLStack160 [16];
  L2CValue aLStack144 [16];
  L2CValue aLStack128 [16];
  L2CValue LStack112;
  L2CValue LStack96;
  L2CValue LStack80;
  

if fighter.sub_transition_group_check_air_cliff().get_bool(){
    let iVar2 = 1;
    //goto LAB_710000a324
}

if 6 > WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_FRAME){
    Stack112 = *GROUND_TOUCH_FLAG_ALL;
    Stack96 = GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL)
    if Stack96 {
        Stack112 = *GROUND_TOUCH_FLAG_UP;
    }
}

      lib::L2CValue::L2CValue((L2CValue *)&LStack112,_GROUND_TOUCH_FLAG_UP);
      FUN_710000a720(&LStack96,this,&LStack112);
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
      uVar5 = lib::L2CValue::operator==((L2CValue *)&LStack96,(L2CValue *)&LStack80);
      if ((uVar5 & 1) == 0) {
        lib::L2CValue::L2CValue(aLStack144,GROUND_TOUCH_FLAG_DOWN);
        FUN_710000a720(aLStack128,this,aLStack144);
        lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
        uVar5 = lib::L2CValue::operator==(aLStack128,(L2CValue *)&LStack80);
        if ((uVar5 & 1) == 0) {
          lib::L2CValue::L2CValue(aLStack176,_GROUND_TOUCH_FLAG_LEFT);
          FUN_710000a720(aLStack160,this,aLStack176);
          lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
          uVar5 = lib::L2CValue::operator==(aLStack160,(L2CValue *)&LStack80);
          lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
          if ((uVar5 & 1) == 0) {
            lib::L2CValue::L2CValue(aLStack208,GROUND_TOUCH_FLAG_RIGHT);
            FUN_710000a720(aLStack192,this,aLStack208);
            lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
            uVar5 = lib::L2CValue::operator==(aLStack192,(L2CValue *)&LStack80);
            if ((uVar5 & 1) == 0) goto LAB_710000a3c4;
            goto LAB_710000a31c;
          }
        }
      }
      pLVar7 = &LStack112;
      goto LAB_710000a318;
    }
LAB_710000a3c4:
    bVar1 = app::lua_bind::StatusModule__is_changing_impl(this->moduleAccessor);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,(bool)(bVar1 & 1));
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
    uVar5 = lib::L2CValue::operator==((L2CValue *)&LStack96,(L2CValue *)&LStack80);
    if ((uVar5 & 1) != 0) {
LAB_710000a40c:
      pLVar8 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)&this->globalTable,0x16);
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
      uVar5 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack80);
      if ((uVar5 & 1) == 0) {
        lib::L2CValue::L2CValue((L2CValue *)&LStack80,GROUND_CORRECT_KIND_AIR);
        GVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack80);
        app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar4);
      }
      else {
        lib::L2CValue::L2CValue((L2CValue *)&LStack80,GROUND_CORRECT_KIND_GROUND);
        GVar4 = lib::L2CValue::as_integer((L2CValue *)&LStack80);
        app::lua_bind::GroundModule__correct_impl(this->moduleAccessor,GVar4);
      }
      goto LAB_710000a1b4;
    }
    pLVar7 = &this->globalTable;
    pLVar8 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar7,0x17);
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack80);
    if ((uVar5 & 1) != 0) {
      pLVar8 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar7,0x16);
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,SITUATION_KIND_AIR);
      uVar5 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack80);
      if ((uVar5 & 1) != 0) {
        goto LAB_710000a40c;
      }
    }
    pLVar8 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar7,0x17);
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack80);
    if ((uVar5 & 1) != 0) {
      pLVar7 = &LStack96;
      goto LAB_710000a318;
    }
    pLVar8 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar7,0x16);
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND);
    uVar5 = lib::L2CValue::operator==(pLVar8,(L2CValue *)&LStack80);
    if ((uVar5 & 1) != 0) goto LAB_710000a40c;
  }
  else {
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3);
    lib::L2CValue::L2CValue((L2CValue *)&LStack96,false);
    lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0xb0,(L2CValue)0xa0);
LAB_710000a1b4:
    pLVar7 = &LStack80;
LAB_710000a318:
  }
LAB_710000a31c:
  iVar2 = 0;
LAB_710000a324:
  lib::L2CValue::L2CValue((L2CValue *)return_value,iVar2);
  return;
}

