#[doc = "Register `REG_BKQ_TXBD_DESA` reader"]
pub type R = crate::R<RegBkqTxbdDesaSpec>;
#[doc = "Register `REG_BKQ_TXBD_DESA` writer"]
pub type W = crate::W<RegBkqTxbdDesaSpec>;
#[doc = "Field `BIT_BKQDESA` reader - Buffer Descriptor Base Address for BK Queue"]
pub type BitBkqdesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BKQDESA` writer - Buffer Descriptor Base Address for BK Queue"]
pub type BitBkqdesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for BK Queue"]
    #[inline(always)]
    pub fn bit_bkqdesa(&self) -> BitBkqdesaR {
        BitBkqdesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for BK Queue"]
    #[inline(always)]
    pub fn bit_bkqdesa(&mut self) -> BitBkqdesaW<'_, RegBkqTxbdDesaSpec> {
        BitBkqdesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bkq_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bkq_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBkqTxbdDesaSpec;
impl crate::RegisterSpec for RegBkqTxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bkq_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegBkqTxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bkq_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegBkqTxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BKQ_TXBD_DESA to value 0"]
impl crate::Resettable for RegBkqTxbdDesaSpec {}
