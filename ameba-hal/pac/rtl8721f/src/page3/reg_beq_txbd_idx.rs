#[doc = "Register `REG_BEQ_TXBD_IDX` reader"]
pub type R = crate::R<RegBeqTxbdIdxSpec>;
#[doc = "Register `REG_BEQ_TXBD_IDX` writer"]
pub type W = crate::W<RegBeqTxbdIdxSpec>;
#[doc = "Field `BIT_BEQ_HOST_IDX` reader - BE queue descriptor's host index"]
pub type BitBeqHostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BEQ_HOST_IDX` writer - BE queue descriptor's host index"]
pub type BitBeqHostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_BEQ_HW_IDX` reader - BE queue descriptor's hardware index"]
pub type BitBeqHwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - BE queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_beq_host_idx(&self) -> BitBeqHostIdxR {
        BitBeqHostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - BE queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_beq_hw_idx(&self) -> BitBeqHwIdxR {
        BitBeqHwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - BE queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_beq_host_idx(&mut self) -> BitBeqHostIdxW<'_, RegBeqTxbdIdxSpec> {
        BitBeqHostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_beq_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_beq_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeqTxbdIdxSpec;
impl crate::RegisterSpec for RegBeqTxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_beq_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegBeqTxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_beq_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegBeqTxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BEQ_TXBD_IDX to value 0"]
impl crate::Resettable for RegBeqTxbdIdxSpec {}
