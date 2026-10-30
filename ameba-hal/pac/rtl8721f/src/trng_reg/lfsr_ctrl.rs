#[doc = "Register `LFSR_CTRL` reader"]
pub type R = crate::R<LfsrCtrlSpec>;
#[doc = "Register `LFSR_CTRL` writer"]
pub type W = crate::W<LfsrCtrlSpec>;
#[doc = "Field `INIT_DROP_COUNT` reader - Discard number of bit after reset or re-init. Default shift out 64 bits in LFSR."]
pub type InitDropCountR = crate::FieldReader;
#[doc = "Field `INIT_DROP_COUNT` writer - Discard number of bit after reset or re-init. Default shift out 64 bits in LFSR."]
pub type InitDropCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FREE_RUN` reader - 0x1: continuously run LFSR when no input data 0x0: only shift LFSR when input 1 bit data"]
pub type FreeRunR = crate::BitReader;
#[doc = "Field `FREE_RUN` writer - 0x1: continuously run LFSR when no input data 0x0: only shift LFSR when input 1 bit data"]
pub type FreeRunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Discard number of bit after reset or re-init. Default shift out 64 bits in LFSR."]
    #[inline(always)]
    pub fn init_drop_count(&self) -> InitDropCountR {
        InitDropCountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 0x1: continuously run LFSR when no input data 0x0: only shift LFSR when input 1 bit data"]
    #[inline(always)]
    pub fn free_run(&self) -> FreeRunR {
        FreeRunR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Discard number of bit after reset or re-init. Default shift out 64 bits in LFSR."]
    #[inline(always)]
    pub fn init_drop_count(&mut self) -> InitDropCountW<'_, LfsrCtrlSpec> {
        InitDropCountW::new(self, 0)
    }
    #[doc = "Bit 8 - 0x1: continuously run LFSR when no input data 0x0: only shift LFSR when input 1 bit data"]
    #[inline(always)]
    pub fn free_run(&mut self) -> FreeRunW<'_, LfsrCtrlSpec> {
        FreeRunW::new(self, 8)
    }
}
#[doc = "LFSR Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfsrCtrlSpec;
impl crate::RegisterSpec for LfsrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsr_ctrl::R`](R) reader structure"]
impl crate::Readable for LfsrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfsr_ctrl::W`](W) writer structure"]
impl crate::Writable for LfsrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSR_CTRL to value 0x0140"]
impl crate::Resettable for LfsrCtrlSpec {
    const RESET_VALUE: u32 = 0x0140;
}
