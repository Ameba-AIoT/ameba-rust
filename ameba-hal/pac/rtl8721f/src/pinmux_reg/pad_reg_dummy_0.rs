#[doc = "Register `PAD_REG_DUMMY_0` reader"]
pub type R = crate::R<PadRegDummy0Spec>;
#[doc = "Register `PAD_REG_DUMMY_0` writer"]
pub type W = crate::W<PadRegDummy0Spec>;
#[doc = "Field `dummy` reader - HW used only , these are for reserved."]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - HW used only , these are for reserved."]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HW used only , these are for reserved."]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HW used only , these are for reserved."]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, PadRegDummy0Spec> {
        DummyW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_reg_dummy_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_reg_dummy_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadRegDummy0Spec;
impl crate::RegisterSpec for PadRegDummy0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_reg_dummy_0::R`](R) reader structure"]
impl crate::Readable for PadRegDummy0Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_reg_dummy_0::W`](W) writer structure"]
impl crate::Writable for PadRegDummy0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_REG_DUMMY_0 to value 0xffff"]
impl crate::Resettable for PadRegDummy0Spec {
    const RESET_VALUE: u32 = 0xffff;
}
