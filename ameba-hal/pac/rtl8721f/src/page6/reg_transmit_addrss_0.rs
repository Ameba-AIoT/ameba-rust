#[doc = "Register `REG_TRANSMIT_ADDRSS_0` reader"]
pub type R = crate::R<RegTransmitAddrss0Spec>;
#[doc = "Register `REG_TRANSMIT_ADDRSS_0` writer"]
pub type W = crate::W<RegTransmitAddrss0Spec>;
#[doc = "Field `BIT_TA0_v1` reader - Transmitter address for port 0"]
pub type BitTa0V1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_TA0_v1` writer - Transmitter address for port 0"]
pub type BitTa0V1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitter address for port 0"]
    #[inline(always)]
    pub fn bit_ta0_v1(&self) -> BitTa0V1R {
        BitTa0V1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmitter address for port 0"]
    #[inline(always)]
    pub fn bit_ta0_v1(&mut self) -> BitTa0V1W<'_, RegTransmitAddrss0Spec> {
        BitTa0V1W::new(self, 0)
    }
}
#[doc = "TA0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_transmit_addrss_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_transmit_addrss_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTransmitAddrss0Spec;
impl crate::RegisterSpec for RegTransmitAddrss0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_transmit_addrss_0::R`](R) reader structure"]
impl crate::Readable for RegTransmitAddrss0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_transmit_addrss_0::W`](W) writer structure"]
impl crate::Writable for RegTransmitAddrss0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TRANSMIT_ADDRSS_0 to value 0"]
impl crate::Resettable for RegTransmitAddrss0Spec {}
