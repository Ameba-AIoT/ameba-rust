#[doc = "Register `GRP_AONPMC_E0_CTRL` reader"]
pub type R = crate::R<GrpAonpmcE0CtrlSpec>;
#[doc = "Register `GRP_AONPMC_E0_CTRL` writer"]
pub type W = crate::W<GrpAonpmcE0CtrlSpec>;
#[doc = "Field `grp_aonpmc_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpAonpmcE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_aonpmc_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpAonpmcE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_aonpmc_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpAonpmcE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_aonpmc_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpAonpmcE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_aonpmc_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpAonpmcE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_aonpmc_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpAonpmcE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_aonpmc_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpAonpmcE0MselR = crate::FieldReader;
#[doc = "Field `grp_aonpmc_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpAonpmcE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aonpmc_e0_dvse` reader - DVSE"]
pub type AonpmcE0DvseR = crate::BitReader;
#[doc = "Field `aonpmc_e0_dvse` writer - DVSE"]
pub type AonpmcE0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aonpmc_e0_dvs` reader - DVS"]
pub type AonpmcE0DvsR = crate::FieldReader;
#[doc = "Field `aonpmc_e0_dvs` writer - DVS"]
pub type AonpmcE0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_pwr_act(&self) -> GrpAonpmcE0PwrActR {
        GrpAonpmcE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_pwr_lps(&self) -> GrpAonpmcE0PwrLpsR {
        GrpAonpmcE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_loop_mode(&self) -> GrpAonpmcE0LoopModeR {
        GrpAonpmcE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_msel(&self) -> GrpAonpmcE0MselR {
        GrpAonpmcE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn aonpmc_e0_dvse(&self) -> AonpmcE0DvseR {
        AonpmcE0DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn aonpmc_e0_dvs(&self) -> AonpmcE0DvsR {
        AonpmcE0DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_pwr_act(&mut self) -> GrpAonpmcE0PwrActW<'_, GrpAonpmcE0CtrlSpec> {
        GrpAonpmcE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_pwr_lps(&mut self) -> GrpAonpmcE0PwrLpsW<'_, GrpAonpmcE0CtrlSpec> {
        GrpAonpmcE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_loop_mode(&mut self) -> GrpAonpmcE0LoopModeW<'_, GrpAonpmcE0CtrlSpec> {
        GrpAonpmcE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_aonpmc_e0_msel(&mut self) -> GrpAonpmcE0MselW<'_, GrpAonpmcE0CtrlSpec> {
        GrpAonpmcE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn aonpmc_e0_dvse(&mut self) -> AonpmcE0DvseW<'_, GrpAonpmcE0CtrlSpec> {
        AonpmcE0DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn aonpmc_e0_dvs(&mut self) -> AonpmcE0DvsW<'_, GrpAonpmcE0CtrlSpec> {
        AonpmcE0DvsW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_aonpmc_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_aonpmc_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpAonpmcE0CtrlSpec;
impl crate::RegisterSpec for GrpAonpmcE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_aonpmc_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpAonpmcE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_aonpmc_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpAonpmcE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_AONPMC_E0_CTRL to value 0x1b10"]
impl crate::Resettable for GrpAonpmcE0CtrlSpec {
    const RESET_VALUE: u32 = 0x1b10;
}
