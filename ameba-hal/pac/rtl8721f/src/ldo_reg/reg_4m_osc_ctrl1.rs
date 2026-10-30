#[doc = "Register `REG_4M_OSC_CTRL1` reader"]
pub type R = crate::R<Reg4mOscCtrl1Spec>;
#[doc = "Register `REG_4M_OSC_CTRL1` writer"]
pub type W = crate::W<Reg4mOscCtrl1Spec>;
#[doc = "Field `Freq_R_SEL` reader - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type FreqRSelR = crate::FieldReader;
#[doc = "Field `Freq_R_SEL` writer - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type FreqRSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VCM_SEL_L` reader - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type VcmSelLR = crate::FieldReader;
#[doc = "Field `VCM_SEL_L` writer - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type VcmSelLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VCM_SEL_H` reader - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type VcmSelHR = crate::FieldReader;
#[doc = "Field `VCM_SEL_H` writer - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
pub type VcmSelHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn freq_r_sel(&self) -> FreqRSelR {
        FreqRSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn vcm_sel_l(&self) -> VcmSelLR {
        VcmSelLR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn vcm_sel_h(&self) -> VcmSelHR {
        VcmSelHR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn freq_r_sel(&mut self) -> FreqRSelW<'_, Reg4mOscCtrl1Spec> {
        FreqRSelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn vcm_sel_l(&mut self) -> VcmSelLW<'_, Reg4mOscCtrl1Spec> {
        VcmSelLW::new(self, 8)
    }
    #[doc = "Bits 12:15 - // \\[RF\\]\\[4MOSC\\] REG_CTRL"]
    #[inline(always)]
    pub fn vcm_sel_h(&mut self) -> VcmSelHW<'_, Reg4mOscCtrl1Spec> {
        VcmSelHW::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_4m_osc_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_4m_osc_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg4mOscCtrl1Spec;
impl crate::RegisterSpec for Reg4mOscCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_4m_osc_ctrl1::R`](R) reader structure"]
impl crate::Readable for Reg4mOscCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_4m_osc_ctrl1::W`](W) writer structure"]
impl crate::Writable for Reg4mOscCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_4M_OSC_CTRL1 to value 0x777f"]
impl crate::Resettable for Reg4mOscCtrl1Spec {
    const RESET_VALUE: u32 = 0x777f;
}
