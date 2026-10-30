#[doc = "Register `PKE_Interrupt` reader"]
pub type R = crate::R<PkeInterruptSpec>;
#[doc = "Register `PKE_Interrupt` writer"]
pub type W = crate::W<PkeInterruptSpec>;
#[doc = "Field `PKE_Interrupt_STATUS_MASK` reader - Interrupt mask for interrupt in PKE status register bit\\[7:0\\] 8'h00: interrupt masked 8'hff: interrupt not masked Caution: valid value of this field is only 8'h00 or 8'hff"]
pub type PkeInterruptStatusMaskR = crate::FieldReader;
#[doc = "Field `PKE_Interrupt_STATUS_MASK` writer - Interrupt mask for interrupt in PKE status register bit\\[7:0\\] 8'h00: interrupt masked 8'hff: interrupt not masked Caution: valid value of this field is only 8'h00 or 8'hff"]
pub type PkeInterruptStatusMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PKE_Interrupt_SIMPLIFY_HASH_MASK` reader - Wait for SIMPLIFY_ECDSA_EDDSA hash value 0: interrupt masked 1: interrupt not masked"]
pub type PkeInterruptSimplifyHashMaskR = crate::BitReader;
#[doc = "Field `PKE_Interrupt_SIMPLIFY_HASH_MASK` writer - Wait for SIMPLIFY_ECDSA_EDDSA hash value 0: interrupt masked 1: interrupt not masked"]
pub type PkeInterruptSimplifyHashMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Interrupt mask for interrupt in PKE status register bit\\[7:0\\] 8'h00: interrupt masked 8'hff: interrupt not masked Caution: valid value of this field is only 8'h00 or 8'hff"]
    #[inline(always)]
    pub fn pke_interrupt_status_mask(&self) -> PkeInterruptStatusMaskR {
        PkeInterruptStatusMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Wait for SIMPLIFY_ECDSA_EDDSA hash value 0: interrupt masked 1: interrupt not masked"]
    #[inline(always)]
    pub fn pke_interrupt_simplify_hash_mask(&self) -> PkeInterruptSimplifyHashMaskR {
        PkeInterruptSimplifyHashMaskR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt mask for interrupt in PKE status register bit\\[7:0\\] 8'h00: interrupt masked 8'hff: interrupt not masked Caution: valid value of this field is only 8'h00 or 8'hff"]
    #[inline(always)]
    pub fn pke_interrupt_status_mask(&mut self) -> PkeInterruptStatusMaskW<'_, PkeInterruptSpec> {
        PkeInterruptStatusMaskW::new(self, 0)
    }
    #[doc = "Bit 8 - Wait for SIMPLIFY_ECDSA_EDDSA hash value 0: interrupt masked 1: interrupt not masked"]
    #[inline(always)]
    pub fn pke_interrupt_simplify_hash_mask(
        &mut self,
    ) -> PkeInterruptSimplifyHashMaskW<'_, PkeInterruptSpec> {
        PkeInterruptSimplifyHashMaskW::new(self, 8)
    }
}
#[doc = "PKE Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pke_interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeInterruptSpec;
impl crate::RegisterSpec for PkeInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_interrupt::R`](R) reader structure"]
impl crate::Readable for PkeInterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`pke_interrupt::W`](W) writer structure"]
impl crate::Writable for PkeInterruptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKE_Interrupt to value 0"]
impl crate::Resettable for PkeInterruptSpec {}
