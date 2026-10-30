#[doc = "Register `REG_BCNQ_TXBD_DESA` reader"]
pub type R = crate::R<RegBcnqTxbdDesaSpec>;
#[doc = "Register `REG_BCNQ_TXBD_DESA` writer"]
pub type W = crate::W<RegBcnqTxbdDesaSpec>;
#[doc = "Field `BIT_BPQDESA` reader - Buffer Descriptor Base Address for Beacon Queue"]
pub type BitBpqdesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BPQDESA` writer - Buffer Descriptor Base Address for Beacon Queue"]
pub type BitBpqdesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for Beacon Queue"]
    #[inline(always)]
    pub fn bit_bpqdesa(&self) -> BitBpqdesaR {
        BitBpqdesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for Beacon Queue"]
    #[inline(always)]
    pub fn bit_bpqdesa(&mut self) -> BitBpqdesaW<'_, RegBcnqTxbdDesaSpec> {
        BitBpqdesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcnq_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcnq_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnqTxbdDesaSpec;
impl crate::RegisterSpec for RegBcnqTxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcnq_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegBcnqTxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcnq_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegBcnqTxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCNQ_TXBD_DESA to value 0"]
impl crate::Resettable for RegBcnqTxbdDesaSpec {}
