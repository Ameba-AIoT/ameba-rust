#[doc = "Register `REG_GMAC_LX_TIMEOUT_STATUS` reader"]
pub type R = crate::R<RegGmacLxTimeoutStatusSpec>;
#[doc = "Field `gmac_lx_slv_timeout` reader - lxbus slave timeout status in gmac"]
pub type GmacLxSlvTimeoutR = crate::BitReader;
#[doc = "Field `gmac_lx_mst_timeout` reader - lxbus master timeout status in gmac"]
pub type GmacLxMstTimeoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - lxbus slave timeout status in gmac"]
    #[inline(always)]
    pub fn gmac_lx_slv_timeout(&self) -> GmacLxSlvTimeoutR {
        GmacLxSlvTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - lxbus master timeout status in gmac"]
    #[inline(always)]
    pub fn gmac_lx_mst_timeout(&self) -> GmacLxMstTimeoutR {
        GmacLxMstTimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gmac_lx_timeout_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGmacLxTimeoutStatusSpec;
impl crate::RegisterSpec for RegGmacLxTimeoutStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gmac_lx_timeout_status::R`](R) reader structure"]
impl crate::Readable for RegGmacLxTimeoutStatusSpec {}
#[doc = "`reset()` method sets REG_GMAC_LX_TIMEOUT_STATUS to value 0"]
impl crate::Resettable for RegGmacLxTimeoutStatusSpec {}
