#[doc = "Register `REG_VOQ_TXBD_IDX` reader"]
pub type R = crate::R<RegVoqTxbdIdxSpec>;
#[doc = "Register `REG_VOQ_TXBD_IDX` writer"]
pub type W = crate::W<RegVoqTxbdIdxSpec>;
#[doc = "Field `BIT_VOQ_HOST_IDX` reader - VO queue descriptor's host index"]
pub type BitVoqHostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_VOQ_HOST_IDX` writer - VO queue descriptor's host index"]
pub type BitVoqHostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_VOQ_HW_IDX` reader - VO queue descriptor's hardware index"]
pub type BitVoqHwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - VO queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_voq_host_idx(&self) -> BitVoqHostIdxR {
        BitVoqHostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - VO queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_voq_hw_idx(&self) -> BitVoqHwIdxR {
        BitVoqHwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - VO queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_voq_host_idx(&mut self) -> BitVoqHostIdxW<'_, RegVoqTxbdIdxSpec> {
        BitVoqHostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_voq_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_voq_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegVoqTxbdIdxSpec;
impl crate::RegisterSpec for RegVoqTxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_voq_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegVoqTxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_voq_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegVoqTxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VOQ_TXBD_IDX to value 0"]
impl crate::Resettable for RegVoqTxbdIdxSpec {}
