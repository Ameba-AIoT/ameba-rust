#[doc = "Register `REG_VOQ_TXBD_DESA` reader"]
pub type R = crate::R<RegVoqTxbdDesaSpec>;
#[doc = "Register `REG_VOQ_TXBD_DESA` writer"]
pub type W = crate::W<RegVoqTxbdDesaSpec>;
#[doc = "Field `BIT_VOQDESA` reader - Buffer Descriptor Base Address for VO Queue"]
pub type BitVoqdesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_VOQDESA` writer - Buffer Descriptor Base Address for VO Queue"]
pub type BitVoqdesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for VO Queue"]
    #[inline(always)]
    pub fn bit_voqdesa(&self) -> BitVoqdesaR {
        BitVoqdesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for VO Queue"]
    #[inline(always)]
    pub fn bit_voqdesa(&mut self) -> BitVoqdesaW<'_, RegVoqTxbdDesaSpec> {
        BitVoqdesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_voq_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_voq_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegVoqTxbdDesaSpec;
impl crate::RegisterSpec for RegVoqTxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_voq_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegVoqTxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_voq_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegVoqTxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VOQ_TXBD_DESA to value 0"]
impl crate::Resettable for RegVoqTxbdDesaSpec {}
