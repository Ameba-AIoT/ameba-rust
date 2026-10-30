#[doc = "Register `REG_AX_PWR_RSVD7` reader"]
pub type R = crate::R<RegAxPwrRsvd7Spec>;
#[doc = "Register `REG_AX_PWR_RSVD7` writer"]
pub type W = crate::W<RegAxPwrRsvd7Spec>;
#[doc = "Field `r_sta_ul_force_pwr_path_from_txpwr` reader - "]
pub type RStaUlForcePwrPathFromTxpwrR = crate::BitReader;
#[doc = "Field `r_sta_ul_force_pwr_path_from_txpwr` writer - "]
pub type RStaUlForcePwrPathFromTxpwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sta_ul_force_pwr_path_on` reader - "]
pub type RStaUlForcePwrPathOnR = crate::BitReader;
#[doc = "Field `r_sta_ul_force_pwr_path_on` writer - "]
pub type RStaUlForcePwrPathOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_sta_ul_force_pwr_path_val` reader - "]
pub type RStaUlForcePwrPathValR = crate::FieldReader<u16>;
#[doc = "Field `r_sta_ul_force_pwr_path_val` writer - "]
pub type RStaUlForcePwrPathValW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_sta_ul_PWRmax_offset` reader - "]
pub type RStaUlPwrmaxOffsetR = crate::FieldReader;
#[doc = "Field `r_sta_ul_PWRmax_offset` writer - "]
pub type RStaUlPwrmaxOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_sta_ul_UPH_offset` reader - "]
pub type RStaUlUphOffsetR = crate::FieldReader;
#[doc = "Field `r_sta_ul_UPH_offset` writer - "]
pub type RStaUlUphOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_en_BF_force_on` reader - "]
pub type REnBfForceOnR = crate::BitReader;
#[doc = "Field `r_en_BF_force_on` writer - "]
pub type REnBfForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_sta_ul_force_pwr_path_from_txpwr(&self) -> RStaUlForcePwrPathFromTxpwrR {
        RStaUlForcePwrPathFromTxpwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_sta_ul_force_pwr_path_on(&self) -> RStaUlForcePwrPathOnR {
        RStaUlForcePwrPathOnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10"]
    #[inline(always)]
    pub fn r_sta_ul_force_pwr_path_val(&self) -> RStaUlForcePwrPathValR {
        RStaUlForcePwrPathValR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn r_sta_ul_pwrmax_offset(&self) -> RStaUlPwrmaxOffsetR {
        RStaUlPwrmaxOffsetR::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn r_sta_ul_uph_offset(&self) -> RStaUlUphOffsetR {
        RStaUlUphOffsetR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_en_bf_force_on(&self) -> REnBfForceOnR {
        REnBfForceOnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn r_sta_ul_force_pwr_path_from_txpwr(
        &mut self,
    ) -> RStaUlForcePwrPathFromTxpwrW<'_, RegAxPwrRsvd7Spec> {
        RStaUlForcePwrPathFromTxpwrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn r_sta_ul_force_pwr_path_on(&mut self) -> RStaUlForcePwrPathOnW<'_, RegAxPwrRsvd7Spec> {
        RStaUlForcePwrPathOnW::new(self, 1)
    }
    #[doc = "Bits 2:10"]
    #[inline(always)]
    pub fn r_sta_ul_force_pwr_path_val(&mut self) -> RStaUlForcePwrPathValW<'_, RegAxPwrRsvd7Spec> {
        RStaUlForcePwrPathValW::new(self, 2)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn r_sta_ul_pwrmax_offset(&mut self) -> RStaUlPwrmaxOffsetW<'_, RegAxPwrRsvd7Spec> {
        RStaUlPwrmaxOffsetW::new(self, 12)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn r_sta_ul_uph_offset(&mut self) -> RStaUlUphOffsetW<'_, RegAxPwrRsvd7Spec> {
        RStaUlUphOffsetW::new(self, 20)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn r_en_bf_force_on(&mut self) -> REnBfForceOnW<'_, RegAxPwrRsvd7Spec> {
        REnBfForceOnW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_rsvd7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_rsvd7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrRsvd7Spec;
impl crate::RegisterSpec for RegAxPwrRsvd7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_rsvd7::R`](R) reader structure"]
impl crate::Readable for RegAxPwrRsvd7Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_rsvd7::W`](W) writer structure"]
impl crate::Writable for RegAxPwrRsvd7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_RSVD7 to value 0"]
impl crate::Resettable for RegAxPwrRsvd7Spec {}
