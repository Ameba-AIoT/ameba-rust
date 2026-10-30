#[doc = "Register `REG_INT_MIG` reader"]
pub type R = crate::R<RegIntMigSpec>;
#[doc = "Register `REG_INT_MIG` writer"]
pub type W = crate::W<RegIntMigSpec>;
#[doc = "Field `BIT_MIGRATE_TIMER` reader - Timer for setting Interrupt Migration time, only use \\[2:0\\] 3'b000: 64 us 3'b001: 128 us 3'b010: 256 us 3'b011: 512 us 3'b100: 1024 us 3'b101: 2048 us 3'b110: 4096 us 3'b111: 8192 us"]
pub type BitMigrateTimerR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MIGRATE_TIMER` writer - Timer for setting Interrupt Migration time, only use \\[2:0\\] 3'b000: 64 us 3'b001: 128 us 3'b010: 256 us 3'b011: 512 us 3'b100: 1024 us 3'b101: 2048 us 3'b110: 4096 us 3'b111: 8192 us"]
pub type BitMigrateTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_RNUMMIT` reader - When the buffered RX packet number is matched, the interrupt is then issued"]
pub type BitRnummitR = crate::FieldReader;
#[doc = "Field `BIT_RNUMMIT` writer - When the buffered RX packet number is matched, the interrupt is then issued"]
pub type BitRnummitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_RTMRMIT` reader - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
pub type BitRtmrmitR = crate::FieldReader;
#[doc = "Field `BIT_RTMRMIT` writer - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
pub type BitRtmrmitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_TNUMMIT` reader - When the buffered TX packet number is matched, the interrupt is then issued"]
pub type BitTnummitR = crate::FieldReader;
#[doc = "Field `BIT_TNUMMIT` writer - When the buffered TX packet number is matched, the interrupt is then issued"]
pub type BitTnummitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_TTMRMIT` reader - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
pub type BitTtmrmitR = crate::FieldReader;
#[doc = "Field `BIT_TTMRMIT` writer - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
pub type BitTtmrmitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Timer for setting Interrupt Migration time, only use \\[2:0\\] 3'b000: 64 us 3'b001: 128 us 3'b010: 256 us 3'b011: 512 us 3'b100: 1024 us 3'b101: 2048 us 3'b110: 4096 us 3'b111: 8192 us"]
    #[inline(always)]
    pub fn bit_migrate_timer(&self) -> BitMigrateTimerR {
        BitMigrateTimerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - When the buffered RX packet number is matched, the interrupt is then issued"]
    #[inline(always)]
    pub fn bit_rnummit(&self) -> BitRnummitR {
        BitRnummitR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
    #[inline(always)]
    pub fn bit_rtmrmit(&self) -> BitRtmrmitR {
        BitRtmrmitR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - When the buffered TX packet number is matched, the interrupt is then issued"]
    #[inline(always)]
    pub fn bit_tnummit(&self) -> BitTnummitR {
        BitTnummitR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
    #[inline(always)]
    pub fn bit_ttmrmit(&self) -> BitTtmrmitR {
        BitTtmrmitR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer for setting Interrupt Migration time, only use \\[2:0\\] 3'b000: 64 us 3'b001: 128 us 3'b010: 256 us 3'b011: 512 us 3'b100: 1024 us 3'b101: 2048 us 3'b110: 4096 us 3'b111: 8192 us"]
    #[inline(always)]
    pub fn bit_migrate_timer(&mut self) -> BitMigrateTimerW<'_, RegIntMigSpec> {
        BitMigrateTimerW::new(self, 0)
    }
    #[doc = "Bits 16:19 - When the buffered RX packet number is matched, the interrupt is then issued"]
    #[inline(always)]
    pub fn bit_rnummit(&mut self) -> BitRnummitW<'_, RegIntMigSpec> {
        BitRnummitW::new(self, 16)
    }
    #[doc = "Bits 20:23 - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
    #[inline(always)]
    pub fn bit_rtmrmit(&mut self) -> BitRtmrmitW<'_, RegIntMigSpec> {
        BitRtmrmitW::new(self, 20)
    }
    #[doc = "Bits 24:27 - When the buffered TX packet number is matched, the interrupt is then issued"]
    #[inline(always)]
    pub fn bit_tnummit(&mut self) -> BitTnummitW<'_, RegIntMigSpec> {
        BitTnummitW::new(self, 24)
    }
    #[doc = "Bits 28:31 - When the number of timer matched condition exceeds this setting, then interrupt is issued."]
    #[inline(always)]
    pub fn bit_ttmrmit(&mut self) -> BitTtmrmitW<'_, RegIntMigSpec> {
        BitTtmrmitW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_int_mig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_int_mig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegIntMigSpec;
impl crate::RegisterSpec for RegIntMigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_int_mig::R`](R) reader structure"]
impl crate::Readable for RegIntMigSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_int_mig::W`](W) writer structure"]
impl crate::Writable for RegIntMigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_INT_MIG to value 0"]
impl crate::Resettable for RegIntMigSpec {}
