#[doc = "Register `GRP_BT_E0_CTRL` reader"]
pub type R = crate::R<GrpBtE0CtrlSpec>;
#[doc = "Register `GRP_BT_E0_CTRL` writer"]
pub type W = crate::W<GrpBtE0CtrlSpec>;
#[doc = "Field `grp_bt_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_bt_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_bt_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_bt_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_bt_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpBtE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_bt_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpBtE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_bt_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpBtE0MselR = crate::FieldReader;
#[doc = "Field `grp_bt_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpBtE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bt_e0_dvse` reader - DVSE"]
pub type BtE0DvseR = crate::BitReader;
#[doc = "Field `bt_e0_dvse` writer - DVSE"]
pub type BtE0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e0_dvs` reader - DVS"]
pub type BtE0DvsR = crate::FieldReader;
#[doc = "Field `bt_e0_dvs` writer - DVS"]
pub type BtE0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bt_e0_tmck` reader - TMCK"]
pub type BtE0TmckR = crate::BitReader;
#[doc = "Field `bt_e0_tmck` writer - TMCK"]
pub type BtE0TmckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e0_rae` reader - RAE"]
pub type BtE0RaeR = crate::BitReader;
#[doc = "Field `bt_e0_rae` writer - RAE"]
pub type BtE0RaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e0_wae` reader - WAE"]
pub type BtE0WaeR = crate::BitReader;
#[doc = "Field `bt_e0_wae` writer - WAE"]
pub type BtE0WaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e0_waem` reader - WAEM"]
pub type BtE0WaemR = crate::FieldReader;
#[doc = "Field `bt_e0_waem` writer - WAEM"]
pub type BtE0WaemW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e0_pwr_act(&self) -> GrpBtE0PwrActR {
        GrpBtE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e0_pwr_lps(&self) -> GrpBtE0PwrLpsR {
        GrpBtE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_bt_e0_loop_mode(&self) -> GrpBtE0LoopModeR {
        GrpBtE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_bt_e0_msel(&self) -> GrpBtE0MselR {
        GrpBtE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn bt_e0_dvse(&self) -> BtE0DvseR {
        BtE0DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn bt_e0_dvs(&self) -> BtE0DvsR {
        BtE0DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn bt_e0_tmck(&self) -> BtE0TmckR {
        BtE0TmckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn bt_e0_rae(&self) -> BtE0RaeR {
        BtE0RaeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn bt_e0_wae(&self) -> BtE0WaeR {
        BtE0WaeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn bt_e0_waem(&self) -> BtE0WaemR {
        BtE0WaemR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e0_pwr_act(&mut self) -> GrpBtE0PwrActW<'_, GrpBtE0CtrlSpec> {
        GrpBtE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e0_pwr_lps(&mut self) -> GrpBtE0PwrLpsW<'_, GrpBtE0CtrlSpec> {
        GrpBtE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_bt_e0_loop_mode(&mut self) -> GrpBtE0LoopModeW<'_, GrpBtE0CtrlSpec> {
        GrpBtE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_bt_e0_msel(&mut self) -> GrpBtE0MselW<'_, GrpBtE0CtrlSpec> {
        GrpBtE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn bt_e0_dvse(&mut self) -> BtE0DvseW<'_, GrpBtE0CtrlSpec> {
        BtE0DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn bt_e0_dvs(&mut self) -> BtE0DvsW<'_, GrpBtE0CtrlSpec> {
        BtE0DvsW::new(self, 11)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn bt_e0_tmck(&mut self) -> BtE0TmckW<'_, GrpBtE0CtrlSpec> {
        BtE0TmckW::new(self, 15)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn bt_e0_rae(&mut self) -> BtE0RaeW<'_, GrpBtE0CtrlSpec> {
        BtE0RaeW::new(self, 16)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn bt_e0_wae(&mut self) -> BtE0WaeW<'_, GrpBtE0CtrlSpec> {
        BtE0WaeW::new(self, 17)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn bt_e0_waem(&mut self) -> BtE0WaemW<'_, GrpBtE0CtrlSpec> {
        BtE0WaemW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_bt_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_bt_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpBtE0CtrlSpec;
impl crate::RegisterSpec for GrpBtE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_bt_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpBtE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_bt_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpBtE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_BT_E0_CTRL to value 0x002b_0310"]
impl crate::Resettable for GrpBtE0CtrlSpec {
    const RESET_VALUE: u32 = 0x002b_0310;
}
