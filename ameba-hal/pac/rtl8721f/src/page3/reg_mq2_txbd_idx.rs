#[doc = "Register `REG_MQ2_TXBD_IDX` reader"]
pub type R = crate::R<RegMq2TxbdIdxSpec>;
#[doc = "Register `REG_MQ2_TXBD_IDX` writer"]
pub type W = crate::W<RegMq2TxbdIdxSpec>;
#[doc = "Field `BIT_MQ2_HOST_IDX` reader - MQ2 queue descriptor's host index"]
pub type BitMq2HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ2_HOST_IDX` writer - MQ2 queue descriptor's host index"]
pub type BitMq2HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ2_HW_IDX` reader - MQ2 queue descriptor's hardware index"]
pub type BitMq2HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ2 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq2_host_idx(&self) -> BitMq2HostIdxR {
        BitMq2HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ2 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq2_hw_idx(&self) -> BitMq2HwIdxR {
        BitMq2HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ2 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq2_host_idx(&mut self) -> BitMq2HostIdxW<'_, RegMq2TxbdIdxSpec> {
        BitMq2HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq2_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq2_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq2TxbdIdxSpec;
impl crate::RegisterSpec for RegMq2TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq2_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq2TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq2_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq2TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ2_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq2TxbdIdxSpec {}
