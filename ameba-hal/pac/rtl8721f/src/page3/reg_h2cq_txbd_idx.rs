#[doc = "Register `REG_H2CQ_TXBD_IDX` reader"]
pub type R = crate::R<RegH2cqTxbdIdxSpec>;
#[doc = "Register `REG_H2CQ_TXBD_IDX` writer"]
pub type W = crate::W<RegH2cqTxbdIdxSpec>;
#[doc = "Field `BIT_H2CQ_HOST_IDX` reader - H2C queue descriptor's host index"]
pub type BitH2cqHostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_H2CQ_HOST_IDX` writer - H2C queue descriptor's host index"]
pub type BitH2cqHostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_H2CQ_HW_IDX` reader - H2C queue descriptor's hardware index"]
pub type BitH2cqHwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - H2C queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_h2cq_host_idx(&self) -> BitH2cqHostIdxR {
        BitH2cqHostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - H2C queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_h2cq_hw_idx(&self) -> BitH2cqHwIdxR {
        BitH2cqHwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - H2C queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_h2cq_host_idx(&mut self) -> BitH2cqHostIdxW<'_, RegH2cqTxbdIdxSpec> {
        BitH2cqHostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_h2cq_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_h2cq_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegH2cqTxbdIdxSpec;
impl crate::RegisterSpec for RegH2cqTxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_h2cq_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegH2cqTxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_h2cq_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegH2cqTxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_H2CQ_TXBD_IDX to value 0"]
impl crate::Resettable for RegH2cqTxbdIdxSpec {}
