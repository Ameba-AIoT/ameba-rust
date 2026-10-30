#[doc = "Register `REG_BKQ_TXBD_IDX` reader"]
pub type R = crate::R<RegBkqTxbdIdxSpec>;
#[doc = "Register `REG_BKQ_TXBD_IDX` writer"]
pub type W = crate::W<RegBkqTxbdIdxSpec>;
#[doc = "Field `BIT_BKQ_HOST_IDX` reader - BK queue descriptor's host index"]
pub type BitBkqHostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BKQ_HOST_IDX` writer - BK queue descriptor's host index"]
pub type BitBkqHostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_BKQ_HW_IDX` reader - BK queue descriptor's hardware index"]
pub type BitBkqHwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - BK queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_bkq_host_idx(&self) -> BitBkqHostIdxR {
        BitBkqHostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - BK queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_bkq_hw_idx(&self) -> BitBkqHwIdxR {
        BitBkqHwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - BK queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_bkq_host_idx(&mut self) -> BitBkqHostIdxW<'_, RegBkqTxbdIdxSpec> {
        BitBkqHostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bkq_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bkq_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBkqTxbdIdxSpec;
impl crate::RegisterSpec for RegBkqTxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bkq_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegBkqTxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bkq_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegBkqTxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BKQ_TXBD_IDX to value 0"]
impl crate::Resettable for RegBkqTxbdIdxSpec {}
