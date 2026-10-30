#[doc = "Register `CLEARERR_L` writer"]
pub type W = crate::W<ClearerrLSpec>;
#[doc = "Field `CLEARERR_L_Clear_Protocol_Error` writer - Interrupt clear for chn0-chn15. (Protocol error)"]
pub type ClearerrLClearProtocolErrorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Interrupt clear for chn0-chn15. (Protocol error)"]
    #[inline(always)]
    pub fn clearerr_l_clear_protocol_error(
        &mut self,
    ) -> ClearerrLClearProtocolErrorW<'_, ClearerrLSpec> {
        ClearerrLClearProtocolErrorW::new(self, 0)
    }
}
#[doc = "Clear for IntErr Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearerr_l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearerrLSpec;
impl crate::RegisterSpec for ClearerrLSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clearerr_l::W`](W) writer structure"]
impl crate::Writable for ClearerrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARERR_L to value 0"]
impl crate::Resettable for ClearerrLSpec {}
