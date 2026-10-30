#[doc = "Register `BUS_ERR_DTL` reader"]
pub type R = crate::R<BusErrDtlSpec>;
#[doc = "Field `RD_ERR` reader - The mark for read error response Assert bit N when receiving error response from axi_rid=N"]
pub type RdErrR = crate::FieldReader;
#[doc = "Field `WR_ERR` reader - The mark for write error response Assert this bit when receiving error response from axi B cahnnel"]
pub type WrErrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - The mark for read error response Assert bit N when receiving error response from axi_rid=N"]
    #[inline(always)]
    pub fn rd_err(&self) -> RdErrR {
        RdErrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - The mark for write error response Assert this bit when receiving error response from axi B cahnnel"]
    #[inline(always)]
    pub fn wr_err(&self) -> WrErrR {
        WrErrR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "BUS error detail register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_err_dtl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusErrDtlSpec;
impl crate::RegisterSpec for BusErrDtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_err_dtl::R`](R) reader structure"]
impl crate::Readable for BusErrDtlSpec {}
#[doc = "`reset()` method sets BUS_ERR_DTL to value 0"]
impl crate::Resettable for BusErrDtlSpec {}
