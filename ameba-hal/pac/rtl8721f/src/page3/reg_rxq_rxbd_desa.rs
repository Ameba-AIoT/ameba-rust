#[doc = "Register `REG_RXQ_RXBD_DESA` reader"]
pub type R = crate::R<RegRxqRxbdDesaSpec>;
#[doc = "Register `REG_RXQ_RXBD_DESA` writer"]
pub type W = crate::W<RegRxqRxbdDesaSpec>;
#[doc = "Field `BIT_RXNRML_DESA` reader - Buffer Descriptor Base Address for RX"]
pub type BitRxnrmlDesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_RXNRML_DESA` writer - Buffer Descriptor Base Address for RX"]
pub type BitRxnrmlDesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for RX"]
    #[inline(always)]
    pub fn bit_rxnrml_desa(&self) -> BitRxnrmlDesaR {
        BitRxnrmlDesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for RX"]
    #[inline(always)]
    pub fn bit_rxnrml_desa(&mut self) -> BitRxnrmlDesaW<'_, RegRxqRxbdDesaSpec> {
        BitRxnrmlDesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxq_rxbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxq_rxbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxqRxbdDesaSpec;
impl crate::RegisterSpec for RegRxqRxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxq_rxbd_desa::R`](R) reader structure"]
impl crate::Readable for RegRxqRxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxq_rxbd_desa::W`](W) writer structure"]
impl crate::Writable for RegRxqRxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXQ_RXBD_DESA to value 0"]
impl crate::Resettable for RegRxqRxbdDesaSpec {}
