#[doc = "Register `REG_GMAC_LX_TIMEOUT_CTRL` reader"]
pub type R = crate::R<RegGmacLxTimeoutCtrlSpec>;
#[doc = "Register `REG_GMAC_LX_TIMEOUT_CTRL` writer"]
pub type W = crate::W<RegGmacLxTimeoutCtrlSpec>;
#[doc = "Field `gmac_lx_timeout_sel` reader - sel lxbus tiomeout in gmac"]
pub type GmacLxTimeoutSelR = crate::FieldReader;
#[doc = "Field `gmac_lx_timeout_sel` writer - sel lxbus tiomeout in gmac"]
pub type GmacLxTimeoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gmac_lx_timeout_en` reader - lxbus timeout enable in gmac"]
pub type GmacLxTimeoutEnR = crate::BitReader;
#[doc = "Field `gmac_lx_timeout_en` writer - lxbus timeout enable in gmac"]
pub type GmacLxTimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - sel lxbus tiomeout in gmac"]
    #[inline(always)]
    pub fn gmac_lx_timeout_sel(&self) -> GmacLxTimeoutSelR {
        GmacLxTimeoutSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - lxbus timeout enable in gmac"]
    #[inline(always)]
    pub fn gmac_lx_timeout_en(&self) -> GmacLxTimeoutEnR {
        GmacLxTimeoutEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - sel lxbus tiomeout in gmac"]
    #[inline(always)]
    pub fn gmac_lx_timeout_sel(&mut self) -> GmacLxTimeoutSelW<'_, RegGmacLxTimeoutCtrlSpec> {
        GmacLxTimeoutSelW::new(self, 0)
    }
    #[doc = "Bit 4 - lxbus timeout enable in gmac"]
    #[inline(always)]
    pub fn gmac_lx_timeout_en(&mut self) -> GmacLxTimeoutEnW<'_, RegGmacLxTimeoutCtrlSpec> {
        GmacLxTimeoutEnW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gmac_lx_timeout_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gmac_lx_timeout_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGmacLxTimeoutCtrlSpec;
impl crate::RegisterSpec for RegGmacLxTimeoutCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gmac_lx_timeout_ctrl::R`](R) reader structure"]
impl crate::Readable for RegGmacLxTimeoutCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_gmac_lx_timeout_ctrl::W`](W) writer structure"]
impl crate::Writable for RegGmacLxTimeoutCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GMAC_LX_TIMEOUT_CTRL to value 0"]
impl crate::Resettable for RegGmacLxTimeoutCtrlSpec {}
