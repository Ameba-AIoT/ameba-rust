#[doc = "Register `REG_MQ0_TXBD_IDX` reader"]
pub type R = crate::R<RegMq0TxbdIdxSpec>;
#[doc = "Register `REG_MQ0_TXBD_IDX` writer"]
pub type W = crate::W<RegMq0TxbdIdxSpec>;
#[doc = "Field `BIT_MQ0_HOST_IDX` reader - MQ0 queue descriptor's host index"]
pub type BitMq0HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ0_HOST_IDX` writer - MQ0 queue descriptor's host index"]
pub type BitMq0HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ0_HW_IDX` reader - MQ0 queue descriptor's hardware index"]
pub type BitMq0HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ0 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq0_host_idx(&self) -> BitMq0HostIdxR {
        BitMq0HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ0 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq0_hw_idx(&self) -> BitMq0HwIdxR {
        BitMq0HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ0 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq0_host_idx(&mut self) -> BitMq0HostIdxW<'_, RegMq0TxbdIdxSpec> {
        BitMq0HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq0_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq0_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq0TxbdIdxSpec;
impl crate::RegisterSpec for RegMq0TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq0_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq0TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq0_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq0TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ0_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq0TxbdIdxSpec {}
