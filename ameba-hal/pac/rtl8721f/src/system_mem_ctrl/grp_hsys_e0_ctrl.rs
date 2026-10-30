#[doc = "Register `GRP_HSYS_E0_CTRL` reader"]
pub type R = crate::R<GrpHsysE0CtrlSpec>;
#[doc = "Register `GRP_HSYS_E0_CTRL` writer"]
pub type W = crate::W<GrpHsysE0CtrlSpec>;
#[doc = "Field `grp_hsys_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpHsysE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_hsys_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpHsysE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_hsys_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpHsysE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_hsys_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpHsysE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_hsys_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpHsysE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_hsys_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpHsysE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_hsys_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpHsysE0MselR = crate::FieldReader;
#[doc = "Field `grp_hsys_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpHsysE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hsys_e0_dvse` reader - DVSE"]
pub type HsysE0DvseR = crate::BitReader;
#[doc = "Field `hsys_e0_dvse` writer - DVSE"]
pub type HsysE0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsys_e0_dvs` reader - DVS"]
pub type HsysE0DvsR = crate::FieldReader;
#[doc = "Field `hsys_e0_dvs` writer - DVS"]
pub type HsysE0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hsys_e0_tmck` reader - TMCK"]
pub type HsysE0TmckR = crate::BitReader;
#[doc = "Field `hsys_e0_tmck` writer - TMCK"]
pub type HsysE0TmckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsys_e0_rae` reader - RAE"]
pub type HsysE0RaeR = crate::BitReader;
#[doc = "Field `hsys_e0_rae` writer - RAE"]
pub type HsysE0RaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsys_e0_wae` reader - WAE"]
pub type HsysE0WaeR = crate::BitReader;
#[doc = "Field `hsys_e0_wae` writer - WAE"]
pub type HsysE0WaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsys_e0_waem` reader - WAEM"]
pub type HsysE0WaemR = crate::FieldReader;
#[doc = "Field `hsys_e0_waem` writer - WAEM"]
pub type HsysE0WaemW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_hsys_e0_pwr_act(&self) -> GrpHsysE0PwrActR {
        GrpHsysE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_hsys_e0_pwr_lps(&self) -> GrpHsysE0PwrLpsR {
        GrpHsysE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_hsys_e0_loop_mode(&self) -> GrpHsysE0LoopModeR {
        GrpHsysE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_hsys_e0_msel(&self) -> GrpHsysE0MselR {
        GrpHsysE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn hsys_e0_dvse(&self) -> HsysE0DvseR {
        HsysE0DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn hsys_e0_dvs(&self) -> HsysE0DvsR {
        HsysE0DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn hsys_e0_tmck(&self) -> HsysE0TmckR {
        HsysE0TmckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn hsys_e0_rae(&self) -> HsysE0RaeR {
        HsysE0RaeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn hsys_e0_wae(&self) -> HsysE0WaeR {
        HsysE0WaeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn hsys_e0_waem(&self) -> HsysE0WaemR {
        HsysE0WaemR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_hsys_e0_pwr_act(&mut self) -> GrpHsysE0PwrActW<'_, GrpHsysE0CtrlSpec> {
        GrpHsysE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_hsys_e0_pwr_lps(&mut self) -> GrpHsysE0PwrLpsW<'_, GrpHsysE0CtrlSpec> {
        GrpHsysE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_hsys_e0_loop_mode(&mut self) -> GrpHsysE0LoopModeW<'_, GrpHsysE0CtrlSpec> {
        GrpHsysE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_hsys_e0_msel(&mut self) -> GrpHsysE0MselW<'_, GrpHsysE0CtrlSpec> {
        GrpHsysE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn hsys_e0_dvse(&mut self) -> HsysE0DvseW<'_, GrpHsysE0CtrlSpec> {
        HsysE0DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn hsys_e0_dvs(&mut self) -> HsysE0DvsW<'_, GrpHsysE0CtrlSpec> {
        HsysE0DvsW::new(self, 11)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn hsys_e0_tmck(&mut self) -> HsysE0TmckW<'_, GrpHsysE0CtrlSpec> {
        HsysE0TmckW::new(self, 15)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn hsys_e0_rae(&mut self) -> HsysE0RaeW<'_, GrpHsysE0CtrlSpec> {
        HsysE0RaeW::new(self, 16)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn hsys_e0_wae(&mut self) -> HsysE0WaeW<'_, GrpHsysE0CtrlSpec> {
        HsysE0WaeW::new(self, 17)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn hsys_e0_waem(&mut self) -> HsysE0WaemW<'_, GrpHsysE0CtrlSpec> {
        HsysE0WaemW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_hsys_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_hsys_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpHsysE0CtrlSpec;
impl crate::RegisterSpec for GrpHsysE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_hsys_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpHsysE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_hsys_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpHsysE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_HSYS_E0_CTRL to value 0x002b_0310"]
impl crate::Resettable for GrpHsysE0CtrlSpec {
    const RESET_VALUE: u32 = 0x002b_0310;
}
