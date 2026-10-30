#[doc = "Register `REG_TRXFF_BNDY` reader"]
pub type R = crate::R<RegTrxffBndySpec>;
#[doc = "Register `REG_TRXFF_BNDY` writer"]
pub type W = crate::W<RegTrxffBndySpec>;
#[doc = "Field `BIT_RXFFOVFL_RSV` reader - Use this register to set reserved entries number of rxpktbuffer for overflow judgment. The rxpktbuffer overflow will be pre-asserted if the left entries count meets the value set here."]
pub type BitRxffovflRsvR = crate::FieldReader;
#[doc = "Field `BIT_RXFFOVFL_RSV` writer - Use this register to set reserved entries number of rxpktbuffer for overflow judgment. The rxpktbuffer overflow will be pre-asserted if the left entries count meets the value set here."]
pub type BitRxffovflRsvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_RXPKT0ENDADDR` reader - This register sets the upper boundary of RXFF0. The unit is 1 byte. The available memory space in RXPKTBUF for RXFF0 is configurable. The value is set to the physical LX bus address."]
pub type BitRxpkt0endaddrR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXPKT0ENDADDR` writer - This register sets the upper boundary of RXFF0. The unit is 1 byte. The available memory space in RXPKTBUF for RXFF0 is configurable. The value is set to the physical LX bus address."]
pub type BitRxpkt0endaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:11 - Use this register to set reserved entries number of rxpktbuffer for overflow judgment. The rxpktbuffer overflow will be pre-asserted if the left entries count meets the value set here."]
    #[inline(always)]
    pub fn bit_rxffovfl_rsv(&self) -> BitRxffovflRsvR {
        BitRxffovflRsvR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - This register sets the upper boundary of RXFF0. The unit is 1 byte. The available memory space in RXPKTBUF for RXFF0 is configurable. The value is set to the physical LX bus address."]
    #[inline(always)]
    pub fn bit_rxpkt0endaddr(&self) -> BitRxpkt0endaddrR {
        BitRxpkt0endaddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:11 - Use this register to set reserved entries number of rxpktbuffer for overflow judgment. The rxpktbuffer overflow will be pre-asserted if the left entries count meets the value set here."]
    #[inline(always)]
    pub fn bit_rxffovfl_rsv(&mut self) -> BitRxffovflRsvW<'_, RegTrxffBndySpec> {
        BitRxffovflRsvW::new(self, 8)
    }
    #[doc = "Bits 12:15 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTrxffBndySpec> {
        RsvdW::new(self, 12)
    }
    #[doc = "Bits 16:31 - This register sets the upper boundary of RXFF0. The unit is 1 byte. The available memory space in RXPKTBUF for RXFF0 is configurable. The value is set to the physical LX bus address."]
    #[inline(always)]
    pub fn bit_rxpkt0endaddr(&mut self) -> BitRxpkt0endaddrW<'_, RegTrxffBndySpec> {
        BitRxpkt0endaddrW::new(self, 16)
    }
}
#[doc = "FIFO Boundary control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_trxff_bndy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_trxff_bndy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTrxffBndySpec;
impl crate::RegisterSpec for RegTrxffBndySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_trxff_bndy::R`](R) reader structure"]
impl crate::Readable for RegTrxffBndySpec {}
#[doc = "`write(|w| ..)` method takes [`reg_trxff_bndy::W`](W) writer structure"]
impl crate::Writable for RegTrxffBndySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRXFF_BNDY to value 0x3fff_0f00"]
impl crate::Resettable for RegTrxffBndySpec {
    const RESET_VALUE: u32 = 0x3fff_0f00;
}
