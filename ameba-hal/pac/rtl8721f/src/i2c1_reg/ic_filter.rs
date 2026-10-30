#[doc = "Register `IC_FILTER` reader"]
pub type R = crate::R<IcFilterSpec>;
#[doc = "Register `IC_FILTER` writer"]
pub type W = crate::W<IcFilterSpec>;
#[doc = "Field `IC_DIG_FLTR_DEG` reader - DIG_FLTR_DEG is to define frequency range of filter. A greater value of DIG_FLTR_DEG results in a slower transfer speed and hardware would be able to filter a lower frequency."]
pub type IcDigFltrDegR = crate::FieldReader;
#[doc = "Field `IC_DIG_FLTR_DEG` writer - DIG_FLTR_DEG is to define frequency range of filter. A greater value of DIG_FLTR_DEG results in a slower transfer speed and hardware would be able to filter a lower frequency."]
pub type IcDigFltrDegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IC_DIG_FLTR_SEL` reader - 1: Enable filter"]
pub type IcDigFltrSelR = crate::BitReader;
#[doc = "Field `IC_DIG_FLTR_SEL` writer - 1: Enable filter"]
pub type IcDigFltrSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - DIG_FLTR_DEG is to define frequency range of filter. A greater value of DIG_FLTR_DEG results in a slower transfer speed and hardware would be able to filter a lower frequency."]
    #[inline(always)]
    pub fn ic_dig_fltr_deg(&self) -> IcDigFltrDegR {
        IcDigFltrDegR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 1: Enable filter"]
    #[inline(always)]
    pub fn ic_dig_fltr_sel(&self) -> IcDigFltrSelR {
        IcDigFltrSelR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DIG_FLTR_DEG is to define frequency range of filter. A greater value of DIG_FLTR_DEG results in a slower transfer speed and hardware would be able to filter a lower frequency."]
    #[inline(always)]
    pub fn ic_dig_fltr_deg(&mut self) -> IcDigFltrDegW<'_, IcFilterSpec> {
        IcDigFltrDegW::new(self, 0)
    }
    #[doc = "Bit 8 - 1: Enable filter"]
    #[inline(always)]
    pub fn ic_dig_fltr_sel(&mut self) -> IcDigFltrSelW<'_, IcFilterSpec> {
        IcDigFltrSelW::new(self, 8)
    }
}
#[doc = "I2C Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcFilterSpec;
impl crate::RegisterSpec for IcFilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_filter::R`](R) reader structure"]
impl crate::Readable for IcFilterSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_filter::W`](W) writer structure"]
impl crate::Writable for IcFilterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_FILTER to value 0"]
impl crate::Resettable for IcFilterSpec {}
