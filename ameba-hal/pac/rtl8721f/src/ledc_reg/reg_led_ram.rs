#[doc = "Register `REG_LED_RAM` reader"]
pub type R = crate::R<RegLedRamSpec>;
#[doc = "Register `REG_LED_RAM` writer"]
pub type W = crate::W<RegLedRamSpec>;
#[doc = "Field `r_led_ram_data` reader - 4100_9280~4100_92FF 128bytes 4100_9280~4100_92BF,64bytes used for led display 4100_9280~4100_92C0,65bytes used for psdo i2c"]
pub type RLedRamDataR = crate::FieldReader<u32>;
#[doc = "Field `r_led_ram_data` writer - 4100_9280~4100_92FF 128bytes 4100_9280~4100_92BF,64bytes used for led display 4100_9280~4100_92C0,65bytes used for psdo i2c"]
pub type RLedRamDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 4100_9280~4100_92FF 128bytes 4100_9280~4100_92BF,64bytes used for led display 4100_9280~4100_92C0,65bytes used for psdo i2c"]
    #[inline(always)]
    pub fn r_led_ram_data(&self) -> RLedRamDataR {
        RLedRamDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 4100_9280~4100_92FF 128bytes 4100_9280~4100_92BF,64bytes used for led display 4100_9280~4100_92C0,65bytes used for psdo i2c"]
    #[inline(always)]
    pub fn r_led_ram_data(&mut self) -> RLedRamDataW<'_, RegLedRamSpec> {
        RLedRamDataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_led_ram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_led_ram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLedRamSpec;
impl crate::RegisterSpec for RegLedRamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_led_ram::R`](R) reader structure"]
impl crate::Readable for RegLedRamSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_led_ram::W`](W) writer structure"]
impl crate::Writable for RegLedRamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LED_RAM to value 0"]
impl crate::Resettable for RegLedRamSpec {}
