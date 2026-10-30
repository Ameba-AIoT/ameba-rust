#[doc = "Register `REG_MQ4_TXBD_IDX` reader"]
pub type R = crate::R<RegMq4TxbdIdxSpec>;
#[doc = "Register `REG_MQ4_TXBD_IDX` writer"]
pub type W = crate::W<RegMq4TxbdIdxSpec>;
#[doc = "Field `BIT_MQ4_HOST_IDX` reader - MQ4 queue descriptor's host index"]
pub type BitMq4HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ4_HOST_IDX` writer - MQ4 queue descriptor's host index"]
pub type BitMq4HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ4_HW_IDX` reader - MQ4 queue descriptor's hardware index"]
pub type BitMq4HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ4 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq4_host_idx(&self) -> BitMq4HostIdxR {
        BitMq4HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ4 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq4_hw_idx(&self) -> BitMq4HwIdxR {
        BitMq4HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ4 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq4_host_idx(&mut self) -> BitMq4HostIdxW<'_, RegMq4TxbdIdxSpec> {
        BitMq4HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq4_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq4_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq4TxbdIdxSpec;
impl crate::RegisterSpec for RegMq4TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq4_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq4TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq4_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq4TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ4_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq4TxbdIdxSpec {}
