#[doc = "Register `REG_BF0_TIME_SETTING` reader"]
pub type R = crate::R<RegBf0TimeSettingSpec>;
#[doc = "Register `REG_BF0_TIME_SETTING` writer"]
pub type W = crate::W<RegBf0TimeSettingSpec>;
#[doc = "Field `BIT_BF0_LIFETIME` reader - beamforming 0 life time"]
pub type BitBf0LifetimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BF0_LIFETIME` writer - beamforming 0 life time"]
pub type BitBf0LifetimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_BF0_PRETIME_OVER` reader - beamforming 0 pre timeout time"]
pub type BitBf0PretimeOverR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BF0_PRETIME_OVER` writer - beamforming 0 pre timeout time"]
pub type BitBf0PretimeOverW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_EN_BF0_TIMER` reader - beamforming 0 timer enable"]
pub type BitEnBf0TimerR = crate::BitReader;
#[doc = "Field `BIT_EN_BF0_TIMER` writer - beamforming 0 timer enable"]
pub type BitEnBf0TimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BF0_UPDATE` reader - 1: when tx ndp ok/fail, enable to update beamforming 0 timer"]
pub type BitEnBf0UpdateR = crate::BitReader;
#[doc = "Field `BIT_EN_BF0_UPDATE` writer - 1: when tx ndp ok/fail, enable to update beamforming 0 timer"]
pub type BitEnBf0UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_bf0_timer_clr` reader - write 1 to clear beamforming 0 timer to 0"]
pub type BitBf0TimerClrR = crate::BitReader;
#[doc = "Field `BIT_bf0_timer_set` reader - write 1 to set beamforming 0 timer to BIT_bf0_lifetime"]
pub type BitBf0TimerSetR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - beamforming 0 life time"]
    #[inline(always)]
    pub fn bit_bf0_lifetime(&self) -> BitBf0LifetimeR {
        BitBf0LifetimeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - beamforming 0 pre timeout time"]
    #[inline(always)]
    pub fn bit_bf0_pretime_over(&self) -> BitBf0PretimeOverR {
        BitBf0PretimeOverR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - beamforming 0 timer enable"]
    #[inline(always)]
    pub fn bit_en_bf0_timer(&self) -> BitEnBf0TimerR {
        BitEnBf0TimerR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: when tx ndp ok/fail, enable to update beamforming 0 timer"]
    #[inline(always)]
    pub fn bit_en_bf0_update(&self) -> BitEnBf0UpdateR {
        BitEnBf0UpdateR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - write 1 to clear beamforming 0 timer to 0"]
    #[inline(always)]
    pub fn bit_bf0_timer_clr(&self) -> BitBf0TimerClrR {
        BitBf0TimerClrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to set beamforming 0 timer to BIT_bf0_lifetime"]
    #[inline(always)]
    pub fn bit_bf0_timer_set(&self) -> BitBf0TimerSetR {
        BitBf0TimerSetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - beamforming 0 life time"]
    #[inline(always)]
    pub fn bit_bf0_lifetime(&mut self) -> BitBf0LifetimeW<'_, RegBf0TimeSettingSpec> {
        BitBf0LifetimeW::new(self, 0)
    }
    #[doc = "Bits 16:27 - beamforming 0 pre timeout time"]
    #[inline(always)]
    pub fn bit_bf0_pretime_over(&mut self) -> BitBf0PretimeOverW<'_, RegBf0TimeSettingSpec> {
        BitBf0PretimeOverW::new(self, 16)
    }
    #[doc = "Bit 28 - beamforming 0 timer enable"]
    #[inline(always)]
    pub fn bit_en_bf0_timer(&mut self) -> BitEnBf0TimerW<'_, RegBf0TimeSettingSpec> {
        BitEnBf0TimerW::new(self, 28)
    }
    #[doc = "Bit 29 - 1: when tx ndp ok/fail, enable to update beamforming 0 timer"]
    #[inline(always)]
    pub fn bit_en_bf0_update(&mut self) -> BitEnBf0UpdateW<'_, RegBf0TimeSettingSpec> {
        BitEnBf0UpdateW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bf0_time_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bf0_time_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBf0TimeSettingSpec;
impl crate::RegisterSpec for RegBf0TimeSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bf0_time_setting::R`](R) reader structure"]
impl crate::Readable for RegBf0TimeSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bf0_time_setting::W`](W) writer structure"]
impl crate::Writable for RegBf0TimeSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BF0_TIME_SETTING to value 0x0fff_ffff"]
impl crate::Resettable for RegBf0TimeSettingSpec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
