#[doc = "Register `TxFDP3` reader"]
pub type R = crate::R<TxFdp3Spec>;
#[doc = "Register `TxFDP3` writer"]
pub type W = crate::W<TxFdp3Spec>;
#[doc = "Field `TxFDP3` reader - Tx 3st priority Descriptor Pointer to the Tx Ring"]
pub type TxFdp3R = crate::FieldReader<u32>;
#[doc = "Field `TxFDP3` writer - Tx 3st priority Descriptor Pointer to the Tx Ring"]
pub type TxFdp3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx 3st priority Descriptor Pointer to the Tx Ring"]
    #[inline(always)]
    pub fn tx_fdp3(&self) -> TxFdp3R {
        TxFdp3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tx 3st priority Descriptor Pointer to the Tx Ring"]
    #[inline(always)]
    pub fn tx_fdp3(&mut self) -> TxFdp3W<'_, TxFdp3Spec> {
        TxFdp3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fdp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fdp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFdp3Spec;
impl crate::RegisterSpec for TxFdp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fdp3::R`](R) reader structure"]
impl crate::Readable for TxFdp3Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_fdp3::W`](W) writer structure"]
impl crate::Writable for TxFdp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TxFDP3 to value 0"]
impl crate::Resettable for TxFdp3Spec {}
