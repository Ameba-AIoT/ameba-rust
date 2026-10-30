#[doc = "Register `GRP_LCDC_E0_CTRL` reader"]
pub type R = crate::R<GrpLcdcE0CtrlSpec>;
#[doc = "Register `GRP_LCDC_E0_CTRL` writer"]
pub type W = crate::W<GrpLcdcE0CtrlSpec>;
#[doc = "Field `grp_lcdc_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLcdcE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_lcdc_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLcdcE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_lcdc_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLcdcE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_lcdc_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpLcdcE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_lcdc_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpLcdcE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_lcdc_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpLcdcE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_lcdc_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpLcdcE0MselR = crate::FieldReader;
#[doc = "Field `grp_lcdc_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpLcdcE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `lcdc_e0_rmea` reader - RMEA"]
pub type LcdcE0RmeaR = crate::BitReader;
#[doc = "Field `lcdc_e0_rmea` writer - RMEA"]
pub type LcdcE0RmeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcdc_e0_rmeb` reader - RMEB"]
pub type LcdcE0RmebR = crate::BitReader;
#[doc = "Field `lcdc_e0_rmeb` writer - RMEB"]
pub type LcdcE0RmebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcdc_e0_rma` reader - RMA"]
pub type LcdcE0RmaR = crate::FieldReader;
#[doc = "Field `lcdc_e0_rma` writer - RMA"]
pub type LcdcE0RmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lcdc_e0_rmb` reader - RMB"]
pub type LcdcE0RmbR = crate::FieldReader;
#[doc = "Field `lcdc_e0_rmb` writer - RMB"]
pub type LcdcE0RmbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `lcdc_e0_test1a` reader - TEST1A"]
pub type LcdcE0Test1aR = crate::BitReader;
#[doc = "Field `lcdc_e0_test1a` writer - TEST1A"]
pub type LcdcE0Test1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lcdc_e0_test1b` reader - TEST1B"]
pub type LcdcE0Test1bR = crate::BitReader;
#[doc = "Field `lcdc_e0_test1b` writer - TEST1B"]
pub type LcdcE0Test1bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lcdc_e0_pwr_act(&self) -> GrpLcdcE0PwrActR {
        GrpLcdcE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lcdc_e0_pwr_lps(&self) -> GrpLcdcE0PwrLpsR {
        GrpLcdcE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_lcdc_e0_loop_mode(&self) -> GrpLcdcE0LoopModeR {
        GrpLcdcE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_lcdc_e0_msel(&self) -> GrpLcdcE0MselR {
        GrpLcdcE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn lcdc_e0_rmea(&self) -> LcdcE0RmeaR {
        LcdcE0RmeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn lcdc_e0_rmeb(&self) -> LcdcE0RmebR {
        LcdcE0RmebR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn lcdc_e0_rma(&self) -> LcdcE0RmaR {
        LcdcE0RmaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn lcdc_e0_rmb(&self) -> LcdcE0RmbR {
        LcdcE0RmbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn lcdc_e0_test1a(&self) -> LcdcE0Test1aR {
        LcdcE0Test1aR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn lcdc_e0_test1b(&self) -> LcdcE0Test1bR {
        LcdcE0Test1bR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lcdc_e0_pwr_act(&mut self) -> GrpLcdcE0PwrActW<'_, GrpLcdcE0CtrlSpec> {
        GrpLcdcE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_lcdc_e0_pwr_lps(&mut self) -> GrpLcdcE0PwrLpsW<'_, GrpLcdcE0CtrlSpec> {
        GrpLcdcE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_lcdc_e0_loop_mode(&mut self) -> GrpLcdcE0LoopModeW<'_, GrpLcdcE0CtrlSpec> {
        GrpLcdcE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_lcdc_e0_msel(&mut self) -> GrpLcdcE0MselW<'_, GrpLcdcE0CtrlSpec> {
        GrpLcdcE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn lcdc_e0_rmea(&mut self) -> LcdcE0RmeaW<'_, GrpLcdcE0CtrlSpec> {
        LcdcE0RmeaW::new(self, 10)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn lcdc_e0_rmeb(&mut self) -> LcdcE0RmebW<'_, GrpLcdcE0CtrlSpec> {
        LcdcE0RmebW::new(self, 11)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn lcdc_e0_rma(&mut self) -> LcdcE0RmaW<'_, GrpLcdcE0CtrlSpec> {
        LcdcE0RmaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn lcdc_e0_rmb(&mut self) -> LcdcE0RmbW<'_, GrpLcdcE0CtrlSpec> {
        LcdcE0RmbW::new(self, 16)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn lcdc_e0_test1a(&mut self) -> LcdcE0Test1aW<'_, GrpLcdcE0CtrlSpec> {
        LcdcE0Test1aW::new(self, 20)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn lcdc_e0_test1b(&mut self) -> LcdcE0Test1bW<'_, GrpLcdcE0CtrlSpec> {
        LcdcE0Test1bW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_lcdc_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_lcdc_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpLcdcE0CtrlSpec;
impl crate::RegisterSpec for GrpLcdcE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_lcdc_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpLcdcE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_lcdc_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpLcdcE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_LCDC_E0_CTRL to value 0x0002_2310"]
impl crate::Resettable for GrpLcdcE0CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_2310;
}
