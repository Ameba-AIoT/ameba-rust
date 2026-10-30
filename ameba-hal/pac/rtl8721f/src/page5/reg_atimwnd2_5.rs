#[doc = "Register `REG_ATIMWND2_5` reader"]
pub type R = crate::R<RegAtimwnd2_5Spec>;
#[doc = "Register `REG_ATIMWND2_5` writer"]
pub type W = crate::W<RegAtimwnd2_5Spec>;
#[doc = "Field `BIT_ATIMWND2` reader - ATIM window width of VAP2 in units of TU."]
pub type BitAtimwnd2R = crate::FieldReader;
#[doc = "Field `BIT_ATIMWND2` writer - ATIM window width of VAP2 in units of TU."]
pub type BitAtimwnd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ATIMWND3` reader - ATIM window width of VAP3 in units of TU."]
pub type BitAtimwnd3R = crate::FieldReader;
#[doc = "Field `BIT_ATIMWND3` writer - ATIM window width of VAP3 in units of TU."]
pub type BitAtimwnd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ATIMWND4` reader - ATIM window width of VAP4 in units of TU."]
pub type BitAtimwnd4R = crate::FieldReader;
#[doc = "Field `BIT_ATIMWND4` writer - ATIM window width of VAP4 in units of TU."]
pub type BitAtimwnd4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ATIMWND5` reader - ATIM window width of VAP5 in units of TU."]
pub type BitAtimwnd5R = crate::FieldReader;
#[doc = "Field `BIT_ATIMWND5` writer - ATIM window width of VAP5 in units of TU."]
pub type BitAtimwnd5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ATIM window width of VAP2 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd2(&self) -> BitAtimwnd2R {
        BitAtimwnd2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATIM window width of VAP3 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd3(&self) -> BitAtimwnd3R {
        BitAtimwnd3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATIM window width of VAP4 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd4(&self) -> BitAtimwnd4R {
        BitAtimwnd4R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ATIM window width of VAP5 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd5(&self) -> BitAtimwnd5R {
        BitAtimwnd5R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATIM window width of VAP2 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd2(&mut self) -> BitAtimwnd2W<'_, RegAtimwnd2_5Spec> {
        BitAtimwnd2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ATIM window width of VAP3 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd3(&mut self) -> BitAtimwnd3W<'_, RegAtimwnd2_5Spec> {
        BitAtimwnd3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - ATIM window width of VAP4 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd4(&mut self) -> BitAtimwnd4W<'_, RegAtimwnd2_5Spec> {
        BitAtimwnd4W::new(self, 16)
    }
    #[doc = "Bits 24:31 - ATIM window width of VAP5 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd5(&mut self) -> BitAtimwnd5W<'_, RegAtimwnd2_5Spec> {
        BitAtimwnd5W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_atimwnd2_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_atimwnd2_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAtimwnd2_5Spec;
impl crate::RegisterSpec for RegAtimwnd2_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_atimwnd2_5::R`](R) reader structure"]
impl crate::Readable for RegAtimwnd2_5Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_atimwnd2_5::W`](W) writer structure"]
impl crate::Writable for RegAtimwnd2_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ATIMWND2_5 to value 0x0202_0202"]
impl crate::Resettable for RegAtimwnd2_5Spec {
    const RESET_VALUE: u32 = 0x0202_0202;
}
