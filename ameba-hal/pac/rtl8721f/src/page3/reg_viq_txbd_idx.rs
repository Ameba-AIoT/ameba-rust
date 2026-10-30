#[doc = "Register `REG_VIQ_TXBD_IDX` reader"]
pub type R = crate::R<RegViqTxbdIdxSpec>;
#[doc = "Register `REG_VIQ_TXBD_IDX` writer"]
pub type W = crate::W<RegViqTxbdIdxSpec>;
#[doc = "Field `BIT_VIQ_HOST_IDX` reader - VI queue descriptor's host index"]
pub type BitViqHostIdxR = crate::FieldReader<u16>;
#[doc = "Field `BIT_VIQ_HOST_IDX` writer - VI queue descriptor's host index"]
pub type BitViqHostIdxW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_VIQ_HW_IDX` reader - VI queue descriptor's hardware index"]
pub type BitViqHwIdxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - VI queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_viq_host_idx(&self) -> BitViqHostIdxR {
        BitViqHostIdxR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - VI queue descriptor's hardware index"]
    #[inline(always)]
    pub fn bit_viq_hw_idx(&self) -> BitViqHwIdxR {
        BitViqHwIdxR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - VI queue descriptor's host index"]
    #[inline(always)]
    pub fn bit_viq_host_idx(&mut self) -> BitViqHostIdxW<'_, RegViqTxbdIdxSpec> {
        BitViqHostIdxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_viq_txbd_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_viq_txbd_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegViqTxbdIdxSpec;
impl crate::RegisterSpec for RegViqTxbdIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_viq_txbd_idx::R`](R) reader structure"]
impl crate::Readable for RegViqTxbdIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_viq_txbd_idx::W`](W) writer structure"]
impl crate::Writable for RegViqTxbdIdxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VIQ_TXBD_IDX to value 0"]
impl crate::Resettable for RegViqTxbdIdxSpec {}
