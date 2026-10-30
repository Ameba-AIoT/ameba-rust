#[doc = "Register `REG_ENABLE_DUMMY_READ` writer"]
pub type W = crate::W<RegEnableDummyReadSpec>;
#[doc = "Field `BIT_EN_OUT_STD` writer - cpl multi tag enable"]
pub type BitEnOutStdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_R_DEBUG` writer - Debug Selection Register"]
pub type BitRDebugW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 6 - cpl multi tag enable"]
    #[inline(always)]
    pub fn bit_en_out_std(&mut self) -> BitEnOutStdW<'_, RegEnableDummyReadSpec> {
        BitEnOutStdW::new(self, 6)
    }
    #[doc = "Bits 22:23 - Debug Selection Register"]
    #[inline(always)]
    pub fn bit_r_debug(&mut self) -> BitRDebugW<'_, RegEnableDummyReadSpec> {
        BitRDebugW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_enable_dummy_read::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegEnableDummyReadSpec;
impl crate::RegisterSpec for RegEnableDummyReadSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reg_enable_dummy_read::W`](W) writer structure"]
impl crate::Writable for RegEnableDummyReadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ENABLE_DUMMY_READ to value 0x40"]
impl crate::Resettable for RegEnableDummyReadSpec {
    const RESET_VALUE: u32 = 0x40;
}
