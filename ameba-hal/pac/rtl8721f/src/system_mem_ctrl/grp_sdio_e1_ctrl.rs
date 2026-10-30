#[doc = "Register `GRP_SDIO_E1_CTRL` reader"]
pub type R = crate::R<GrpSdioE1CtrlSpec>;
#[doc = "Register `GRP_SDIO_E1_CTRL` writer"]
pub type W = crate::W<GrpSdioE1CtrlSpec>;
#[doc = "Field `grp_sdio_e1_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpSdioE1PwrActR = crate::FieldReader;
#[doc = "Field `grp_sdio_e1_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpSdioE1PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_sdio_e1_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpSdioE1PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_sdio_e1_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpSdioE1PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_sdio_e1_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpSdioE1LoopModeR = crate::BitReader;
#[doc = "Field `grp_sdio_e1_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpSdioE1LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_sdio_e1_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpSdioE1MselR = crate::FieldReader;
#[doc = "Field `grp_sdio_e1_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpSdioE1MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sdio_e1_rmea` reader - RMEA"]
pub type SdioE1RmeaR = crate::BitReader;
#[doc = "Field `sdio_e1_rmea` writer - RMEA"]
pub type SdioE1RmeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_e1_rmeb` reader - RMEB"]
pub type SdioE1RmebR = crate::BitReader;
#[doc = "Field `sdio_e1_rmeb` writer - RMEB"]
pub type SdioE1RmebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_e1_rma` reader - RMA"]
pub type SdioE1RmaR = crate::FieldReader;
#[doc = "Field `sdio_e1_rma` writer - RMA"]
pub type SdioE1RmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sdio_e1_rmb` reader - RMB"]
pub type SdioE1RmbR = crate::FieldReader;
#[doc = "Field `sdio_e1_rmb` writer - RMB"]
pub type SdioE1RmbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sdio_e1_test1a` reader - TEST1A"]
pub type SdioE1Test1aR = crate::BitReader;
#[doc = "Field `sdio_e1_test1a` writer - TEST1A"]
pub type SdioE1Test1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_e1_test1b` reader - TEST1B"]
pub type SdioE1Test1bR = crate::BitReader;
#[doc = "Field `sdio_e1_test1b` writer - TEST1B"]
pub type SdioE1Test1bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_sdio_e1_pwr_act(&self) -> GrpSdioE1PwrActR {
        GrpSdioE1PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_sdio_e1_pwr_lps(&self) -> GrpSdioE1PwrLpsR {
        GrpSdioE1PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_sdio_e1_loop_mode(&self) -> GrpSdioE1LoopModeR {
        GrpSdioE1LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_sdio_e1_msel(&self) -> GrpSdioE1MselR {
        GrpSdioE1MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn sdio_e1_rmea(&self) -> SdioE1RmeaR {
        SdioE1RmeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn sdio_e1_rmeb(&self) -> SdioE1RmebR {
        SdioE1RmebR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn sdio_e1_rma(&self) -> SdioE1RmaR {
        SdioE1RmaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn sdio_e1_rmb(&self) -> SdioE1RmbR {
        SdioE1RmbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn sdio_e1_test1a(&self) -> SdioE1Test1aR {
        SdioE1Test1aR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn sdio_e1_test1b(&self) -> SdioE1Test1bR {
        SdioE1Test1bR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_sdio_e1_pwr_act(&mut self) -> GrpSdioE1PwrActW<'_, GrpSdioE1CtrlSpec> {
        GrpSdioE1PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_sdio_e1_pwr_lps(&mut self) -> GrpSdioE1PwrLpsW<'_, GrpSdioE1CtrlSpec> {
        GrpSdioE1PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_sdio_e1_loop_mode(&mut self) -> GrpSdioE1LoopModeW<'_, GrpSdioE1CtrlSpec> {
        GrpSdioE1LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_sdio_e1_msel(&mut self) -> GrpSdioE1MselW<'_, GrpSdioE1CtrlSpec> {
        GrpSdioE1MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn sdio_e1_rmea(&mut self) -> SdioE1RmeaW<'_, GrpSdioE1CtrlSpec> {
        SdioE1RmeaW::new(self, 10)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn sdio_e1_rmeb(&mut self) -> SdioE1RmebW<'_, GrpSdioE1CtrlSpec> {
        SdioE1RmebW::new(self, 11)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn sdio_e1_rma(&mut self) -> SdioE1RmaW<'_, GrpSdioE1CtrlSpec> {
        SdioE1RmaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn sdio_e1_rmb(&mut self) -> SdioE1RmbW<'_, GrpSdioE1CtrlSpec> {
        SdioE1RmbW::new(self, 16)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn sdio_e1_test1a(&mut self) -> SdioE1Test1aW<'_, GrpSdioE1CtrlSpec> {
        SdioE1Test1aW::new(self, 20)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn sdio_e1_test1b(&mut self) -> SdioE1Test1bW<'_, GrpSdioE1CtrlSpec> {
        SdioE1Test1bW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_sdio_e1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_sdio_e1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpSdioE1CtrlSpec;
impl crate::RegisterSpec for GrpSdioE1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_sdio_e1_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpSdioE1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_sdio_e1_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpSdioE1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_SDIO_E1_CTRL to value 0x0002_2310"]
impl crate::Resettable for GrpSdioE1CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_2310;
}
