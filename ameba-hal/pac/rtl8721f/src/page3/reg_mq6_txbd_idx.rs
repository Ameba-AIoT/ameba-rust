#[doc = "Register `REG_MQ6_TXBD_IDX` reader"]
pub type R = crate::R<RegMq6TxbdIdxSpec>;
#[doc = "Register `REG_MQ6_TXBD_IDX` writer"]
pub type W = crate::W<RegMq6TxbdIdxSpec>;
#[doc = "Field `BIT_MQ6_HOST_IDX` reader - MQ6 queue descriptor's host index"]
pub type BitMq6HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ6_HOST_IDX` writer - MQ6 queue descriptor's host index"]
pub type BitMq6HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ6_HW_IDX` reader - MQ6 queue descriptor's hardware index"]
pub type BitMq6HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ6 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq6_host_idx(&self) -> BitMq6HostIdxR {
        BitMq6HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ6 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq6_hw_idx(&self) -> BitMq6HwIdxR {
        BitMq6HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ6 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq6_host_idx(&mut self) -> BitMq6HostIdxW<'_, RegMq6TxbdIdxSpec> {
        BitMq6HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq6_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq6_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq6TxbdIdxSpec;
impl crate::RegisterSpec for RegMq6TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq6_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq6TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq6_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq6TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ6_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq6TxbdIdxSpec {}
