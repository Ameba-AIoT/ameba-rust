#[doc = "Register `REG_NAN_SETTING_2` reader"]
pub type R = crate::R<RegNanSetting2Spec>;
#[doc = "Register `REG_NAN_SETTING_2` writer"]
pub type W = crate::W<RegNanSetting2Spec>;
#[doc = "Field `BIT_SYNC_BCN_DELAY` reader - r_HC * r_SLOTTIME * 8'd40 (unit:us) default: r_HC = 1, r_SLOTTIME = 9, => 360us max is 16ms (DW)"]
pub type BitSyncBcnDelayR = crate::FieldReader<u16>;
#[doc = "Field `BIT_SYNC_BCN_DELAY` writer - r_HC * r_SLOTTIME * 8'd40 (unit:us) default: r_HC = 1, r_SLOTTIME = 9, => 360us max is 16ms (DW)"]
pub type BitSyncBcnDelayW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u32>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:13 - r_HC * r_SLOTTIME * 8'd40 (unit:us) default: r_HC = 1, r_SLOTTIME = 9, => 360us max is 16ms (DW)"]
    #[inline(always)]
    pub fn bit_sync_bcn_delay(&self) -> BitSyncBcnDelayR {
        BitSyncBcnDelayR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:13 - r_HC * r_SLOTTIME * 8'd40 (unit:us) default: r_HC = 1, r_SLOTTIME = 9, => 360us max is 16ms (DW)"]
    #[inline(always)]
    pub fn bit_sync_bcn_delay(&mut self) -> BitSyncBcnDelayW<'_, RegNanSetting2Spec> {
        BitSyncBcnDelayW::new(self, 0)
    }
    #[doc = "Bits 14:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegNanSetting2Spec> {
        DummyW::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_nan_setting_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_nan_setting_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNanSetting2Spec;
impl crate::RegisterSpec for RegNanSetting2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_nan_setting_2::R`](R) reader structure"]
impl crate::Readable for RegNanSetting2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_nan_setting_2::W`](W) writer structure"]
impl crate::Writable for RegNanSetting2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_NAN_SETTING_2 to value 0x0168"]
impl crate::Resettable for RegNanSetting2Spec {
    const RESET_VALUE: u32 = 0x0168;
}
