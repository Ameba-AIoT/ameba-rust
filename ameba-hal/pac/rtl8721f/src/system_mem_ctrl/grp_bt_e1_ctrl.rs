#[doc = "Register `GRP_BT_E1_CTRL` reader"]
pub type R = crate::R<GrpBtE1CtrlSpec>;
#[doc = "Register `GRP_BT_E1_CTRL` writer"]
pub type W = crate::W<GrpBtE1CtrlSpec>;
#[doc = "Field `grp_bt_e1_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE1PwrActR = crate::FieldReader;
#[doc = "Field `grp_bt_e1_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE1PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_bt_e1_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE1PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_bt_e1_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpBtE1PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_bt_e1_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpBtE1LoopModeR = crate::BitReader;
#[doc = "Field `grp_bt_e1_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpBtE1LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_bt_e1_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpBtE1MselR = crate::FieldReader;
#[doc = "Field `grp_bt_e1_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpBtE1MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `bt_e1_dvse` reader - DVSE"]
pub type BtE1DvseR = crate::BitReader;
#[doc = "Field `bt_e1_dvse` writer - DVSE"]
pub type BtE1DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e1_dvs` reader - DVS"]
pub type BtE1DvsR = crate::FieldReader;
#[doc = "Field `bt_e1_dvs` writer - DVS"]
pub type BtE1DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `bt_e1_tmck` reader - TMCK"]
pub type BtE1TmckR = crate::BitReader;
#[doc = "Field `bt_e1_tmck` writer - TMCK"]
pub type BtE1TmckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e1_rae` reader - RAE"]
pub type BtE1RaeR = crate::BitReader;
#[doc = "Field `bt_e1_rae` writer - RAE"]
pub type BtE1RaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e1_wae` reader - WAE"]
pub type BtE1WaeR = crate::BitReader;
#[doc = "Field `bt_e1_wae` writer - WAE"]
pub type BtE1WaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bt_e1_waem` reader - WAEM"]
pub type BtE1WaemR = crate::FieldReader;
#[doc = "Field `bt_e1_waem` writer - WAEM"]
pub type BtE1WaemW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e1_pwr_act(&self) -> GrpBtE1PwrActR {
        GrpBtE1PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e1_pwr_lps(&self) -> GrpBtE1PwrLpsR {
        GrpBtE1PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_bt_e1_loop_mode(&self) -> GrpBtE1LoopModeR {
        GrpBtE1LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_bt_e1_msel(&self) -> GrpBtE1MselR {
        GrpBtE1MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn bt_e1_dvse(&self) -> BtE1DvseR {
        BtE1DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn bt_e1_dvs(&self) -> BtE1DvsR {
        BtE1DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn bt_e1_tmck(&self) -> BtE1TmckR {
        BtE1TmckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn bt_e1_rae(&self) -> BtE1RaeR {
        BtE1RaeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn bt_e1_wae(&self) -> BtE1WaeR {
        BtE1WaeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn bt_e1_waem(&self) -> BtE1WaemR {
        BtE1WaemR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e1_pwr_act(&mut self) -> GrpBtE1PwrActW<'_, GrpBtE1CtrlSpec> {
        GrpBtE1PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_bt_e1_pwr_lps(&mut self) -> GrpBtE1PwrLpsW<'_, GrpBtE1CtrlSpec> {
        GrpBtE1PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_bt_e1_loop_mode(&mut self) -> GrpBtE1LoopModeW<'_, GrpBtE1CtrlSpec> {
        GrpBtE1LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_bt_e1_msel(&mut self) -> GrpBtE1MselW<'_, GrpBtE1CtrlSpec> {
        GrpBtE1MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn bt_e1_dvse(&mut self) -> BtE1DvseW<'_, GrpBtE1CtrlSpec> {
        BtE1DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn bt_e1_dvs(&mut self) -> BtE1DvsW<'_, GrpBtE1CtrlSpec> {
        BtE1DvsW::new(self, 11)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn bt_e1_tmck(&mut self) -> BtE1TmckW<'_, GrpBtE1CtrlSpec> {
        BtE1TmckW::new(self, 15)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn bt_e1_rae(&mut self) -> BtE1RaeW<'_, GrpBtE1CtrlSpec> {
        BtE1RaeW::new(self, 16)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn bt_e1_wae(&mut self) -> BtE1WaeW<'_, GrpBtE1CtrlSpec> {
        BtE1WaeW::new(self, 17)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn bt_e1_waem(&mut self) -> BtE1WaemW<'_, GrpBtE1CtrlSpec> {
        BtE1WaemW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_bt_e1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_bt_e1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpBtE1CtrlSpec;
impl crate::RegisterSpec for GrpBtE1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_bt_e1_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpBtE1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_bt_e1_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpBtE1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_BT_E1_CTRL to value 0x002b_0310"]
impl crate::Resettable for GrpBtE1CtrlSpec {
    const RESET_VALUE: u32 = 0x002b_0310;
}
