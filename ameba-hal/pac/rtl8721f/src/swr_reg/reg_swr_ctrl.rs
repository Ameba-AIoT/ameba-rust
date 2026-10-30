#[doc = "Register `REG_SWR_CTRL` reader"]
pub type R = crate::R<RegSwrCtrlSpec>;
#[doc = "Register `REG_SWR_CTRL` writer"]
pub type W = crate::W<RegSwrCtrlSpec>;
#[doc = "Field `DCORE_MODE_L` reader - SWR mode flag 1: PWM 0: PFM"]
pub type DcoreModeLR = crate::BitReader;
#[doc = "Field `swr_mode_src` reader - Select SWR mode ctrl source to determine SWR parameter 1: from Analog mode flag , which is 0x50\\[0\\] 0: from D->A mode control bit (Note , scan value is 1:PWM)"]
pub type SwrModeSrcR = crate::BitReader;
#[doc = "Field `swr_mode_src` writer - Select SWR mode ctrl source to determine SWR parameter 1: from Analog mode flag , which is 0x50\\[0\\] 0: from D->A mode control bit (Note , scan value is 1:PWM)"]
pub type SwrModeSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwm_load` reader - SWR pwm loading(scan value is 0) 1:heavy 0:light"]
pub type PwmLoadR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SWR mode flag 1: PWM 0: PFM"]
    #[inline(always)]
    pub fn dcore_mode_l(&self) -> DcoreModeLR {
        DcoreModeLR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select SWR mode ctrl source to determine SWR parameter 1: from Analog mode flag , which is 0x50\\[0\\] 0: from D->A mode control bit (Note , scan value is 1:PWM)"]
    #[inline(always)]
    pub fn swr_mode_src(&self) -> SwrModeSrcR {
        SwrModeSrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - SWR pwm loading(scan value is 0) 1:heavy 0:light"]
    #[inline(always)]
    pub fn pwm_load(&self) -> PwmLoadR {
        PwmLoadR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Select SWR mode ctrl source to determine SWR parameter 1: from Analog mode flag , which is 0x50\\[0\\] 0: from D->A mode control bit (Note , scan value is 1:PWM)"]
    #[inline(always)]
    pub fn swr_mode_src(&mut self) -> SwrModeSrcW<'_, RegSwrCtrlSpec> {
        SwrModeSrcW::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_swr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_swr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwrCtrlSpec;
impl crate::RegisterSpec for RegSwrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_swr_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSwrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_swr_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSwrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SWR_CTRL to value 0"]
impl crate::Resettable for RegSwrCtrlSpec {}
