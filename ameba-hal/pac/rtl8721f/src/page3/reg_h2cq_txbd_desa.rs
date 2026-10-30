#[doc = "Register `REG_H2CQ_TXBD_DESA` reader"]
pub type R = crate::R<RegH2cqTxbdDesaSpec>;
#[doc = "Register `REG_H2CQ_TXBD_DESA` writer"]
pub type W = crate::W<RegH2cqTxbdDesaSpec>;
#[doc = "Field `BIT_H2CQDESA` reader - Buffer Descriptor Base Address for H2C Queue"]
pub type BitH2cqdesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_H2CQDESA` writer - Buffer Descriptor Base Address for H2C Queue"]
pub type BitH2cqdesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for H2C Queue"]
    #[inline(always)]
    pub fn bit_h2cqdesa(&self) -> BitH2cqdesaR {
        BitH2cqdesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for H2C Queue"]
    #[inline(always)]
    pub fn bit_h2cqdesa(&mut self) -> BitH2cqdesaW<'_, RegH2cqTxbdDesaSpec> {
        BitH2cqdesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_h2cq_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_h2cq_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegH2cqTxbdDesaSpec;
impl crate::RegisterSpec for RegH2cqTxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_h2cq_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegH2cqTxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_h2cq_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegH2cqTxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_H2CQ_TXBD_DESA to value 0"]
impl crate::Resettable for RegH2cqTxbdDesaSpec {}
