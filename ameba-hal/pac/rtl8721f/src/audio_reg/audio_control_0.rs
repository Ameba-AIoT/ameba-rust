#[doc = "Register `audio_control_0` reader"]
pub type R = crate::R<AudioControl0Spec>;
#[doc = "Register `audio_control_0` writer"]
pub type W = crate::W<AudioControl0Spec>;
#[doc = "Field `audio_ip_en` reader - "]
pub type AudioIpEnR = crate::BitReader;
#[doc = "Field `audio_ip_en` writer - "]
pub type AudioIpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `audio_control_0_dummy` reader - DUMMY"]
pub type AudioControl0DummyR = crate::BitReader;
#[doc = "Field `audio_control_0_dummy` writer - DUMMY"]
pub type AudioControl0DummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sys_clk_rate_sel` reader - Audio sys_clk selection"]
pub type SysClkRateSelR = crate::FieldReader;
#[doc = "Field `sys_clk_rate_sel` writer - Audio sys_clk selection"]
pub type SysClkRateSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `audio_dbg_sel` reader - Debug probe selection"]
pub type AudioDbgSelR = crate::FieldReader;
#[doc = "Field `audio_dbg_sel` writer - Debug probe selection"]
pub type AudioDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn audio_ip_en(&self) -> AudioIpEnR {
        AudioIpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - DUMMY"]
    #[inline(always)]
    pub fn audio_control_0_dummy(&self) -> AudioControl0DummyR {
        AudioControl0DummyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Audio sys_clk selection"]
    #[inline(always)]
    pub fn sys_clk_rate_sel(&self) -> SysClkRateSelR {
        SysClkRateSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 13:17 - Debug probe selection"]
    #[inline(always)]
    pub fn audio_dbg_sel(&self) -> AudioDbgSelR {
        AudioDbgSelR::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn audio_ip_en(&mut self) -> AudioIpEnW<'_, AudioControl0Spec> {
        AudioIpEnW::new(self, 0)
    }
    #[doc = "Bit 3 - DUMMY"]
    #[inline(always)]
    pub fn audio_control_0_dummy(&mut self) -> AudioControl0DummyW<'_, AudioControl0Spec> {
        AudioControl0DummyW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Audio sys_clk selection"]
    #[inline(always)]
    pub fn sys_clk_rate_sel(&mut self) -> SysClkRateSelW<'_, AudioControl0Spec> {
        SysClkRateSelW::new(self, 4)
    }
    #[doc = "Bits 13:17 - Debug probe selection"]
    #[inline(always)]
    pub fn audio_dbg_sel(&mut self) -> AudioDbgSelW<'_, AudioControl0Spec> {
        AudioDbgSelW::new(self, 13)
    }
}
#[doc = "Audio Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_control_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_control_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioControl0Spec;
impl crate::RegisterSpec for AudioControl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_control_0::R`](R) reader structure"]
impl crate::Readable for AudioControl0Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_control_0::W`](W) writer structure"]
impl crate::Writable for AudioControl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets audio_control_0 to value 0x20"]
impl crate::Resettable for AudioControl0Spec {
    const RESET_VALUE: u32 = 0x20;
}
