#[doc = "Register `REG_RXQ_RXBD_IDX` reader"]
pub type R = crate::R<RegRxqRxbdIdxSpec>;
#[doc = "Register `REG_RXQ_RXBD_IDX` writer"]
pub type W = crate::W<RegRxqRxbdIdxSpec>;
#[doc = "Field `BIT_RXNRML_HOST_IDX` reader - RX queue descriptor's host index"]
pub type BitRxnrmlHostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXNRML_HOST_IDX` writer - RX queue descriptor's host index"]
pub type BitRxnrmlHostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_RXNRML_HW_IDX` reader - RX queue descriptor's hardware index"]
pub type BitRxnrmlHwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - RX queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_rxnrml_host_idx(&self) -> BitRxnrmlHostIdxR {
        BitRxnrmlHostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_rxnrml_hw_idx(&self) -> BitRxnrmlHwIdxR {
        BitRxnrmlHwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_rxnrml_host_idx(&mut self) -> BitRxnrmlHostIdxW<'_, RegRxqRxbdIdxSpec> {
        BitRxnrmlHostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxq_rxbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rxq_rxbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxqRxbdIdxSpec;
impl crate::RegisterSpec for RegRxqRxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxq_rxbd_idx::R`](R) reader structure"]
impl crate::Readable for RegRxqRxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rxq_rxbd_idx::W`](W) writer structure"]
impl crate::Writable for RegRxqRxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RXQ_RXBD_IDX to value 0"]
impl crate::Resettable for RegRxqRxbdIdxSpec {}
