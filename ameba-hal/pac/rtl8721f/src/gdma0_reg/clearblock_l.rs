#[doc = "Register `CLEARBLOCK_L` writer"]
pub type W = crate::W<ClearblockLSpec>;
#[doc = "Field `CLEARBLOCK_L_Clear_Block` writer - Interrupt clear for chn0-chn15"]
pub type ClearblockLClearBlockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Interrupt clear for chn0-chn15"]
    #[inline(always)]
    pub fn clearblock_l_clear_block(&mut self) -> ClearblockLClearBlockW<'_, ClearblockLSpec> {
        ClearblockLClearBlockW::new(self, 0)
    }
}
#[doc = "Clear for IntBlock Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearblock_l::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearblockLSpec;
impl crate::RegisterSpec for ClearblockLSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clearblock_l::W`](W) writer structure"]
impl crate::Writable for ClearblockLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLEARBLOCK_L to value 0"]
impl crate::Resettable for ClearblockLSpec {}
