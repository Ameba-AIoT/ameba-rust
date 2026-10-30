#[doc = "Register `MISR_DOUT_SEL` reader"]
pub type R = crate::R<MisrDoutSelSpec>;
#[doc = "Register `MISR_DOUT_SEL` writer"]
pub type W = crate::W<MisrDoutSelSpec>;
#[doc = "Field `hsys_e1_misr_sel` reader - "]
pub type HsysE1MisrSelR = crate::FieldReader;
#[doc = "Field `hsys_e1_misr_sel` writer - "]
pub type HsysE1MisrSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `aonpmc_e0_misr_sel` reader - "]
pub type AonpmcE0MisrSelR = crate::BitReader;
#[doc = "Field `aonpmc_e0_misr_sel` writer - "]
pub type AonpmcE0MisrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `km4tz_itcm0_e0_misr_sel` reader - "]
pub type Km4tzItcm0E0MisrSelR = crate::FieldReader;
#[doc = "Field `km4tz_itcm0_e0_misr_sel` writer - "]
pub type Km4tzItcm0E0MisrSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hsys_e1_misr_sel(&self) -> HsysE1MisrSelR {
        HsysE1MisrSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn aonpmc_e0_misr_sel(&self) -> AonpmcE0MisrSelR {
        AonpmcE0MisrSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn km4tz_itcm0_e0_misr_sel(&self) -> Km4tzItcm0E0MisrSelR {
        Km4tzItcm0E0MisrSelR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hsys_e1_misr_sel(&mut self) -> HsysE1MisrSelW<'_, MisrDoutSelSpec> {
        HsysE1MisrSelW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn aonpmc_e0_misr_sel(&mut self) -> AonpmcE0MisrSelW<'_, MisrDoutSelSpec> {
        AonpmcE0MisrSelW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn km4tz_itcm0_e0_misr_sel(&mut self) -> Km4tzItcm0E0MisrSelW<'_, MisrDoutSelSpec> {
        Km4tzItcm0E0MisrSelW::new(self, 5)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`misr_dout_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misr_dout_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisrDoutSelSpec;
impl crate::RegisterSpec for MisrDoutSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr_dout_sel::R`](R) reader structure"]
impl crate::Readable for MisrDoutSelSpec {}
#[doc = "`write(|w| ..)` method takes [`misr_dout_sel::W`](W) writer structure"]
impl crate::Writable for MisrDoutSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISR_DOUT_SEL to value 0"]
impl crate::Resettable for MisrDoutSelSpec {}
