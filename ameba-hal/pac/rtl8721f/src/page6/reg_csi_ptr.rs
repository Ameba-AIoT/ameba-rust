#[doc = "Register `REG_CSI_PTR` reader"]
pub type R = crate::R<RegCsiPtrSpec>;
#[doc = "Field `BIT_csi_waddr_latch_v2` reader - Tx_fifo write pointer latch value at ndp_valid negative edge (unit: 8B)"]
pub type BitCsiWaddrLatchV2R = crate::FieldReader<u16>;
#[doc = "Field `BIT_csi_raddr_latch_v2` reader - Tx_fifo read pointer latch value after Tx CSI report pkt (unit: 8B)"]
pub type BitCsiRaddrLatchV2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Tx_fifo write pointer latch value at ndp_valid negative edge (unit: 8B)"]
    #[inline(always)]
    pub fn bit_csi_waddr_latch_v2(&self) -> BitCsiWaddrLatchV2R {
        BitCsiWaddrLatchV2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Tx_fifo read pointer latch value after Tx CSI report pkt (unit: 8B)"]
    #[inline(always)]
    pub fn bit_csi_raddr_latch_v2(&self) -> BitCsiRaddrLatchV2R {
        BitCsiRaddrLatchV2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_csi_ptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCsiPtrSpec;
impl crate::RegisterSpec for RegCsiPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_csi_ptr::R`](R) reader structure"]
impl crate::Readable for RegCsiPtrSpec {}
#[doc = "`reset()` method sets REG_CSI_PTR to value 0"]
impl crate::Resettable for RegCsiPtrSpec {}
