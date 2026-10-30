#[doc = "Register `CAN_MB_RXINT_EN` reader"]
pub type R = crate::R<CanMbRxintEnSpec>;
#[doc = "Register `CAN_MB_RXINT_EN` writer"]
pub type W = crate::W<CanMbRxintEnSpec>;
#[doc = "Field `can_mb_rxint_en` reader - rx interrupt enable for each message buffer"]
pub type CanMbRxintEnR = crate::FieldReader<u16>;
#[doc = "Field `can_mb_rxint_en` writer - rx interrupt enable for each message buffer"]
pub type CanMbRxintEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - rx interrupt enable for each message buffer"]
    #[inline(always)]
    pub fn can_mb_rxint_en(&self) -> CanMbRxintEnR {
        CanMbRxintEnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - rx interrupt enable for each message buffer"]
    #[inline(always)]
    pub fn can_mb_rxint_en(&mut self) -> CanMbRxintEnW<'_, CanMbRxintEnSpec> {
        CanMbRxintEnW::new(self, 0)
    }
}
#[doc = "CAN rx interrupt for message buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb_rxint_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb_rxint_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMbRxintEnSpec;
impl crate::RegisterSpec for CanMbRxintEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb_rxint_en::R`](R) reader structure"]
impl crate::Readable for CanMbRxintEnSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb_rxint_en::W`](W) writer structure"]
impl crate::Writable for CanMbRxintEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB_RXINT_EN to value 0"]
impl crate::Resettable for CanMbRxintEnSpec {}
