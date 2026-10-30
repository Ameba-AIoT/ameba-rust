#[doc = "Register `REG_MQ7_TXBD_IDX` reader"]
pub type R = crate::R<RegMq7TxbdIdxSpec>;
#[doc = "Register `REG_MQ7_TXBD_IDX` writer"]
pub type W = crate::W<RegMq7TxbdIdxSpec>;
#[doc = "Field `BIT_MQ7_HOST_IDX` reader - MQ7 queue descriptor's host index"]
pub type BitMq7HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ7_HOST_IDX` writer - MQ7 queue descriptor's host index"]
pub type BitMq7HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ7_HW_IDX` reader - MQ7 queue descriptor's hardware index"]
pub type BitMq7HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ7 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq7_host_idx(&self) -> BitMq7HostIdxR {
        BitMq7HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ7 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq7_hw_idx(&self) -> BitMq7HwIdxR {
        BitMq7HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ7 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq7_host_idx(&mut self) -> BitMq7HostIdxW<'_, RegMq7TxbdIdxSpec> {
        BitMq7HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq7_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq7_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq7TxbdIdxSpec;
impl crate::RegisterSpec for RegMq7TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq7_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq7TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq7_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq7TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ7_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq7TxbdIdxSpec {}
