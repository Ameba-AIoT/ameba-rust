#[doc = "Register `AUTO_LENGTH2` reader"]
pub type R = crate::R<AutoLength2Spec>;
#[doc = "Register `AUTO_LENGTH2` writer"]
pub type W = crate::W<AutoLength2Spec>;
#[doc = "Field `WR_DUMMY_LENGTH` reader - Indicates delay cycles for transmitting data in auto mode. It is referenced by bus_clk. Available only if configuration of PSRAM_EN is defined."]
pub type WrDummyLengthR = crate::FieldReader<u16>;
#[doc = "Field `WR_DUMMY_LENGTH` writer - Indicates delay cycles for transmitting data in auto mode. It is referenced by bus_clk. Available only if configuration of PSRAM_EN is defined."]
pub type WrDummyLengthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Indicates delay cycles for transmitting data in auto mode. It is referenced by bus_clk. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn wr_dummy_length(&self) -> WrDummyLengthR {
        WrDummyLengthR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicates delay cycles for transmitting data in auto mode. It is referenced by bus_clk. Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn wr_dummy_length(&mut self) -> WrDummyLengthW<'_, AutoLength2Spec> {
        WrDummyLengthW::new(self, 0)
    }
}
#[doc = "Auto Address Length Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`auto_length2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auto_length2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoLength2Spec;
impl crate::RegisterSpec for AutoLength2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auto_length2::R`](R) reader structure"]
impl crate::Readable for AutoLength2Spec {}
#[doc = "`write(|w| ..)` method takes [`auto_length2::W`](W) writer structure"]
impl crate::Writable for AutoLength2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTO_LENGTH2 to value 0"]
impl crate::Resettable for AutoLength2Spec {}
