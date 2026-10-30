#[doc = "Register `REG_BCN_ELY_ADJ` reader"]
pub type R = crate::R<RegBcnElyAdjSpec>;
#[doc = "Register `REG_BCN_ELY_ADJ` writer"]
pub type W = crate::W<RegBcnElyAdjSpec>;
#[doc = "Field `BIT_BCN_ELY_ADJ` reader - beacon early time adjust, unit is 32us"]
pub type BitBcnElyAdjR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BCN_ELY_ADJ` writer - beacon early time adjust, unit is 32us"]
pub type BitBcnElyAdjW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - beacon early time adjust, unit is 32us"]
    #[inline(always)]
    pub fn bit_bcn_ely_adj(&self) -> BitBcnElyAdjR {
        BitBcnElyAdjR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - beacon early time adjust, unit is 32us"]
    #[inline(always)]
    pub fn bit_bcn_ely_adj(&mut self) -> BitBcnElyAdjW<'_, RegBcnElyAdjSpec> {
        BitBcnElyAdjW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegBcnElyAdjSpec> {
        DummyW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_ely_adj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_ely_adj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnElyAdjSpec;
impl crate::RegisterSpec for RegBcnElyAdjSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_ely_adj::R`](R) reader structure"]
impl crate::Readable for RegBcnElyAdjSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_ely_adj::W`](W) writer structure"]
impl crate::Writable for RegBcnElyAdjSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_ELY_ADJ to value 0"]
impl crate::Resettable for RegBcnElyAdjSpec {}
