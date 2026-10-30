#[doc = "Register `GRP_PPE_E1_CTRL` reader"]
pub type R = crate::R<GrpPpeE1CtrlSpec>;
#[doc = "Register `GRP_PPE_E1_CTRL` writer"]
pub type W = crate::W<GrpPpeE1CtrlSpec>;
#[doc = "Field `grp_ppe_e1_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPpeE1PwrActR = crate::FieldReader;
#[doc = "Field `grp_ppe_e1_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPpeE1PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_ppe_e1_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPpeE1PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_ppe_e1_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPpeE1PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_ppe_e1_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpPpeE1LoopModeR = crate::BitReader;
#[doc = "Field `grp_ppe_e1_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpPpeE1LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_ppe_e1_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpPpeE1MselR = crate::FieldReader;
#[doc = "Field `grp_ppe_e1_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpPpeE1MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppe_e1_rme` reader - RME"]
pub type PpeE1RmeR = crate::BitReader;
#[doc = "Field `ppe_e1_rme` writer - RME"]
pub type PpeE1RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ppe_e1_rm` reader - RM"]
pub type PpeE1RmR = crate::FieldReader;
#[doc = "Field `ppe_e1_rm` writer - RM"]
pub type PpeE1RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ppe_e1_test1` reader - TEST1"]
pub type PpeE1Test1R = crate::BitReader;
#[doc = "Field `ppe_e1_test1` writer - TEST1"]
pub type PpeE1Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_ppe_e1_pwr_act(&self) -> GrpPpeE1PwrActR {
        GrpPpeE1PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_ppe_e1_pwr_lps(&self) -> GrpPpeE1PwrLpsR {
        GrpPpeE1PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_ppe_e1_loop_mode(&self) -> GrpPpeE1LoopModeR {
        GrpPpeE1LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_ppe_e1_msel(&self) -> GrpPpeE1MselR {
        GrpPpeE1MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn ppe_e1_rme(&self) -> PpeE1RmeR {
        PpeE1RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn ppe_e1_rm(&self) -> PpeE1RmR {
        PpeE1RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn ppe_e1_test1(&self) -> PpeE1Test1R {
        PpeE1Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_ppe_e1_pwr_act(&mut self) -> GrpPpeE1PwrActW<'_, GrpPpeE1CtrlSpec> {
        GrpPpeE1PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_ppe_e1_pwr_lps(&mut self) -> GrpPpeE1PwrLpsW<'_, GrpPpeE1CtrlSpec> {
        GrpPpeE1PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_ppe_e1_loop_mode(&mut self) -> GrpPpeE1LoopModeW<'_, GrpPpeE1CtrlSpec> {
        GrpPpeE1LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_ppe_e1_msel(&mut self) -> GrpPpeE1MselW<'_, GrpPpeE1CtrlSpec> {
        GrpPpeE1MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn ppe_e1_rme(&mut self) -> PpeE1RmeW<'_, GrpPpeE1CtrlSpec> {
        PpeE1RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn ppe_e1_rm(&mut self) -> PpeE1RmW<'_, GrpPpeE1CtrlSpec> {
        PpeE1RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn ppe_e1_test1(&mut self) -> PpeE1Test1W<'_, GrpPpeE1CtrlSpec> {
        PpeE1Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_ppe_e1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_ppe_e1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpPpeE1CtrlSpec;
impl crate::RegisterSpec for GrpPpeE1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_ppe_e1_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpPpeE1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_ppe_e1_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpPpeE1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_PPE_E1_CTRL to value 0x1310"]
impl crate::Resettable for GrpPpeE1CtrlSpec {
    const RESET_VALUE: u32 = 0x1310;
}
