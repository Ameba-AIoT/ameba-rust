#[doc = "Register `REG_TCUNIT_BASE` reader"]
pub type R = crate::R<RegTcunitBaseSpec>;
#[doc = "Register `REG_TCUNIT_BASE` writer"]
pub type W = crate::W<RegTcunitBaseSpec>;
#[doc = "Field `BIT_TC_UNIT_BASE` reader - The register set the number of clocks that equals to the base time unit of timer/counter. The clock domain is 32k clock domain."]
pub type BitTcUnitBaseR = crate::FieldReader<u16>;
#[doc = "Field `BIT_TC_UNIT_BASE` writer - The register set the number of clocks that equals to the base time unit of timer/counter. The clock domain is 32k clock domain."]
pub type BitTcUnitBaseW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:13 - The register set the number of clocks that equals to the base time unit of timer/counter. The clock domain is 32k clock domain."]
    #[inline(always)]
    pub fn bit_tc_unit_base(&self) -> BitTcUnitBaseR {
        BitTcUnitBaseR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:13 - The register set the number of clocks that equals to the base time unit of timer/counter. The clock domain is 32k clock domain."]
    #[inline(always)]
    pub fn bit_tc_unit_base(&mut self) -> BitTcUnitBaseW<'_, RegTcunitBaseSpec> {
        BitTcUnitBaseW::new(self, 0)
    }
    #[doc = "Bits 14:31 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegTcunitBaseSpec> {
        RsvdW::new(self, 14)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_tcunit_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_tcunit_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTcunitBaseSpec;
impl crate::RegisterSpec for RegTcunitBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_tcunit_base::R`](R) reader structure"]
impl crate::Readable for RegTcunitBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_tcunit_base::W`](W) writer structure"]
impl crate::Writable for RegTcunitBaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TCUNIT_BASE to value 0"]
impl crate::Resettable for RegTcunitBaseSpec {}
