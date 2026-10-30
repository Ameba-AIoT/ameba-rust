#[doc = "Register `SPDIO_TXBD_NUM` reader"]
pub type R = crate::R<SpdioTxbdNumSpec>;
#[doc = "Register `SPDIO_TXBD_NUM` writer"]
pub type W = crate::W<SpdioTxbdNumSpec>;
#[doc = "Field `SPDIO_TXBD_NUM` reader - The Number of TXBD. Unit: Number The value of this register should be configured by CPU"]
pub type SpdioTxbdNumR = crate::FieldReader<u16>;
#[doc = "Field `SPDIO_TXBD_NUM` writer - The Number of TXBD. Unit: Number The value of this register should be configured by CPU"]
pub type SpdioTxbdNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The Number of TXBD. Unit: Number The value of this register should be configured by CPU"]
    #[inline(always)]
    pub fn spdio_txbd_num(&self) -> SpdioTxbdNumR {
        SpdioTxbdNumR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The Number of TXBD. Unit: Number The value of this register should be configured by CPU"]
    #[inline(always)]
    pub fn spdio_txbd_num(&mut self) -> SpdioTxbdNumW<'_, SpdioTxbdNumSpec> {
        SpdioTxbdNumW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_txbd_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_txbd_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioTxbdNumSpec;
impl crate::RegisterSpec for SpdioTxbdNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_txbd_num::R`](R) reader structure"]
impl crate::Readable for SpdioTxbdNumSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_txbd_num::W`](W) writer structure"]
impl crate::Writable for SpdioTxbdNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_TXBD_NUM to value 0"]
impl crate::Resettable for SpdioTxbdNumSpec {}
