#[doc = "Register `GRP_GMAC_E0_CTRL` reader"]
pub type R = crate::R<GrpGmacE0CtrlSpec>;
#[doc = "Register `GRP_GMAC_E0_CTRL` writer"]
pub type W = crate::W<GrpGmacE0CtrlSpec>;
#[doc = "Field `grp_gmac_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpGmacE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_gmac_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpGmacE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_gmac_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpGmacE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_gmac_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpGmacE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_gmac_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpGmacE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_gmac_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpGmacE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_gmac_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpGmacE0MselR = crate::FieldReader;
#[doc = "Field `grp_gmac_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpGmacE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac_e0_rmea` reader - RMEA"]
pub type GmacE0RmeaR = crate::BitReader;
#[doc = "Field `gmac_e0_rmea` writer - RMEA"]
pub type GmacE0RmeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac_e0_rmeb` reader - RMEB"]
pub type GmacE0RmebR = crate::BitReader;
#[doc = "Field `gmac_e0_rmeb` writer - RMEB"]
pub type GmacE0RmebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac_e0_rma` reader - RMA"]
pub type GmacE0RmaR = crate::FieldReader;
#[doc = "Field `gmac_e0_rma` writer - RMA"]
pub type GmacE0RmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gmac_e0_rmb` reader - RMB"]
pub type GmacE0RmbR = crate::FieldReader;
#[doc = "Field `gmac_e0_rmb` writer - RMB"]
pub type GmacE0RmbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gmac_e0_test1a` reader - TEST1A"]
pub type GmacE0Test1aR = crate::BitReader;
#[doc = "Field `gmac_e0_test1a` writer - TEST1A"]
pub type GmacE0Test1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmac_e0_test1b` reader - TEST1B"]
pub type GmacE0Test1bR = crate::BitReader;
#[doc = "Field `gmac_e0_test1b` writer - TEST1B"]
pub type GmacE0Test1bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_gmac_e0_pwr_act(&self) -> GrpGmacE0PwrActR {
        GrpGmacE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_gmac_e0_pwr_lps(&self) -> GrpGmacE0PwrLpsR {
        GrpGmacE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_gmac_e0_loop_mode(&self) -> GrpGmacE0LoopModeR {
        GrpGmacE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_gmac_e0_msel(&self) -> GrpGmacE0MselR {
        GrpGmacE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn gmac_e0_rmea(&self) -> GmacE0RmeaR {
        GmacE0RmeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn gmac_e0_rmeb(&self) -> GmacE0RmebR {
        GmacE0RmebR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn gmac_e0_rma(&self) -> GmacE0RmaR {
        GmacE0RmaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn gmac_e0_rmb(&self) -> GmacE0RmbR {
        GmacE0RmbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn gmac_e0_test1a(&self) -> GmacE0Test1aR {
        GmacE0Test1aR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn gmac_e0_test1b(&self) -> GmacE0Test1bR {
        GmacE0Test1bR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_gmac_e0_pwr_act(&mut self) -> GrpGmacE0PwrActW<'_, GrpGmacE0CtrlSpec> {
        GrpGmacE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_gmac_e0_pwr_lps(&mut self) -> GrpGmacE0PwrLpsW<'_, GrpGmacE0CtrlSpec> {
        GrpGmacE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_gmac_e0_loop_mode(&mut self) -> GrpGmacE0LoopModeW<'_, GrpGmacE0CtrlSpec> {
        GrpGmacE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_gmac_e0_msel(&mut self) -> GrpGmacE0MselW<'_, GrpGmacE0CtrlSpec> {
        GrpGmacE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn gmac_e0_rmea(&mut self) -> GmacE0RmeaW<'_, GrpGmacE0CtrlSpec> {
        GmacE0RmeaW::new(self, 10)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn gmac_e0_rmeb(&mut self) -> GmacE0RmebW<'_, GrpGmacE0CtrlSpec> {
        GmacE0RmebW::new(self, 11)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn gmac_e0_rma(&mut self) -> GmacE0RmaW<'_, GrpGmacE0CtrlSpec> {
        GmacE0RmaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn gmac_e0_rmb(&mut self) -> GmacE0RmbW<'_, GrpGmacE0CtrlSpec> {
        GmacE0RmbW::new(self, 16)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn gmac_e0_test1a(&mut self) -> GmacE0Test1aW<'_, GrpGmacE0CtrlSpec> {
        GmacE0Test1aW::new(self, 20)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn gmac_e0_test1b(&mut self) -> GmacE0Test1bW<'_, GrpGmacE0CtrlSpec> {
        GmacE0Test1bW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_gmac_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_gmac_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpGmacE0CtrlSpec;
impl crate::RegisterSpec for GrpGmacE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_gmac_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpGmacE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_gmac_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpGmacE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_GMAC_E0_CTRL to value 0x0002_2310"]
impl crate::Resettable for GrpGmacE0CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_2310;
}
