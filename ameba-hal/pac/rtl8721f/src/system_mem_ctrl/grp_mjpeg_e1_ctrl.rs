#[doc = "Register `GRP_MJPEG_E1_CTRL` reader"]
pub type R = crate::R<GrpMjpegE1CtrlSpec>;
#[doc = "Register `GRP_MJPEG_E1_CTRL` writer"]
pub type W = crate::W<GrpMjpegE1CtrlSpec>;
#[doc = "Field `grp_mjpeg_e1_pwr_act` reader - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE1PwrActR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e1_pwr_act` writer - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE1PwrActW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_mjpeg_e1_pwr_lps` reader - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE1PwrLpsR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e1_pwr_lps` writer - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
pub type GrpMjpegE1PwrLpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `grp_mjpeg_e1_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpMjpegE1LoopModeR = crate::BitReader;
#[doc = "Field `grp_mjpeg_e1_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpMjpegE1LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_mjpeg_e1_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpMjpegE1MselR = crate::FieldReader;
#[doc = "Field `grp_mjpeg_e1_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpMjpegE1MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mjpeg_e1_rme` reader - RME"]
pub type MjpegE1RmeR = crate::BitReader;
#[doc = "Field `mjpeg_e1_rme` writer - RME"]
pub type MjpegE1RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mjpeg_e1_rm` reader - RM"]
pub type MjpegE1RmR = crate::FieldReader;
#[doc = "Field `mjpeg_e1_rm` writer - RM"]
pub type MjpegE1RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `mjpeg_e1_test1` reader - TEST1"]
pub type MjpegE1Test1R = crate::BitReader;
#[doc = "Field `mjpeg_e1_test1` writer - TEST1"]
pub type MjpegE1Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_pwr_act(&self) -> GrpMjpegE1PwrActR {
        GrpMjpegE1PwrActR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_pwr_lps(&self) -> GrpMjpegE1PwrLpsR {
        GrpMjpegE1PwrLpsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_loop_mode(&self) -> GrpMjpegE1LoopModeR {
        GrpMjpegE1LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_msel(&self) -> GrpMjpegE1MselR {
        GrpMjpegE1MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn mjpeg_e1_rme(&self) -> MjpegE1RmeR {
        MjpegE1RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn mjpeg_e1_rm(&self) -> MjpegE1RmR {
        MjpegE1RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn mjpeg_e1_test1(&self) -> MjpegE1Test1R {
        MjpegE1Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Used to controll power mode of this memory group when chip is ACTIVE. For FTC_ULP mem, pwr_act\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_act\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_pwr_act(&mut self) -> GrpMjpegE1PwrActW<'_, GrpMjpegE1CtrlSpec> {
        GrpMjpegE1PwrActW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Reserved for grp_@_pwr_lps for memory groups power controlled by SYSPMC. Setted before chip goes to SLEEP.Used to controll power mode of this memory group when chip is in SLEEP. For FTC_ULP mem, pwr_lps\\[2:0\\] map to {SLP,RET,NAP}; For SNPS mem, pwr_lps\\[2:0\\] map to {SD,DS,LS}. *000 - active *001 - light sleep *010 - deep sleep *100 - shut down"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_pwr_lps(&mut self) -> GrpMjpegE1PwrLpsW<'_, GrpMjpegE1CtrlSpec> {
        GrpMjpegE1PwrLpsW::new(self, 3)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_loop_mode(&mut self) -> GrpMjpegE1LoopModeW<'_, GrpMjpegE1CtrlSpec> {
        GrpMjpegE1LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_mjpeg_e1_msel(&mut self) -> GrpMjpegE1MselW<'_, GrpMjpegE1CtrlSpec> {
        GrpMjpegE1MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn mjpeg_e1_rme(&mut self) -> MjpegE1RmeW<'_, GrpMjpegE1CtrlSpec> {
        MjpegE1RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn mjpeg_e1_rm(&mut self) -> MjpegE1RmW<'_, GrpMjpegE1CtrlSpec> {
        MjpegE1RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn mjpeg_e1_test1(&mut self) -> MjpegE1Test1W<'_, GrpMjpegE1CtrlSpec> {
        MjpegE1Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_mjpeg_e1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_mjpeg_e1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpMjpegE1CtrlSpec;
impl crate::RegisterSpec for GrpMjpegE1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_mjpeg_e1_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpMjpegE1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_mjpeg_e1_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpMjpegE1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_MJPEG_E1_CTRL to value 0x1310"]
impl crate::Resettable for GrpMjpegE1CtrlSpec {
    const RESET_VALUE: u32 = 0x1310;
}
