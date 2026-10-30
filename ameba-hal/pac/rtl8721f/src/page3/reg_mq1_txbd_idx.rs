#[doc = "Register `REG_MQ1_TXBD_IDX` reader"]
pub type R = crate::R<RegMq1TxbdIdxSpec>;
#[doc = "Register `REG_MQ1_TXBD_IDX` writer"]
pub type W = crate::W<RegMq1TxbdIdxSpec>;
#[doc = "Field `BIT_MQ1_HOST_IDX` reader - MQ1 queue descriptor's host index"]
pub type BitMq1HostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MQ1_HOST_IDX` writer - MQ1 queue descriptor's host index"]
pub type BitMq1HostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MQ1_HW_IDX` reader - MQ1 queue descriptor's hardware index"]
pub type BitMq1HwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MQ1 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq1_host_idx(&self) -> BitMq1HostIdxR {
        BitMq1HostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MQ1 queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mq1_hw_idx(&self) -> BitMq1HwIdxR {
        BitMq1HwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MQ1 queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mq1_host_idx(&mut self) -> BitMq1HostIdxW<'_, RegMq1TxbdIdxSpec> {
        BitMq1HostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mq1_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mq1_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMq1TxbdIdxSpec;
impl crate::RegisterSpec for RegMq1TxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mq1_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMq1TxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mq1_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMq1TxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MQ1_TXBD_IDX to value 0"]
impl crate::Resettable for RegMq1TxbdIdxSpec {}
