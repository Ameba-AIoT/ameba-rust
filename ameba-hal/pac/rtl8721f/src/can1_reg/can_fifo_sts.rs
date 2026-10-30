#[doc = "Register `CAN_FIFO_STS` reader"]
pub type R = crate::R<CanFifoStsSpec>;
#[doc = "Field `fifo_msg_full` reader - rx fifo full"]
pub type FifoMsgFullR = crate::BitReader;
#[doc = "Field `fifo_msg_empty` reader - rx fifo empty"]
pub type FifoMsgEmptyR = crate::BitReader;
#[doc = "Field `fifo_msg_overflow` reader - rx fifo overflow"]
pub type FifoMsgOverflowR = crate::BitReader;
#[doc = "Field `fifo_msg_lvl` reader - rx fifo message level"]
pub type FifoMsgLvlR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - rx fifo full"]
    #[inline(always)]
    pub fn fifo_msg_full(&self) -> FifoMsgFullR {
        FifoMsgFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx fifo empty"]
    #[inline(always)]
    pub fn fifo_msg_empty(&self) -> FifoMsgEmptyR {
        FifoMsgEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - rx fifo overflow"]
    #[inline(always)]
    pub fn fifo_msg_overflow(&self) -> FifoMsgOverflowR {
        FifoMsgOverflowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - rx fifo message level"]
    #[inline(always)]
    pub fn fifo_msg_lvl(&self) -> FifoMsgLvlR {
        FifoMsgLvlR::new(((self.bits >> 4) & 7) as u8)
    }
}
#[doc = "CAN bus FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`can_fifo_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanFifoStsSpec;
impl crate::RegisterSpec for CanFifoStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fifo_sts::R`](R) reader structure"]
impl crate::Readable for CanFifoStsSpec {}
#[doc = "`reset()` method sets CAN_FIFO_STS to value 0x02"]
impl crate::Resettable for CanFifoStsSpec {
    const RESET_VALUE: u32 = 0x02;
}
