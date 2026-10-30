#[doc = "Register `REG_CH0_EP_CTRL` reader"]
pub type R = crate::R<RegCh0EpCtrlSpec>;
#[doc = "Register `REG_CH0_EP_CTRL` writer"]
pub type W = crate::W<RegCh0EpCtrlSpec>;
#[doc = "Field `ch0_ep_auto_en` reader - "]
pub type Ch0EpAutoEnR = crate::BitReader;
#[doc = "Field `ch0_ep_auto_en` writer - "]
pub type Ch0EpAutoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_ep_binterval` reader - "]
pub type Ch0EpBintervalR = crate::FieldReader;
#[doc = "Field `ch0_ep_binterval` writer - "]
pub type Ch0EpBintervalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_ep_auto_en(&self) -> Ch0EpAutoEnR {
        Ch0EpAutoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ch0_ep_binterval(&self) -> Ch0EpBintervalR {
        Ch0EpBintervalR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_ep_auto_en(&mut self) -> Ch0EpAutoEnW<'_, RegCh0EpCtrlSpec> {
        Ch0EpAutoEnW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ch0_ep_binterval(&mut self) -> Ch0EpBintervalW<'_, RegCh0EpCtrlSpec> {
        Ch0EpBintervalW::new(self, 8)
    }
}
#[doc = "Channel_0 Endpoint Control\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_ep_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_ep_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0EpCtrlSpec;
impl crate::RegisterSpec for RegCh0EpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_ep_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCh0EpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_ep_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCh0EpCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_EP_CTRL to value 0"]
impl crate::Resettable for RegCh0EpCtrlSpec {}
