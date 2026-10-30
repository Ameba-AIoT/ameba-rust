#[doc = "Register `REG_BF1_TIME_SETTING` reader"]
pub type R = crate::R<RegBf1TimeSettingSpec>;
#[doc = "Register `REG_BF1_TIME_SETTING` writer"]
pub type W = crate::W<RegBf1TimeSettingSpec>;
#[doc = "Field `BIT_BF1_LIFETIME` reader - beamforming 1 life time"]
pub type BitBf1LifetimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BF1_LIFETIME` writer - beamforming 1 life time"]
pub type BitBf1LifetimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_BF1_PRETIME_OVER` reader - beamforming 1 pre timeout time"]
pub type BitBf1PretimeOverR = crate::FieldReader<u16>;
#[doc = "Field `BIT_BF1_PRETIME_OVER` writer - beamforming 1 pre timeout time"]
pub type BitBf1PretimeOverW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_EN_BF1_TIMER` reader - beamforming 1 timer enable"]
pub type BitEnBf1TimerR = crate::BitReader;
#[doc = "Field `BIT_EN_BF1_TIMER` writer - beamforming 1 timer enable"]
pub type BitEnBf1TimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_BF1_UPDATE` reader - 1: when tx ndp ok/fail, enable to update beamforming 1 timer"]
pub type BitEnBf1UpdateR = crate::BitReader;
#[doc = "Field `BIT_EN_BF1_UPDATE` writer - 1: when tx ndp ok/fail, enable to update beamforming 1 timer"]
pub type BitEnBf1UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_bf1_timer_clr` reader - write 1 to clear beamforming 1 timer to 0"]
pub type BitBf1TimerClrR = crate::BitReader;
#[doc = "Field `BIT_bf1_timer_set` reader - write 1 to set beamforming 1 timer to BIT_bf1_lifetime"]
pub type BitBf1TimerSetR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - beamforming 1 life time"]
    #[inline(always)]
    pub fn bit_bf1_lifetime(&self) -> BitBf1LifetimeR {
        BitBf1LifetimeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - beamforming 1 pre timeout time"]
    #[inline(always)]
    pub fn bit_bf1_pretime_over(&self) -> BitBf1PretimeOverR {
        BitBf1PretimeOverR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - beamforming 1 timer enable"]
    #[inline(always)]
    pub fn bit_en_bf1_timer(&self) -> BitEnBf1TimerR {
        BitEnBf1TimerR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: when tx ndp ok/fail, enable to update beamforming 1 timer"]
    #[inline(always)]
    pub fn bit_en_bf1_update(&self) -> BitEnBf1UpdateR {
        BitEnBf1UpdateR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - write 1 to clear beamforming 1 timer to 0"]
    #[inline(always)]
    pub fn bit_bf1_timer_clr(&self) -> BitBf1TimerClrR {
        BitBf1TimerClrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to set beamforming 1 timer to BIT_bf1_lifetime"]
    #[inline(always)]
    pub fn bit_bf1_timer_set(&self) -> BitBf1TimerSetR {
        BitBf1TimerSetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - beamforming 1 life time"]
    #[inline(always)]
    pub fn bit_bf1_lifetime(&mut self) -> BitBf1LifetimeW<'_, RegBf1TimeSettingSpec> {
        BitBf1LifetimeW::new(self, 0)
    }
    #[doc = "Bits 16:27 - beamforming 1 pre timeout time"]
    #[inline(always)]
    pub fn bit_bf1_pretime_over(&mut self) -> BitBf1PretimeOverW<'_, RegBf1TimeSettingSpec> {
        BitBf1PretimeOverW::new(self, 16)
    }
    #[doc = "Bit 28 - beamforming 1 timer enable"]
    #[inline(always)]
    pub fn bit_en_bf1_timer(&mut self) -> BitEnBf1TimerW<'_, RegBf1TimeSettingSpec> {
        BitEnBf1TimerW::new(self, 28)
    }
    #[doc = "Bit 29 - 1: when tx ndp ok/fail, enable to update beamforming 1 timer"]
    #[inline(always)]
    pub fn bit_en_bf1_update(&mut self) -> BitEnBf1UpdateW<'_, RegBf1TimeSettingSpec> {
        BitEnBf1UpdateW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bf1_time_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bf1_time_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBf1TimeSettingSpec;
impl crate::RegisterSpec for RegBf1TimeSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bf1_time_setting::R`](R) reader structure"]
impl crate::Readable for RegBf1TimeSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bf1_time_setting::W`](W) writer structure"]
impl crate::Writable for RegBf1TimeSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BF1_TIME_SETTING to value 0x0fff_ffff"]
impl crate::Resettable for RegBf1TimeSettingSpec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
