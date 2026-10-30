#[doc = "Register `ANAPAR_XTAL_OFF_2` reader"]
pub type R = crate::R<AnaparXtalOff2Spec>;
#[doc = "Register `ANAPAR_XTAL_OFF_2` writer"]
pub type W = crate::W<AnaparXtalOff2Spec>;
#[doc = "Field `xtal_off_dummy0` reader - HW used only"]
pub type XtalOffDummy0R = crate::BitReader;
#[doc = "Field `xtal_off_dummy0` writer - HW used only"]
pub type XtalOffDummy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG_T_OPTION` reader - BG options"]
pub type BgTOptionR = crate::FieldReader;
#[doc = "Field `BG_T_OPTION` writer - BG options"]
pub type BgTOptionW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `xtal_off_dummy1` reader - HW used only"]
pub type XtalOffDummy1R = crate::FieldReader;
#[doc = "Field `xtal_off_dummy1` writer - HW used only"]
pub type XtalOffDummy1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - HW used only"]
    #[inline(always)]
    pub fn xtal_off_dummy0(&self) -> XtalOffDummy0R {
        XtalOffDummy0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - BG options"]
    #[inline(always)]
    pub fn bg_t_option(&self) -> BgTOptionR {
        BgTOptionR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:9 - HW used only"]
    #[inline(always)]
    pub fn xtal_off_dummy1(&self) -> XtalOffDummy1R {
        XtalOffDummy1R::new(((self.bits >> 4) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HW used only"]
    #[inline(always)]
    pub fn xtal_off_dummy0(&mut self) -> XtalOffDummy0W<'_, AnaparXtalOff2Spec> {
        XtalOffDummy0W::new(self, 0)
    }
    #[doc = "Bits 1:3 - BG options"]
    #[inline(always)]
    pub fn bg_t_option(&mut self) -> BgTOptionW<'_, AnaparXtalOff2Spec> {
        BgTOptionW::new(self, 1)
    }
    #[doc = "Bits 4:9 - HW used only"]
    #[inline(always)]
    pub fn xtal_off_dummy1(&mut self) -> XtalOffDummy1W<'_, AnaparXtalOff2Spec> {
        XtalOffDummy1W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_off_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_off_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalOff2Spec;
impl crate::RegisterSpec for AnaparXtalOff2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_off_2::R`](R) reader structure"]
impl crate::Readable for AnaparXtalOff2Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_off_2::W`](W) writer structure"]
impl crate::Writable for AnaparXtalOff2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_OFF_2 to value 0x06"]
impl crate::Resettable for AnaparXtalOff2Spec {
    const RESET_VALUE: u32 = 0x06;
}
