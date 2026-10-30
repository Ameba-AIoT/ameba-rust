#[doc = "Register `REG_MQ5_TXBD_IDX` reader"]
pub type R = crate::R<RegMq5TxbdIdxSpec>;
#[doc = "Register `REG_MQ5_TXBD_IDX` writer"]
pub type W = crate::W<RegMq5TxbdIdxSpec>;
#[doc = "Field `BIT_MQ5_HOST_IDX` reader - MQ5 queue descriptor's host index"]
pub type BitMq5HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ5_HOST_IDX` writer - MQ5 queue descriptor's host index"]
pub type BitMq5HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ5_HW_IDX` reader - MQ5 queue descriptor's hardware index"]
pub type BitMq5HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ5 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq5_host_idx(&self) -> BitMq5HostIdxR {
        BitMq5HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ5 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq5_hw_idx(&self) -> BitMq5HwIdxR {
        BitMq5HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ5 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq5_host_idx(&mut self) -> BitMq5HostIdxW<'_, RegMq5TxbdIdxSpec> {
        BitMq5HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq5_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq5_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq5TxbdIdxSpec;
impl crate::RegisterSpec for RegMq5TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq5_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq5TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq5_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq5TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ5_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq5TxbdIdxSpec {}
