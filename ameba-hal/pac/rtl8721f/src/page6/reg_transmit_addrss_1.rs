#[doc = "Register `REG_TRANSMIT_ADDRSS_1` reader"]
pub type R = crate::R<RegTransmitAddrss1Spec>;
#[doc = "Register `REG_TRANSMIT_ADDRSS_1` writer"]
pub type W = crate::W<RegTransmitAddrss1Spec>;
#[doc = "Field `BIT_TA1_V1` reader - Transmitter address for port 1"]
pub type BitTa1V1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_TA1_V1` writer - Transmitter address for port 1"]
pub type BitTa1V1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitter address for port 1"]
    #[inline(always)]
    pub fn bit_ta1_v1(&self) -> BitTa1V1R {
        BitTa1V1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmitter address for port 1"]
    #[inline(always)]
    pub fn bit_ta1_v1(&mut self) -> BitTa1V1W<'_, RegTransmitAddrss1Spec> {
        BitTa1V1W::new(self, 0)
    }
}
#[doc = "TA1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_transmit_addrss_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_transmit_addrss_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTransmitAddrss1Spec;
impl crate::RegisterSpec for RegTransmitAddrss1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_transmit_addrss_1::R`](R) reader structure"]
impl crate::Readable for RegTransmitAddrss1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_transmit_addrss_1::W`](W) writer structure"]
impl crate::Writable for RegTransmitAddrss1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRANSMIT_ADDRSS_1 to value 0"]
impl crate::Resettable for RegTransmitAddrss1Spec {}
