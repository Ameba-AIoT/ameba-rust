#[doc = "Register `REG_BACAM0_ENTRY19` reader"]
pub type R = crate::R<RegBacam0Entry19Spec>;
#[doc = "Register `REG_BACAM0_ENTRY19` writer"]
pub type W = crate::W<RegBacam0Entry19Spec>;
#[doc = "Field `BACAM0_ENTRY19_INFO_TA0` reader - "]
pub type Bacam0Entry19InfoTa0R = crate::FieldReader<u32>;
#[doc = "Field `BACAM0_ENTRY19_INFO_TA0` writer - "]
pub type Bacam0Entry19InfoTa0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam0_entry19_info_ta0(&self) -> Bacam0Entry19InfoTa0R {
        Bacam0Entry19InfoTa0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bacam0_entry19_info_ta0(&mut self) -> Bacam0Entry19InfoTa0W<'_, RegBacam0Entry19Spec> {
        Bacam0Entry19InfoTa0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bacam0_entry19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bacam0_entry19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBacam0Entry19Spec;
impl crate::RegisterSpec for RegBacam0Entry19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bacam0_entry19::R`](R) reader structure"]
impl crate::Readable for RegBacam0Entry19Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bacam0_entry19::W`](W) writer structure"]
impl crate::Writable for RegBacam0Entry19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BACAM0_ENTRY19 to value 0"]
impl crate::Resettable for RegBacam0Entry19Spec {}
