#[doc = "Register `TxFDP5` reader"]
pub type R = crate::R<TxFdp5Spec>;
#[doc = "Register `TxFDP5` writer"]
pub type W = crate::W<TxFdp5Spec>;
#[doc = "Field `TxFDP5` reader - Tx 5st priority Descriptor Pointer to the Tx Ring"]
pub type TxFdp5R = crate::FieldReader<u32>;
#[doc = "Field `TxFDP5` writer - Tx 5st priority Descriptor Pointer to the Tx Ring"]
pub type TxFdp5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx 5st priority Descriptor Pointer to the Tx Ring"]
    #[inline(always)]
    pub fn tx_fdp5(&self) -> TxFdp5R {
        TxFdp5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tx 5st priority Descriptor Pointer to the Tx Ring"]
    #[inline(always)]
    pub fn tx_fdp5(&mut self) -> TxFdp5W<'_, TxFdp5Spec> {
        TxFdp5W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fdp5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fdp5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFdp5Spec;
impl crate::RegisterSpec for TxFdp5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fdp5::R`](R) reader structure"]
impl crate::Readable for TxFdp5Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_fdp5::W`](W) writer structure"]
impl crate::Writable for TxFdp5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TxFDP5 to value 0"]
impl crate::Resettable for TxFdp5Spec {}
