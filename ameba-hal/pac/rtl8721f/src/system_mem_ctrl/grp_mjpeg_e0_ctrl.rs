#[doc = "Register `GRP_MJPEG_E0_CTRL` reader"]
pub type R = crate::R<GrpMjpegE0CtrlSpec>;
#[doc = "Register `GRP_MJPEG_E0_CTRL` writer"]
pub type W = crate::W<GrpMjpegE0CtrlSpec>;
#[doc = "Field `grp_mjpeg_e0_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE0PwrActR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e0_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE0PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_mjpeg_e0_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE0PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e0_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE0PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_mjpeg_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpMjpegE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_mjpeg_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpMjpegE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_mjpeg_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpMjpegE0MselR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpMjpegE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mjpeg_e0_rme` reader - RME"]
pub type MjpegE0RmeR = crate::BitReader;
#[doc = "Field `mjpeg_e0_rme` writer - RME"]
pub type MjpegE0RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mjpeg_e0_rm` reader - RM"]
pub type MjpegE0RmR = crate::FieldReader;
#[doc = "Field `mjpeg_e0_rm` writer - RM"]
pub type MjpegE0RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `mjpeg_e0_test1` reader - TEST1"]
pub type MjpegE0Test1R = crate::BitReader;
#[doc = "Field `mjpeg_e0_test1` writer - TEST1"]
pub type MjpegE0Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_pwr_act(&self) -> GrpMjpegE0PwrActR {
        GrpMjpegE0PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_pwr_lps(&self) -> GrpMjpegE0PwrLpsR {
        GrpMjpegE0PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_loop_mode(&self) -> GrpMjpegE0LoopModeR {
        GrpMjpegE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_msel(&self) -> GrpMjpegE0MselR {
        GrpMjpegE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn mjpeg_e0_rme(&self) -> MjpegE0RmeR {
        MjpegE0RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn mjpeg_e0_rm(&self) -> MjpegE0RmR {
        MjpegE0RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn mjpeg_e0_test1(&self) -> MjpegE0Test1R {
        MjpegE0Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_pwr_act(&mut self) -> GrpMjpegE0PwrActW<'_, GrpMjpegE0CtrlSpec> {
        GrpMjpegE0PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_pwr_lps(&mut self) -> GrpMjpegE0PwrLpsW<'_, GrpMjpegE0CtrlSpec> {
        GrpMjpegE0PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_loop_mode(&mut self) -> GrpMjpegE0LoopModeW<'_, GrpMjpegE0CtrlSpec> {
        GrpMjpegE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_mjpeg_e0_msel(&mut self) -> GrpMjpegE0MselW<'_, GrpMjpegE0CtrlSpec> {
        GrpMjpegE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn mjpeg_e0_rme(&mut self) -> MjpegE0RmeW<'_, GrpMjpegE0CtrlSpec> {
        MjpegE0RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn mjpeg_e0_rm(&mut self) -> MjpegE0RmW<'_, GrpMjpegE0CtrlSpec> {
        MjpegE0RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn mjpeg_e0_test1(&mut self) -> MjpegE0Test1W<'_, GrpMjpegE0CtrlSpec> {
        MjpegE0Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_mjpeg_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_mjpeg_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpMjpegE0CtrlSpec;
impl crate::RegisterSpec for GrpMjpegE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_mjpeg_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpMjpegE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_mjpeg_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpMjpegE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_MJPEG_E0_CTRL to value 0x1310"]
impl crate::Resettable for GrpMjpegE0CtrlSpec {
    const RESET_VALUE: u32 = 0x1310;
}
