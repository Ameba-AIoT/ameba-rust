#[doc = "Register `GRP_LRET_E0_CTRL` reader"]
pub type R = crate::R<GrpLretE0CtrlSpec>;
#[doc = "Register `GRP_LRET_E0_CTRL` writer"]
pub type W = crate::W<GrpLretE0CtrlSpec>;
#[doc = "Field `grp_lret_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLretE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_lret_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLretE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_lret_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLretE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_lret_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLretE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_lret_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpLretE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_lret_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpLretE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_lret_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpLretE0MselR = crate::FieldReader;
#[doc = "Field `grp_lret_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpLretE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lret_e0_dvse` reader - DVSE"]
pub type LretE0DvseR = crate::BitReader;
#[doc = "Field `lret_e0_dvse` writer - DVSE"]
pub type LretE0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lret_e0_dvs` reader - DVS"]
pub type LretE0DvsR = crate::FieldReader;
#[doc = "Field `lret_e0_dvs` writer - DVS"]
pub type LretE0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lret_e0_tmck` reader - TMCK"]
pub type LretE0TmckR = crate::BitReader;
#[doc = "Field `lret_e0_tmck` writer - TMCK"]
pub type LretE0TmckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lret_e0_rae` reader - RAE"]
pub type LretE0RaeR = crate::BitReader;
#[doc = "Field `lret_e0_rae` writer - RAE"]
pub type LretE0RaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lret_e0_wae` reader - WAE"]
pub type LretE0WaeR = crate::BitReader;
#[doc = "Field `lret_e0_wae` writer - WAE"]
pub type LretE0WaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lret_e0_waem` reader - WAEM"]
pub type LretE0WaemR = crate::FieldReader;
#[doc = "Field `lret_e0_waem` writer - WAEM"]
pub type LretE0WaemW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lret_e0_pwr_act(&self) -> GrpLretE0PwrActR {
        GrpLretE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lret_e0_pwr_lps(&self) -> GrpLretE0PwrLpsR {
        GrpLretE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_lret_e0_loop_mode(&self) -> GrpLretE0LoopModeR {
        GrpLretE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_lret_e0_msel(&self) -> GrpLretE0MselR {
        GrpLretE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn lret_e0_dvse(&self) -> LretE0DvseR {
        LretE0DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn lret_e0_dvs(&self) -> LretE0DvsR {
        LretE0DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn lret_e0_tmck(&self) -> LretE0TmckR {
        LretE0TmckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn lret_e0_rae(&self) -> LretE0RaeR {
        LretE0RaeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn lret_e0_wae(&self) -> LretE0WaeR {
        LretE0WaeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn lret_e0_waem(&self) -> LretE0WaemR {
        LretE0WaemR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lret_e0_pwr_act(&mut self) -> GrpLretE0PwrActW<'_, GrpLretE0CtrlSpec> {
        GrpLretE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lret_e0_pwr_lps(&mut self) -> GrpLretE0PwrLpsW<'_, GrpLretE0CtrlSpec> {
        GrpLretE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_lret_e0_loop_mode(&mut self) -> GrpLretE0LoopModeW<'_, GrpLretE0CtrlSpec> {
        GrpLretE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_lret_e0_msel(&mut self) -> GrpLretE0MselW<'_, GrpLretE0CtrlSpec> {
        GrpLretE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn lret_e0_dvse(&mut self) -> LretE0DvseW<'_, GrpLretE0CtrlSpec> {
        LretE0DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn lret_e0_dvs(&mut self) -> LretE0DvsW<'_, GrpLretE0CtrlSpec> {
        LretE0DvsW::new(self, 11)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn lret_e0_tmck(&mut self) -> LretE0TmckW<'_, GrpLretE0CtrlSpec> {
        LretE0TmckW::new(self, 15)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn lret_e0_rae(&mut self) -> LretE0RaeW<'_, GrpLretE0CtrlSpec> {
        LretE0RaeW::new(self, 16)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn lret_e0_wae(&mut self) -> LretE0WaeW<'_, GrpLretE0CtrlSpec> {
        LretE0WaeW::new(self, 17)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn lret_e0_waem(&mut self) -> LretE0WaemW<'_, GrpLretE0CtrlSpec> {
        LretE0WaemW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_lret_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_lret_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpLretE0CtrlSpec;
impl crate::RegisterSpec for GrpLretE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_lret_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpLretE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_lret_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpLretE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_LRET_E0_CTRL to value 0x002f_0310"]
impl crate::Resettable for GrpLretE0CtrlSpec {
    const RESET_VALUE: u32 = 0x002f_0310;
}
