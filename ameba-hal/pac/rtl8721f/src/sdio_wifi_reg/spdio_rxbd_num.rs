#[doc = "Register `SPDIO_RXBD_NUM` reader"]
pub type R = crate::R<SpdioRxbdNumSpec>;
#[doc = "Register `SPDIO_RXBD_NUM` writer"]
pub type W = crate::W<SpdioRxbdNumSpec>;
#[doc = "Field `SPDIO_RXBD_NUM` reader - The total count of RX BD The value of this register should be configured by CPU. The unit is BD count"]
pub type SpdioRxbdNumR = crate::FieldReader<u16>;
#[doc = "Field `SPDIO_RXBD_NUM` writer - The total count of RX BD The value of this register should be configured by CPU. The unit is BD count"]
pub type SpdioRxbdNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The total count of RX BD The value of this register should be configured by CPU. The unit is BD count"]
    #[inline(always)]
    pub fn spdio_rxbd_num(&self) -> SpdioRxbdNumR {
        SpdioRxbdNumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - The total count of RX BD The value of this register should be configured by CPU. The unit is BD count"]
    #[inline(always)]
    pub fn spdio_rxbd_num(&mut self) -> SpdioRxbdNumW<'_, SpdioRxbdNumSpec> {
        SpdioRxbdNumW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_rxbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_rxbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioRxbdNumSpec;
impl crate::RegisterSpec for SpdioRxbdNumSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spdio_rxbd_num::R`](R) reader structure"]
impl crate::Readable for SpdioRxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_rxbd_num::W`](W) writer structure"]
impl crate::Writable for SpdioRxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_RXBD_NUM to value 0"]
impl crate::Resettable for SpdioRxbdNumSpec {}
