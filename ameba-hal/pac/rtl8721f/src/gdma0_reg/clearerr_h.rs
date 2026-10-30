#[doc = "Register `CLEARERR_H` writer"]
pub type W = crate::W<ClearerrHSpec>;
#[doc = "Field `CLEARERR_H_Clear_Sec_Error` writer - Interrupt clear for chn0-chn15 (Non-secure access secure register error, Sec Error) when DMAC_SECURE_MODE has beed defined, else it is reserved;"]
pub type ClearerrHClearSecErrorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Interrupt clear for chn0-chn15 (Non-secure access secure register error, Sec Error) when DMAC_SECURE_MODE has beed defined, else it is reserved;"]
    #[inline(always)]
    pub fn clearerr_h_clear_sec_error(&mut self) -> ClearerrHClearSecErrorW<'_, ClearerrHSpec> {
        ClearerrHClearSecErrorW::new(self, 0)
    }
}
#[doc = "Clear for non-secure access secure register error if defined\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearerr_h::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearerrHSpec;
impl crate::RegisterSpec for ClearerrHSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clearerr_h::W`](W) writer structure"]
impl crate::Writable for ClearerrHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARERR_H to value 0"]
impl crate::Resettable for ClearerrHSpec {}
