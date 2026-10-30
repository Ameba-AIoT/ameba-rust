#[doc = "Register `REG_SSC_CTRL0` reader"]
pub type R = crate::R<RegSscCtrl0Spec>;
#[doc = "Register `REG_SSC_CTRL0` writer"]
pub type W = crate::W<RegSscCtrl0Spec>;
#[doc = "Field `NCODE_T_SCPU` reader - Ncode corresponding to the VCO target frequency"]
pub type NcodeTScpuR = crate::FieldReader;
#[doc = "Field `NCODE_T_SCPU` writer - Ncode corresponding to the VCO target frequency"]
pub type NcodeTScpuW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FCODE_T_SCPU` reader - Fcode corresponding to the VCO target frequency"]
pub type FcodeTScpuR = crate::FieldReader<u16>;
#[doc = "Field `FCODE_T_SCPU` writer - Fcode corresponding to the VCO target frequency"]
pub type FcodeTScpuW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:4 - Ncode corresponding to the VCO target frequency"]
    #[inline(always)]
    pub fn ncode_t_scpu(&self) -> NcodeTScpuR {
        NcodeTScpuR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:18 - Fcode corresponding to the VCO target frequency"]
    #[inline(always)]
    pub fn fcode_t_scpu(&self) -> FcodeTScpuR {
        FcodeTScpuR::new(((self.bits >> 5) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Ncode corresponding to the VCO target frequency"]
    #[inline(always)]
    pub fn ncode_t_scpu(&mut self) -> NcodeTScpuW<'_, RegSscCtrl0Spec> {
        NcodeTScpuW::new(self, 0)
    }
    #[doc = "Bits 5:18 - Fcode corresponding to the VCO target frequency"]
    #[inline(always)]
    pub fn fcode_t_scpu(&mut self) -> FcodeTScpuW<'_, RegSscCtrl0Spec> {
        FcodeTScpuW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ssc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ssc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSscCtrl0Spec;
impl crate::RegisterSpec for RegSscCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ssc_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegSscCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ssc_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegSscCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SSC_CTRL0 to value 0x08"]
impl crate::Resettable for RegSscCtrl0Spec {
    const RESET_VALUE: u32 = 0x08;
}
