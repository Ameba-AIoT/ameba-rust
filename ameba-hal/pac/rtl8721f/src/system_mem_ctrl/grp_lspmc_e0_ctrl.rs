#[doc = "Register `GRP_LSPMC_E0_CTRL` reader"]
pub type R = crate::R<GrpLspmcE0CtrlSpec>;
#[doc = "Register `GRP_LSPMC_E0_CTRL` writer"]
pub type W = crate::W<GrpLspmcE0CtrlSpec>;
#[doc = "Field `grp_lspmc_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLspmcE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_lspmc_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLspmcE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_lspmc_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLspmcE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_lspmc_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLspmcE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_lspmc_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpLspmcE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_lspmc_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpLspmcE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_lspmc_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpLspmcE0MselR = crate::FieldReader;
#[doc = "Field `grp_lspmc_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpLspmcE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lspmc_e0_dvse` reader - DVSE"]
pub type LspmcE0DvseR = crate::BitReader;
#[doc = "Field `lspmc_e0_dvse` writer - DVSE"]
pub type LspmcE0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lspmc_e0_dvs` reader - DVS"]
pub type LspmcE0DvsR = crate::FieldReader;
#[doc = "Field `lspmc_e0_dvs` writer - DVS"]
pub type LspmcE0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lspmc_e0_pwr_act(&self) -> GrpLspmcE0PwrActR {
        GrpLspmcE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lspmc_e0_pwr_lps(&self) -> GrpLspmcE0PwrLpsR {
        GrpLspmcE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_lspmc_e0_loop_mode(&self) -> GrpLspmcE0LoopModeR {
        GrpLspmcE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_lspmc_e0_msel(&self) -> GrpLspmcE0MselR {
        GrpLspmcE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn lspmc_e0_dvse(&self) -> LspmcE0DvseR {
        LspmcE0DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn lspmc_e0_dvs(&self) -> LspmcE0DvsR {
        LspmcE0DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lspmc_e0_pwr_act(&mut self) -> GrpLspmcE0PwrActW<'_, GrpLspmcE0CtrlSpec> {
        GrpLspmcE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lspmc_e0_pwr_lps(&mut self) -> GrpLspmcE0PwrLpsW<'_, GrpLspmcE0CtrlSpec> {
        GrpLspmcE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_lspmc_e0_loop_mode(&mut self) -> GrpLspmcE0LoopModeW<'_, GrpLspmcE0CtrlSpec> {
        GrpLspmcE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_lspmc_e0_msel(&mut self) -> GrpLspmcE0MselW<'_, GrpLspmcE0CtrlSpec> {
        GrpLspmcE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn lspmc_e0_dvse(&mut self) -> LspmcE0DvseW<'_, GrpLspmcE0CtrlSpec> {
        LspmcE0DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn lspmc_e0_dvs(&mut self) -> LspmcE0DvsW<'_, GrpLspmcE0CtrlSpec> {
        LspmcE0DvsW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_lspmc_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_lspmc_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpLspmcE0CtrlSpec;
impl crate::RegisterSpec for GrpLspmcE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_lspmc_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpLspmcE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_lspmc_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpLspmcE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_LSPMC_E0_CTRL to value 0x1b10"]
impl crate::Resettable for GrpLspmcE0CtrlSpec {
    const RESET_VALUE: u32 = 0x1b10;
}
