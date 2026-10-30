#[doc = "Register `REG_AX_PWR_UL_FORCE0` reader"]
pub type R = crate::R<RegAxPwrUlForce0Spec>;
#[doc = "Register `REG_AX_PWR_UL_FORCE0` writer"]
pub type W = crate::W<RegAxPwrUlForce0Spec>;
#[doc = "Field `r_force_TF_RSSI_Target_value` reader - (RTL_only) Force input from MAC"]
pub type RForceTfRssiTargetValueR = crate::FieldReader;
#[doc = "Field `r_force_TF_RSSI_Target_value` writer - (RTL_only) Force input from MAC"]
pub type RForceTfRssiTargetValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_force_TF_RSSI_Target_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceTfRssiTargetEnR = crate::BitReader;
#[doc = "Field `r_force_TF_RSSI_Target_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceTfRssiTargetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_TF_macid_value` reader - (RTL_only) Force input from MAC"]
pub type RForceTfMacidValueR = crate::FieldReader;
#[doc = "Field `r_force_TF_macid_value` writer - (RTL_only) Force input from MAC"]
pub type RForceTfMacidValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `r_force_TF_macid_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceTfMacidEnR = crate::BitReader;
#[doc = "Field `r_force_TF_macid_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceTfMacidEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_force_RU_alloc_value` reader - (RTL_only) Force input from MAC"]
pub type RForceRuAllocValueR = crate::FieldReader;
#[doc = "Field `r_force_RU_alloc_value` writer - (RTL_only) Force input from MAC"]
pub type RForceRuAllocValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_force_RU_alloc_en` reader - (RTL_only) Force input from MAC en"]
pub type RForceRuAllocEnR = crate::BitReader;
#[doc = "Field `r_force_RU_alloc_en` writer - (RTL_only) Force input from MAC en"]
pub type RForceRuAllocEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_rssi_target_value(&self) -> RForceTfRssiTargetValueR {
        RForceTfRssiTargetValueR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_rssi_target_en(&self) -> RForceTfRssiTargetEnR {
        RForceTfRssiTargetEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_macid_value(&self) -> RForceTfMacidValueR {
        RForceTfMacidValueR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_macid_en(&self) -> RForceTfMacidEnR {
        RForceTfMacidEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_ru_alloc_value(&self) -> RForceRuAllocValueR {
        RForceRuAllocValueR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_ru_alloc_en(&self) -> RForceRuAllocEnR {
        RForceRuAllocEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_rssi_target_value(
        &mut self,
    ) -> RForceTfRssiTargetValueW<'_, RegAxPwrUlForce0Spec> {
        RForceTfRssiTargetValueW::new(self, 0)
    }
    #[doc = "Bit 7 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_rssi_target_en(&mut self) -> RForceTfRssiTargetEnW<'_, RegAxPwrUlForce0Spec> {
        RForceTfRssiTargetEnW::new(self, 7)
    }
    #[doc = "Bits 8:14 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_tf_macid_value(&mut self) -> RForceTfMacidValueW<'_, RegAxPwrUlForce0Spec> {
        RForceTfMacidValueW::new(self, 8)
    }
    #[doc = "Bit 15 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_tf_macid_en(&mut self) -> RForceTfMacidEnW<'_, RegAxPwrUlForce0Spec> {
        RForceTfMacidEnW::new(self, 15)
    }
    #[doc = "Bits 16:23 - (RTL_only) Force input from MAC"]
    #[inline(always)]
    pub fn r_force_ru_alloc_value(&mut self) -> RForceRuAllocValueW<'_, RegAxPwrUlForce0Spec> {
        RForceRuAllocValueW::new(self, 16)
    }
    #[doc = "Bit 24 - (RTL_only) Force input from MAC en"]
    #[inline(always)]
    pub fn r_force_ru_alloc_en(&mut self) -> RForceRuAllocEnW<'_, RegAxPwrUlForce0Spec> {
        RForceRuAllocEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ax_pwr_ul_force0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ax_pwr_ul_force0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAxPwrUlForce0Spec;
impl crate::RegisterSpec for RegAxPwrUlForce0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ax_pwr_ul_force0::R`](R) reader structure"]
impl crate::Readable for RegAxPwrUlForce0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ax_pwr_ul_force0::W`](W) writer structure"]
impl crate::Writable for RegAxPwrUlForce0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_AX_PWR_UL_FORCE0 to value 0"]
impl crate::Resettable for RegAxPwrUlForce0Spec {}
