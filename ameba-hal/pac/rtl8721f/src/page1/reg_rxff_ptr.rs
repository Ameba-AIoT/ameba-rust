#[doc = "Register `REG_RXFF_PTR` reader"]
pub type R = crate::R<RegRxffPtrSpec>;
#[doc = "Field `BIT_RXFF0_WTPTR` reader - The write pointer address of RXFF0. The value is set to the physical LX bus address."]
pub type BitRxff0WtptrR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXFF0_RDPTR` reader - The read pointer address of RXFF0. The value is set to the physical LX bus address."]
pub type BitRxff0RdptrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The write pointer address of RXFF0. The value is set to the physical LX bus address."]
    #[inline(always)]
    pub fn bit_rxff0_wtptr(&self) -> BitRxff0WtptrR {
        BitRxff0WtptrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The read pointer address of RXFF0. The value is set to the physical LX bus address."]
    #[inline(always)]
    pub fn bit_rxff0_rdptr(&self) -> BitRxff0RdptrR {
        BitRxff0RdptrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "RX FIFO Read/write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rxff_ptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRxffPtrSpec;
impl crate::RegisterSpec for RegRxffPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rxff_ptr::R`](R) reader structure"]
impl crate::Readable for RegRxffPtrSpec {}
#[doc = "`reset()` method sets REG_RXFF_PTR to value 0"]
impl crate::Resettable for RegRxffPtrSpec {}
