#[doc = "Register `SPDIO_TXBD_ADDR` reader"]
pub type R = crate::R<SpdioTxbdAddrSpec>;
#[doc = "Register `SPDIO_TXBD_ADDR` writer"]
pub type W = crate::W<SpdioTxbdAddrSpec>;
#[doc = "Field `SPDIO_TXFIFO_ADDR` reader - The base address of TXBD The value of this register should be configure by CPU It should be 4 byte alignment (Because TXBD Size is 8 Byte)."]
pub type SpdioTxfifoAddrR = crate::FieldReader<u32>;
#[doc = "Field `SPDIO_TXFIFO_ADDR` writer - The base address of TXBD The value of this register should be configure by CPU It should be 4 byte alignment (Because TXBD Size is 8 Byte)."]
pub type SpdioTxfifoAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The base address of TXBD The value of this register should be configure by CPU It should be 4 byte alignment (Because TXBD Size is 8 Byte)."]
    #[inline(always)]
    pub fn spdio_txfifo_addr(&self) -> SpdioTxfifoAddrR {
        SpdioTxfifoAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The base address of TXBD The value of this register should be configure by CPU It should be 4 byte alignment (Because TXBD Size is 8 Byte)."]
    #[inline(always)]
    pub fn spdio_txfifo_addr(&mut self) -> SpdioTxfifoAddrW<'_, SpdioTxbdAddrSpec> {
        SpdioTxfifoAddrW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spdio_txbd_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdio_txbd_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdioTxbdAddrSpec;
impl crate::RegisterSpec for SpdioTxbdAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdio_txbd_addr::R`](R) reader structure"]
impl crate::Readable for SpdioTxbdAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`spdio_txbd_addr::W`](W) writer structure"]
impl crate::Writable for SpdioTxbdAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPDIO_TXBD_ADDR to value 0"]
impl crate::Resettable for SpdioTxbdAddrSpec {}
