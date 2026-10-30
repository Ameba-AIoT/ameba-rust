#[doc = "Register `REG_MQ3_TXBD_IDX` reader"]
pub type R = crate::R<RegMq3TxbdIdxSpec>;
#[doc = "Register `REG_MQ3_TXBD_IDX` writer"]
pub type W = crate::W<RegMq3TxbdIdxSpec>;
#[doc = "Field `BIT_MQ3_HOST_IDX` reader - MQ3 queue descriptor's host index"]
pub type BitMq3HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ3_HOST_IDX` writer - MQ3 queue descriptor's host index"]
pub type BitMq3HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ3_HW_IDX` reader - MQ3 queue descriptor's hardware index"]
pub type BitMq3HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ3 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq3_host_idx(&self) -> BitMq3HostIdxR {
        BitMq3HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ3 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq3_hw_idx(&self) -> BitMq3HwIdxR {
        BitMq3HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ3 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq3_host_idx(&mut self) -> BitMq3HostIdxW<'_, RegMq3TxbdIdxSpec> {
        BitMq3HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq3_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq3_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq3TxbdIdxSpec;
impl crate::RegisterSpec for RegMq3TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq3_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq3TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq3_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq3TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ3_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq3TxbdIdxSpec {}
