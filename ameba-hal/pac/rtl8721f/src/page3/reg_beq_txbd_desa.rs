#[doc = "Register `REG_BEQ_TXBD_DESA` reader"]
pub type R = crate::R<RegBeqTxbdDesaSpec>;
#[doc = "Register `REG_BEQ_TXBD_DESA` writer"]
pub type W = crate::W<RegBeqTxbdDesaSpec>;
#[doc = "Field `BIT_BEQDESA` reader - Buffer Descriptor Base Address for BE Queue"]
pub type BitBeqdesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BEQDESA` writer - Buffer Descriptor Base Address for BE Queue"]
pub type BitBeqdesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for BE Queue"]
    #[inline(always)]
    pub fn bit_beqdesa(&self) -> BitBeqdesaR {
        BitBeqdesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for BE Queue"]
    #[inline(always)]
    pub fn bit_beqdesa(&mut self) -> BitBeqdesaW<'_, RegBeqTxbdDesaSpec> {
        BitBeqdesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_beq_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_beq_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeqTxbdDesaSpec;
impl crate::RegisterSpec for RegBeqTxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_beq_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegBeqTxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_beq_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegBeqTxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BEQ_TXBD_DESA to value 0"]
impl crate::Resettable for RegBeqTxbdDesaSpec {}
