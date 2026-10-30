#[doc = "Register `REG_MGQ_TXBD_IDX` reader"]
pub type R = crate::R<RegMgqTxbdIdxSpec>;
#[doc = "Register `REG_MGQ_TXBD_IDX` writer"]
pub type W = crate::W<RegMgqTxbdIdxSpec>;
#[doc = "Field `BIT_MGQ_HOST_IDX` reader - MG queue descriptor's host index"]
pub type BitMgqHostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MGQ_HOST_IDX` writer - MG queue descriptor's host index"]
pub type BitMgqHostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_MGQ_HW_IDX` reader - MG queue descriptor's hardware index"]
pub type BitMgqHwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - MG queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mgq_host_idx(&self) -> BitMgqHostIdxR {
        BitMgqHostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MG queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_mgq_hw_idx(&self) -> BitMgqHwIdxR {
        BitMgqHwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - MG queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_mgq_host_idx(&mut self) -> BitMgqHostIdxW<'_, RegMgqTxbdIdxSpec> {
        BitMgqHostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mgq_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mgq_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMgqTxbdIdxSpec;
impl crate::RegisterSpec for RegMgqTxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mgq_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegMgqTxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mgq_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegMgqTxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MGQ_TXBD_IDX to value 0"]
impl crate::Resettable for RegMgqTxbdIdxSpec {}
