#[doc = "Register `SDM_CTRL4` reader"]
pub type R = crate::R<SdmCtrl4Spec>;
#[doc = "Register `SDM_CTRL4` writer"]
pub type W = crate::W<SdmCtrl4Spec>;
#[doc = "Field `SDM_DBG_SEL` reader - Set in observed cycles, refer to counters value"]
pub type SdmDbgSelR = crate::FieldReader;
#[doc = "Field `SDM_DBG_SEL` writer - Set in observed cycles, refer to counters value"]
pub type SdmDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDM_DBG_EN` reader - 1: Enable SDM debug"]
pub type SdmDbgEnR = crate::BitReader;
#[doc = "Field `SDM_DBG_EN` writer - 1: Enable SDM debug"]
pub type SdmDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Set in observed cycles, refer to counters value"]
    #[inline(always)]
    pub fn sdm_dbg_sel(&self) -> SdmDbgSelR {
        SdmDbgSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1: Enable SDM debug"]
    #[inline(always)]
    pub fn sdm_dbg_en(&self) -> SdmDbgEnR {
        SdmDbgEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set in observed cycles, refer to counters value"]
    #[inline(always)]
    pub fn sdm_dbg_sel(&mut self) -> SdmDbgSelW<'_, SdmCtrl4Spec> {
        SdmDbgSelW::new(self, 0)
    }
    #[doc = "Bit 3 - 1: Enable SDM debug"]
    #[inline(always)]
    pub fn sdm_dbg_en(&mut self) -> SdmDbgEnW<'_, SdmCtrl4Spec> {
        SdmDbgEnW::new(self, 3)
    }
}
#[doc = "SDM Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmCtrl4Spec;
impl crate::RegisterSpec for SdmCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_ctrl4::R`](R) reader structure"]
impl crate::Readable for SdmCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`sdm_ctrl4::W`](W) writer structure"]
impl crate::Writable for SdmCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_CTRL4 to value 0"]
impl crate::Resettable for SdmCtrl4Spec {}
