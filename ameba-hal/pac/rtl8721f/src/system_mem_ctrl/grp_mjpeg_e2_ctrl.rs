#[doc = "Register `GRP_MJPEG_E2_CTRL` reader"]
pub type R = crate::R<GrpMjpegE2CtrlSpec>;
#[doc = "Register `GRP_MJPEG_E2_CTRL` writer"]
pub type W = crate::W<GrpMjpegE2CtrlSpec>;
#[doc = "Field `grp_mjpeg_e2_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE2PwrActR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e2_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE2PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_mjpeg_e2_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE2PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e2_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE2PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_mjpeg_e2_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpMjpegE2LoopModeR = crate::BitReader;
#[doc = "Field `grp_mjpeg_e2_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpMjpegE2LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_mjpeg_e2_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpMjpegE2MselR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e2_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpMjpegE2MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mjpeg_e2_rme` reader - RME"]
pub type MjpegE2RmeR = crate::BitReader;
#[doc = "Field `mjpeg_e2_rme` writer - RME"]
pub type MjpegE2RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mjpeg_e2_rm` reader - RM"]
pub type MjpegE2RmR = crate::FieldReader;
#[doc = "Field `mjpeg_e2_rm` writer - RM"]
pub type MjpegE2RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `mjpeg_e2_test1` reader - TEST1"]
pub type MjpegE2Test1R = crate::BitReader;
#[doc = "Field `mjpeg_e2_test1` writer - TEST1"]
pub type MjpegE2Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_pwr_act(&self) -> GrpMjpegE2PwrActR {
        GrpMjpegE2PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_pwr_lps(&self) -> GrpMjpegE2PwrLpsR {
        GrpMjpegE2PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_loop_mode(&self) -> GrpMjpegE2LoopModeR {
        GrpMjpegE2LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_msel(&self) -> GrpMjpegE2MselR {
        GrpMjpegE2MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn mjpeg_e2_rme(&self) -> MjpegE2RmeR {
        MjpegE2RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn mjpeg_e2_rm(&self) -> MjpegE2RmR {
        MjpegE2RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn mjpeg_e2_test1(&self) -> MjpegE2Test1R {
        MjpegE2Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_pwr_act(&mut self) -> GrpMjpegE2PwrActW<'_, GrpMjpegE2CtrlSpec> {
        GrpMjpegE2PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_pwr_lps(&mut self) -> GrpMjpegE2PwrLpsW<'_, GrpMjpegE2CtrlSpec> {
        GrpMjpegE2PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_loop_mode(&mut self) -> GrpMjpegE2LoopModeW<'_, GrpMjpegE2CtrlSpec> {
        GrpMjpegE2LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_mjpeg_e2_msel(&mut self) -> GrpMjpegE2MselW<'_, GrpMjpegE2CtrlSpec> {
        GrpMjpegE2MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn mjpeg_e2_rme(&mut self) -> MjpegE2RmeW<'_, GrpMjpegE2CtrlSpec> {
        MjpegE2RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn mjpeg_e2_rm(&mut self) -> MjpegE2RmW<'_, GrpMjpegE2CtrlSpec> {
        MjpegE2RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn mjpeg_e2_test1(&mut self) -> MjpegE2Test1W<'_, GrpMjpegE2CtrlSpec> {
        MjpegE2Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_mjpeg_e2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_mjpeg_e2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpMjpegE2CtrlSpec;
impl crate::RegisterSpec for GrpMjpegE2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_mjpeg_e2_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpMjpegE2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_mjpeg_e2_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpMjpegE2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_MJPEG_E2_CTRL to value 0x1310"]
impl crate::Resettable for GrpMjpegE2CtrlSpec {
    const RESET_VALUE: u32 = 0x1310;
}
