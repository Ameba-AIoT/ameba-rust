#[doc = "Register `REG_TRANSMIT_ADDRSS_0_H` reader"]
pub type R = crate::R<RegTransmitAddrss0HSpec>;
#[doc = "Register `REG_TRANSMIT_ADDRSS_0_H` writer"]
pub type W = crate::W<RegTransmitAddrss0HSpec>;
#[doc = "Field `BIT_TA0_H_V1` reader - Transmitter address for port 0"]
pub type BitTa0HV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_TA0_H_V1` writer - Transmitter address for port 0"]
pub type BitTa0HV1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitter address for port 0"]
    #[inline(always)]
    pub fn bit_ta0_h_v1(&self) -> BitTa0HV1R {
        BitTa0HV1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmitter address for port 0"]
    #[inline(always)]
    pub fn bit_ta0_h_v1(&mut self) -> BitTa0HV1W<'_, RegTransmitAddrss0HSpec> {
        BitTa0HV1W::new(self, 0)
    }
}
#[doc = "TA0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_transmit_addrss_0_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_transmit_addrss_0_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTransmitAddrss0HSpec;
impl crate::RegisterSpec for RegTransmitAddrss0HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_transmit_addrss_0_h::R`](R) reader structure"]
impl crate::Readable for RegTransmitAddrss0HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_transmit_addrss_0_h::W`](W) writer structure"]
impl crate::Writable for RegTransmitAddrss0HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRANSMIT_ADDRSS_0_H to value 0"]
impl crate::Resettable for RegTransmitAddrss0HSpec {}
