#[doc = "Register `REG_BACAM1_ENTRY25` reader"]
pub type R = crate::R<RegBacam1Entry25Spec>;
#[doc = "Register `REG_BACAM1_ENTRY25` writer"]
pub type W = crate::W<RegBacam1Entry25Spec>;
#[doc = "Field `BACAM1_ENTRY25_INFO_TA1` reader - "]
pub type Bacam1Entry25InfoTa1R = crate::FieldReader<u16>;
#[doc = "Field `BACAM1_ENTRY25_INFO_TA1` writer - "]
pub type Bacam1Entry25InfoTa1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BACAM1_ENTRY25_INFO_TID` reader - "]
pub type Bacam1Entry25InfoTidR = crate::FieldReader;
#[doc = "Field `BACAM1_ENTRY25_INFO_TID` writer - "]
pub type Bacam1Entry25InfoTidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BACAM1_ENTRY25_INFO_SSN` reader - "]
pub type Bacam1Entry25InfoSsnR = crate::FieldReader<u16>;
#[doc = "Field `BACAM1_ENTRY25_INFO_SSN` writer - "]
pub type Bacam1Entry25InfoSsnW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bacam1_entry25_info_ta1(&self) -> Bacam1Entry25InfoTa1R {
        Bacam1Entry25InfoTa1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn bacam1_entry25_info_tid(&self) -> Bacam1Entry25InfoTidR {
        Bacam1Entry25InfoTidR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn bacam1_entry25_info_ssn(&self) -> Bacam1Entry25InfoSsnR {
        Bacam1Entry25InfoSsnR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bacam1_entry25_info_ta1(&mut self) -> Bacam1Entry25InfoTa1W<'_, RegBacam1Entry25Spec> {
        Bacam1Entry25InfoTa1W::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn bacam1_entry25_info_tid(&mut self) -> Bacam1Entry25InfoTidW<'_, RegBacam1Entry25Spec> {
        Bacam1Entry25InfoTidW::new(self, 16)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn bacam1_entry25_info_ssn(&mut self) -> Bacam1Entry25InfoSsnW<'_, RegBacam1Entry25Spec> {
        Bacam1Entry25InfoSsnW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam1_entry25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam1_entry25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacam1Entry25Spec;
impl crate::RegisterSpec for RegBacam1Entry25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam1_entry25::R`](R) reader structure"]
impl crate::Readable for RegBacam1Entry25Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam1_entry25::W`](W) writer structure"]
impl crate::Writable for RegBacam1Entry25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM1_ENTRY25 to value 0"]
impl crate::Resettable for RegBacam1Entry25Spec {}
