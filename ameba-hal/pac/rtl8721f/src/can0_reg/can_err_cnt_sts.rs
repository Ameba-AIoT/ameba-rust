#[doc = "Register `CAN_ERR_CNT_STS` reader"]
pub type R = crate::R<CanErrCntStsSpec>;
#[doc = "Field `can_tec` reader - transmit error count"]
pub type CanTecR = crate::FieldReader<u16>;
#[doc = "Field `can_rec` reader - receive error count"]
pub type CanRecR = crate::FieldReader<u16>;
#[doc = "Field `can_error_passive` reader - error counter arrive at error passive level"]
pub type CanErrorPassiveR = crate::BitReader;
#[doc = "Field `can_error_busoff` reader - error counter arrive at bus off level"]
pub type CanErrorBusoffR = crate::BitReader;
#[doc = "Field `can_error_warning` reader - error counter arrive at warning threshold"]
pub type CanErrorWarningR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - transmit error count"]
    #[inline(always)]
    pub fn can_tec(&self) -> CanTecR {
        CanTecR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - receive error count"]
    #[inline(always)]
    pub fn can_rec(&self) -> CanRecR {
        CanRecR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 28 - error counter arrive at error passive level"]
    #[inline(always)]
    pub fn can_error_passive(&self) -> CanErrorPassiveR {
        CanErrorPassiveR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - error counter arrive at bus off level"]
    #[inline(always)]
    pub fn can_error_busoff(&self) -> CanErrorBusoffR {
        CanErrorBusoffR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - error counter arrive at warning threshold"]
    #[inline(always)]
    pub fn can_error_warning(&self) -> CanErrorWarningR {
        CanErrorWarningR::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "CAN bus error counter status\n\nYou can [`read`](crate::Reg::read) this register and get [`can_err_cnt_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanErrCntStsSpec;
impl crate::RegisterSpec for CanErrCntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_err_cnt_sts::R`](R) reader structure"]
impl crate::Readable for CanErrCntStsSpec {}
#[doc = "`reset()` method sets CAN_ERR_CNT_STS to value 0"]
impl crate::Resettable for CanErrCntStsSpec {}
