#[doc = "Register `chn0_aes_int_clear` writer"]
pub type W = crate::W<Chn0AesIntClearSpec>;
#[doc = "Field `aes_int_clear` writer - Interrupt clear. Writing 1'b1 to the register will trigger a 1-cycle pulse interrupt clear."]
pub type AesIntClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt clear. Writing 1'b1 to the register will trigger a 1-cycle pulse interrupt clear."]
    #[inline(always)]
    pub fn aes_int_clear(&mut self) -> AesIntClearW<'_, Chn0AesIntClearSpec> {
        AesIntClearW::new(self, 0)
    }
}
#[doc = "Clear interrupt register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn0_aes_int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chn0AesIntClearSpec;
impl crate::RegisterSpec for Chn0AesIntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chn0_aes_int_clear::W`](W) writer structure"]
impl crate::Writable for Chn0AesIntClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets chn0_aes_int_clear to value 0"]
impl crate::Resettable for Chn0AesIntClearSpec {}
