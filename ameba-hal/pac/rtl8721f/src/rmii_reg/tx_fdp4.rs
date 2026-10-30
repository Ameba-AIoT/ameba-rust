#[doc = "Register `TxFDP4` reader"]
pub type R = crate::R<TxFdp4Spec>;
#[doc = "Register `TxFDP4` writer"]
pub type W = crate::W<TxFdp4Spec>;
#[doc = "Field `TxFDP4` reader - Tx 4st priority Descriptor Pointer to the Tx Ring"]
pub type TxFdp4R = crate::FieldReader<u32>;
#[doc = "Field `TxFDP4` writer - Tx 4st priority Descriptor Pointer to the Tx Ring"]
pub type TxFdp4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx 4st priority Descriptor Pointer to the Tx Ring"]
    #[inline(always)]
    pub fn tx_fdp4(&self) -> TxFdp4R {
        TxFdp4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tx 4st priority Descriptor Pointer to the Tx Ring"]
    #[inline(always)]
    pub fn tx_fdp4(&mut self) -> TxFdp4W<'_, TxFdp4Spec> {
        TxFdp4W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fdp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fdp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFdp4Spec;
impl crate::RegisterSpec for TxFdp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fdp4::R`](R) reader structure"]
impl crate::Readable for TxFdp4Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_fdp4::W`](W) writer structure"]
impl crate::Writable for TxFdp4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TxFDP4 to value 0"]
impl crate::Resettable for TxFdp4Spec {}
