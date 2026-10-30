#[doc = "Register `CAN_ERR_STATUS` reader"]
pub type R = crate::R<CanErrStatusSpec>;
#[doc = "Field `can_error_bit0` reader - latest error is bit0 error, tx=0 but rx=1"]
pub type CanErrorBit0R = crate::BitReader;
#[doc = "Field `can_error_bit1` reader - latest error is bit1 error, tx=1 but rx=0"]
pub type CanErrorBit1R = crate::BitReader;
#[doc = "Field `can_error_form` reader - latest error is form error"]
pub type CanErrorFormR = crate::BitReader;
#[doc = "Field `can_error_crc` reader - latest error is ctc error"]
pub type CanErrorCrcR = crate::BitReader;
#[doc = "Field `can_error_stuff` reader - latest error is stuff error"]
pub type CanErrorStuffR = crate::BitReader;
#[doc = "Field `can_error_ack` reader - latest error is ack error"]
pub type CanErrorAckR = crate::BitReader;
#[doc = "Field `can_error_tx` reader - can tx error flag"]
pub type CanErrorTxR = crate::BitReader;
#[doc = "Field `can_error_rx` reader - can rx error flag"]
pub type CanErrorRxR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - latest error is bit0 error, tx=0 but rx=1"]
    #[inline(always)]
    pub fn can_error_bit0(&self) -> CanErrorBit0R {
        CanErrorBit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - latest error is bit1 error, tx=1 but rx=0"]
    #[inline(always)]
    pub fn can_error_bit1(&self) -> CanErrorBit1R {
        CanErrorBit1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - latest error is form error"]
    #[inline(always)]
    pub fn can_error_form(&self) -> CanErrorFormR {
        CanErrorFormR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - latest error is ctc error"]
    #[inline(always)]
    pub fn can_error_crc(&self) -> CanErrorCrcR {
        CanErrorCrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - latest error is stuff error"]
    #[inline(always)]
    pub fn can_error_stuff(&self) -> CanErrorStuffR {
        CanErrorStuffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - latest error is ack error"]
    #[inline(always)]
    pub fn can_error_ack(&self) -> CanErrorAckR {
        CanErrorAckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - can tx error flag"]
    #[inline(always)]
    pub fn can_error_tx(&self) -> CanErrorTxR {
        CanErrorTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - can rx error flag"]
    #[inline(always)]
    pub fn can_error_rx(&self) -> CanErrorRxR {
        CanErrorRxR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "CAN error status\n\nYou can [`read`](crate::Reg::read) this register and get [`can_err_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanErrStatusSpec;
impl crate::RegisterSpec for CanErrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_err_status::R`](R) reader structure"]
impl crate::Readable for CanErrStatusSpec {}
#[doc = "`reset()` method sets CAN_ERR_STATUS to value 0"]
impl crate::Resettable for CanErrStatusSpec {}
