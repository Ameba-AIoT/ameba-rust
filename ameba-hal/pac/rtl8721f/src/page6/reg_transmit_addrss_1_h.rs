#[doc = "Register `REG_TRANSMIT_ADDRSS_1_H` reader"]
pub type R = crate::R<RegTransmitAddrss1HSpec>;
#[doc = "Register `REG_TRANSMIT_ADDRSS_1_H` writer"]
pub type W = crate::W<RegTransmitAddrss1HSpec>;
#[doc = "Field `BIT_TA1_H_V1` reader - Transmitter address for port 1"]
pub type BitTa1HV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_TA1_H_V1` writer - Transmitter address for port 1"]
pub type BitTa1HV1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmitter address for port 1"]
    #[inline(always)]
    pub fn bit_ta1_h_v1(&self) -> BitTa1HV1R {
        BitTa1HV1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmitter address for port 1"]
    #[inline(always)]
    pub fn bit_ta1_h_v1(&mut self) -> BitTa1HV1W<'_, RegTransmitAddrss1HSpec> {
        BitTa1HV1W::new(self, 0)
    }
}
#[doc = "TA1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_transmit_addrss_1_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_transmit_addrss_1_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTransmitAddrss1HSpec;
impl crate::RegisterSpec for RegTransmitAddrss1HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_transmit_addrss_1_h::R`](R) reader structure"]
impl crate::Readable for RegTransmitAddrss1HSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_transmit_addrss_1_h::W`](W) writer structure"]
impl crate::Writable for RegTransmitAddrss1HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRANSMIT_ADDRSS_1_H to value 0"]
impl crate::Resettable for RegTransmitAddrss1HSpec {}
