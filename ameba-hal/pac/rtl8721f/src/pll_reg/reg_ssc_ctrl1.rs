#[doc = "Register `REG_SSC_CTRL1` reader"]
pub type R = crate::R<RegSscCtrl1Spec>;
#[doc = "Register `REG_SSC_CTRL1` writer"]
pub type W = crate::W<RegSscCtrl1Spec>;
#[doc = "Field `NCODE_SSC` reader - Ncode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
pub type NcodeSscR = crate::FieldReader;
#[doc = "Field `NCODE_SSC` writer - Ncode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
pub type NcodeSscW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FCODE_SSC` reader - Fcode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
pub type FcodeSscR = crate::FieldReader<u16>;
#[doc = "Field `FCODE_SSC` writer - Fcode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
pub type FcodeSscW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:4 - Ncode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
    #[inline(always)]
    pub fn ncode_ssc(&self) -> NcodeSscR {
        NcodeSscR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:18 - Fcode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
    #[inline(always)]
    pub fn fcode_ssc(&self) -> FcodeSscR {
        FcodeSscR::new(((self.bits >> 5) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Ncode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
    #[inline(always)]
    pub fn ncode_ssc(&mut self) -> NcodeSscW<'_, RegSscCtrl1Spec> {
        NcodeSscW::new(self, 0)
    }
    #[doc = "Bits 5:18 - Fcode corresponding to the SSC low-point frequency During SSC, the frequency varies as a triangle wave between \"target frequency\" and \"SSC low-point frequency\". Note: the SSC low-point frequency must be lower than the target frequency"]
    #[inline(always)]
    pub fn fcode_ssc(&mut self) -> FcodeSscW<'_, RegSscCtrl1Spec> {
        FcodeSscW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ssc_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ssc_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSscCtrl1Spec;
impl crate::RegisterSpec for RegSscCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ssc_ctrl1::R`](R) reader structure"]
impl crate::Readable for RegSscCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ssc_ctrl1::W`](W) writer structure"]
impl crate::Writable for RegSscCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SSC_CTRL1 to value 0x08"]
impl crate::Resettable for RegSscCtrl1Spec {
    const RESET_VALUE: u32 = 0x08;
}
