#[doc = "Register `GRP_PKE_E1_CTRL` reader"]
pub type R = crate::R<GrpPkeE1CtrlSpec>;
#[doc = "Register `GRP_PKE_E1_CTRL` writer"]
pub type W = crate::W<GrpPkeE1CtrlSpec>;
#[doc = "Field `grp_pke_e1_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE1PwrActR = crate::FieldReader;
#[doc = "Field `grp_pke_e1_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE1PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_pke_e1_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE1PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_pke_e1_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE1PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_pke_e1_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpPkeE1LoopModeR = crate::BitReader;
#[doc = "Field `grp_pke_e1_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpPkeE1LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pke_e1_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpPkeE1MselR = crate::FieldReader;
#[doc = "Field `grp_pke_e1_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpPkeE1MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pke_e1_dvse` reader - DVSE"]
pub type PkeE1DvseR = crate::BitReader;
#[doc = "Field `pke_e1_dvse` writer - DVSE"]
pub type PkeE1DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_e1_dvs` reader - DVS"]
pub type PkeE1DvsR = crate::FieldReader;
#[doc = "Field `pke_e1_dvs` writer - DVS"]
pub type PkeE1DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e1_pwr_act(&self) -> GrpPkeE1PwrActR {
        GrpPkeE1PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e1_pwr_lps(&self) -> GrpPkeE1PwrLpsR {
        GrpPkeE1PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_pke_e1_loop_mode(&self) -> GrpPkeE1LoopModeR {
        GrpPkeE1LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_pke_e1_msel(&self) -> GrpPkeE1MselR {
        GrpPkeE1MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn pke_e1_dvse(&self) -> PkeE1DvseR {
        PkeE1DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn pke_e1_dvs(&self) -> PkeE1DvsR {
        PkeE1DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e1_pwr_act(&mut self) -> GrpPkeE1PwrActW<'_, GrpPkeE1CtrlSpec> {
        GrpPkeE1PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e1_pwr_lps(&mut self) -> GrpPkeE1PwrLpsW<'_, GrpPkeE1CtrlSpec> {
        GrpPkeE1PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_pke_e1_loop_mode(&mut self) -> GrpPkeE1LoopModeW<'_, GrpPkeE1CtrlSpec> {
        GrpPkeE1LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_pke_e1_msel(&mut self) -> GrpPkeE1MselW<'_, GrpPkeE1CtrlSpec> {
        GrpPkeE1MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn pke_e1_dvse(&mut self) -> PkeE1DvseW<'_, GrpPkeE1CtrlSpec> {
        PkeE1DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn pke_e1_dvs(&mut self) -> PkeE1DvsW<'_, GrpPkeE1CtrlSpec> {
        PkeE1DvsW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_pke_e1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_pke_e1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpPkeE1CtrlSpec;
impl crate::RegisterSpec for GrpPkeE1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_pke_e1_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpPkeE1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_pke_e1_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpPkeE1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_PKE_E1_CTRL to value 0x2b10"]
impl crate::Resettable for GrpPkeE1CtrlSpec {
    const RESET_VALUE: u32 = 0x2b10;
}
