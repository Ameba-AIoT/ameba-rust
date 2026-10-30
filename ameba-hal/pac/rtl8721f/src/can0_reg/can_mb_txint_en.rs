#[doc = "Register `CAN_MB_TXINT_EN` reader"]
pub type R = crate::R<CanMbTxintEnSpec>;
#[doc = "Register `CAN_MB_TXINT_EN` writer"]
pub type W = crate::W<CanMbTxintEnSpec>;
#[doc = "Field `can_mb_txint_en` reader - tx interrupt enable for each message buffer"]
pub type CanMbTxintEnR = crate::FieldReader<u16>;
#[doc = "Field `can_mb_txint_en` writer - tx interrupt enable for each message buffer"]
pub type CanMbTxintEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - tx interrupt enable for each message buffer"]
    #[inline(always)]
    pub fn can_mb_txint_en(&self) -> CanMbTxintEnR {
        CanMbTxintEnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - tx interrupt enable for each message buffer"]
    #[inline(always)]
    pub fn can_mb_txint_en(&mut self) -> CanMbTxintEnW<'_, CanMbTxintEnSpec> {
        CanMbTxintEnW::new(self, 0)
    }
}
#[doc = "CAN tx interrupt for message buffers\n\nYou can [`read`](crate::Reg::read) this register and get [`can_mb_txint_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_mb_txint_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanMbTxintEnSpec;
impl crate::RegisterSpec for CanMbTxintEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_mb_txint_en::R`](R) reader structure"]
impl crate::Readable for CanMbTxintEnSpec {}
#[doc = "`write(|w| ..)` method takes [`can_mb_txint_en::W`](W) writer structure"]
impl crate::Writable for CanMbTxintEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_MB_TXINT_EN to value 0"]
impl crate::Resettable for CanMbTxintEnSpec {}
