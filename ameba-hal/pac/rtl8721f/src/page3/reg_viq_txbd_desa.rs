#[doc = "Register `REG_VIQ_TXBD_DESA` reader"]
pub type R = crate::R<RegViqTxbdDesaSpec>;
#[doc = "Register `REG_VIQ_TXBD_DESA` writer"]
pub type W = crate::W<RegViqTxbdDesaSpec>;
#[doc = "Field `BIT_VIQDESA` reader - Buffer Descriptor Base Address for VI Queue"]
pub type BitViqdesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_VIQDESA` writer - Buffer Descriptor Base Address for VI Queue"]
pub type BitViqdesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for VI Queue"]
    #[inline(always)]
    pub fn bit_viqdesa(&self) -> BitViqdesaR {
        BitViqdesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for VI Queue"]
    #[inline(always)]
    pub fn bit_viqdesa(&mut self) -> BitViqdesaW<'_, RegViqTxbdDesaSpec> {
        BitViqdesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_viq_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_viq_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegViqTxbdDesaSpec;
impl crate::RegisterSpec for RegViqTxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_viq_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegViqTxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_viq_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegViqTxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VIQ_TXBD_DESA to value 0"]
impl crate::Resettable for RegViqTxbdDesaSpec {}
