#[doc = "Register `GRP_PKE_E0_CTRL` reader"]
pub type R = crate::R<GrpPkeE0CtrlSpec>;
#[doc = "Register `GRP_PKE_E0_CTRL` writer"]
pub type W = crate::W<GrpPkeE0CtrlSpec>;
#[doc = "Field `grp_pke_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_pke_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_pke_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_pke_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpPkeE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_pke_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpPkeE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_pke_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpPkeE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_pke_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpPkeE0MselR = crate::FieldReader;
#[doc = "Field `grp_pke_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpPkeE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pke_e0_rmea` reader - RMEA"]
pub type PkeE0RmeaR = crate::BitReader;
#[doc = "Field `pke_e0_rmea` writer - RMEA"]
pub type PkeE0RmeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_e0_rmeb` reader - RMEB"]
pub type PkeE0RmebR = crate::BitReader;
#[doc = "Field `pke_e0_rmeb` writer - RMEB"]
pub type PkeE0RmebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_e0_rma` reader - RMA"]
pub type PkeE0RmaR = crate::FieldReader;
#[doc = "Field `pke_e0_rma` writer - RMA"]
pub type PkeE0RmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pke_e0_rmb` reader - RMB"]
pub type PkeE0RmbR = crate::FieldReader;
#[doc = "Field `pke_e0_rmb` writer - RMB"]
pub type PkeE0RmbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pke_e0_test1a` reader - TEST1A"]
pub type PkeE0Test1aR = crate::BitReader;
#[doc = "Field `pke_e0_test1a` writer - TEST1A"]
pub type PkeE0Test1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_e0_test1b` reader - TEST1B"]
pub type PkeE0Test1bR = crate::BitReader;
#[doc = "Field `pke_e0_test1b` writer - TEST1B"]
pub type PkeE0Test1bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e0_pwr_act(&self) -> GrpPkeE0PwrActR {
        GrpPkeE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e0_pwr_lps(&self) -> GrpPkeE0PwrLpsR {
        GrpPkeE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_pke_e0_loop_mode(&self) -> GrpPkeE0LoopModeR {
        GrpPkeE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_pke_e0_msel(&self) -> GrpPkeE0MselR {
        GrpPkeE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn pke_e0_rmea(&self) -> PkeE0RmeaR {
        PkeE0RmeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn pke_e0_rmeb(&self) -> PkeE0RmebR {
        PkeE0RmebR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn pke_e0_rma(&self) -> PkeE0RmaR {
        PkeE0RmaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn pke_e0_rmb(&self) -> PkeE0RmbR {
        PkeE0RmbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn pke_e0_test1a(&self) -> PkeE0Test1aR {
        PkeE0Test1aR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn pke_e0_test1b(&self) -> PkeE0Test1bR {
        PkeE0Test1bR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e0_pwr_act(&mut self) -> GrpPkeE0PwrActW<'_, GrpPkeE0CtrlSpec> {
        GrpPkeE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_pke_e0_pwr_lps(&mut self) -> GrpPkeE0PwrLpsW<'_, GrpPkeE0CtrlSpec> {
        GrpPkeE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_pke_e0_loop_mode(&mut self) -> GrpPkeE0LoopModeW<'_, GrpPkeE0CtrlSpec> {
        GrpPkeE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_pke_e0_msel(&mut self) -> GrpPkeE0MselW<'_, GrpPkeE0CtrlSpec> {
        GrpPkeE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn pke_e0_rmea(&mut self) -> PkeE0RmeaW<'_, GrpPkeE0CtrlSpec> {
        PkeE0RmeaW::new(self, 10)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn pke_e0_rmeb(&mut self) -> PkeE0RmebW<'_, GrpPkeE0CtrlSpec> {
        PkeE0RmebW::new(self, 11)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn pke_e0_rma(&mut self) -> PkeE0RmaW<'_, GrpPkeE0CtrlSpec> {
        PkeE0RmaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn pke_e0_rmb(&mut self) -> PkeE0RmbW<'_, GrpPkeE0CtrlSpec> {
        PkeE0RmbW::new(self, 16)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn pke_e0_test1a(&mut self) -> PkeE0Test1aW<'_, GrpPkeE0CtrlSpec> {
        PkeE0Test1aW::new(self, 20)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn pke_e0_test1b(&mut self) -> PkeE0Test1bW<'_, GrpPkeE0CtrlSpec> {
        PkeE0Test1bW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_pke_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_pke_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpPkeE0CtrlSpec;
impl crate::RegisterSpec for GrpPkeE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_pke_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpPkeE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_pke_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpPkeE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_PKE_E0_CTRL to value 0x0002_2310"]
impl crate::Resettable for GrpPkeE0CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_2310;
}
