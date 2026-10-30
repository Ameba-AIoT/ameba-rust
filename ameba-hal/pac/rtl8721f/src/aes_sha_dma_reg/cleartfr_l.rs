#[doc = "Register `CLEARTFR_L` writer"]
pub type W = crate::W<CleartfrLSpec>;
#[doc = "Field `CLEARTFR_L_Clear_Tfr` writer - Interrupt clear for chn0-chn15 0 = no effect 1= clear interrupt"]
pub type CleartfrLClearTfrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Interrupt clear for chn0-chn15 0 = no effect 1= clear interrupt"]
    #[inline(always)]
    pub fn cleartfr_l_clear_tfr(&mut self) -> CleartfrLClearTfrW<'_, CleartfrLSpec> {
        CleartfrLClearTfrW::new(self, 0)
    }
}
#[doc = "Clear for IntTfr Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cleartfr_l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CleartfrLSpec;
impl crate::RegisterSpec for CleartfrLSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cleartfr_l::W`](W) writer structure"]
impl crate::Writable for CleartfrLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARTFR_L to value 0"]
impl crate::Resettable for CleartfrLSpec {}
