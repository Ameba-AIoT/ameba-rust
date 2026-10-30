#[doc = "Register `REG_CH1_EP_CTRL` reader"]
pub type R = crate::R<RegCh1EpCtrlSpec>;
#[doc = "Register `REG_CH1_EP_CTRL` writer"]
pub type W = crate::W<RegCh1EpCtrlSpec>;
#[doc = "Field `ch1_ep_auto_en` reader - "]
pub type Ch1EpAutoEnR = crate::BitReader;
#[doc = "Field `ch1_ep_auto_en` writer - "]
pub type Ch1EpAutoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_ep_binterval` reader - "]
pub type Ch1EpBintervalR = crate::FieldReader;
#[doc = "Field `ch1_ep_binterval` writer - "]
pub type Ch1EpBintervalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch1_ep_auto_en(&self) -> Ch1EpAutoEnR {
        Ch1EpAutoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ch1_ep_binterval(&self) -> Ch1EpBintervalR {
        Ch1EpBintervalR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch1_ep_auto_en(&mut self) -> Ch1EpAutoEnW<'_, RegCh1EpCtrlSpec> {
        Ch1EpAutoEnW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ch1_ep_binterval(&mut self) -> Ch1EpBintervalW<'_, RegCh1EpCtrlSpec> {
        Ch1EpBintervalW::new(self, 8)
    }
}
#[doc = "Channel_1 Endpoint Control\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_ep_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch1_ep_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1EpCtrlSpec;
impl crate::RegisterSpec for RegCh1EpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_ep_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCh1EpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch1_ep_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCh1EpCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH1_EP_CTRL to value 0"]
impl crate::Resettable for RegCh1EpCtrlSpec {}
