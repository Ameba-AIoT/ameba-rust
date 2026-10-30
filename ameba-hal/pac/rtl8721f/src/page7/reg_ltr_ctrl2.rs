#[doc = "Register `REG_LTR_CTRL2` reader"]
pub type R = crate::R<RegLtrCtrl2Spec>;
#[doc = "Register `REG_LTR_CTRL2` writer"]
pub type W = crate::W<RegLtrCtrl2Spec>;
#[doc = "Field `BIT_RX_EXHAUST_ID` reader - When rx page less than this threshold, release obff block rx."]
pub type BitRxExhaustIdR = crate::FieldReader;
#[doc = "Field `BIT_RX_EXHAUST_ID` writer - When rx page less than this threshold, release obff block rx."]
pub type BitRxExhaustIdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_LOW_RATE_TRIGS` reader - Low rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
pub type BitLowRateTrigsR = crate::FieldReader;
#[doc = "Field `BIT_LOW_RATE_TRIGS` writer - Low rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
pub type BitLowRateTrigsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MEDIA_RATE_TRIGS` reader - Media rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
pub type BitMediaRateTrigsR = crate::FieldReader;
#[doc = "Field `BIT_MEDIA_RATE_TRIGS` writer - Media rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
pub type BitMediaRateTrigsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_HIGH_RATE_TRIGS` reader - High rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
pub type BitHighRateTrigsR = crate::FieldReader;
#[doc = "Field `BIT_HIGH_RATE_TRIGS` writer - High rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
pub type BitHighRateTrigsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_LOW_RATE_BDRY` reader - High, media, low rate boundary"]
pub type BitLowRateBdryR = crate::FieldReader;
#[doc = "Field `BIT_LOW_RATE_BDRY` writer - High, media, low rate boundary"]
pub type BitLowRateBdryW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_HIGH_RATE_BDRY` reader - High, media, low rate boundary"]
pub type BitHighRateBdryR = crate::FieldReader;
#[doc = "Field `BIT_HIGH_RATE_BDRY` writer - High, media, low rate boundary"]
pub type BitHighRateBdryW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_OBFF_TIMER_ID_HIGH` reader - When media rate rx packet in , delay rx dma timeout value index. 0: no delay,1: 12.8us; 2: 102.4us; 3: 204.8us; 4: 819.2 us; 5: 1.638ms; 6: 52ms; 7: always block until obff or cpu active window"]
pub type BitObffTimerIdHighR = crate::FieldReader;
#[doc = "Field `BIT_OBFF_TIMER_ID_HIGH` writer - When media rate rx packet in , delay rx dma timeout value index. 0: no delay,1: 12.8us; 2: 102.4us; 3: 204.8us; 4: 819.2 us; 5: 1.638ms; 6: 52ms; 7: always block until obff or cpu active window"]
pub type BitObffTimerIdHighW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - When rx page less than this threshold, release obff block rx."]
    #[inline(always)]
    pub fn bit_rx_exhaust_id(&self) -> BitRxExhaustIdR {
        BitRxExhaustIdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Low rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
    #[inline(always)]
    pub fn bit_low_rate_trigs(&self) -> BitLowRateTrigsR {
        BitLowRateTrigsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Media rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
    #[inline(always)]
    pub fn bit_media_rate_trigs(&self) -> BitMediaRateTrigsR {
        BitMediaRateTrigsR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - High rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
    #[inline(always)]
    pub fn bit_high_rate_trigs(&self) -> BitHighRateTrigsR {
        BitHighRateTrigsR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:15 - High, media, low rate boundary"]
    #[inline(always)]
    pub fn bit_low_rate_bdry(&self) -> BitLowRateBdryR {
        BitLowRateBdryR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - High, media, low rate boundary"]
    #[inline(always)]
    pub fn bit_high_rate_bdry(&self) -> BitHighRateBdryR {
        BitHighRateBdryR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 29:31 - When media rate rx packet in , delay rx dma timeout value index. 0: no delay,1: 12.8us; 2: 102.4us; 3: 204.8us; 4: 819.2 us; 5: 1.638ms; 6: 52ms; 7: always block until obff or cpu active window"]
    #[inline(always)]
    pub fn bit_obff_timer_id_high(&self) -> BitObffTimerIdHighR {
        BitObffTimerIdHighR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - When rx page less than this threshold, release obff block rx."]
    #[inline(always)]
    pub fn bit_rx_exhaust_id(&mut self) -> BitRxExhaustIdW<'_, RegLtrCtrl2Spec> {
        BitRxExhaustIdW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Low rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
    #[inline(always)]
    pub fn bit_low_rate_trigs(&mut self) -> BitLowRateTrigsW<'_, RegLtrCtrl2Spec> {
        BitLowRateTrigsW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Media rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
    #[inline(always)]
    pub fn bit_media_rate_trigs(&mut self) -> BitMediaRateTrigsW<'_, RegLtrCtrl2Spec> {
        BitMediaRateTrigsW::new(self, 5)
    }
    #[doc = "Bits 7:8 - High rate rx ltr trigger point select: 0 plcp ready;1: rx a1 latch OK; 2: fcs ready."]
    #[inline(always)]
    pub fn bit_high_rate_trigs(&mut self) -> BitHighRateTrigsW<'_, RegLtrCtrl2Spec> {
        BitHighRateTrigsW::new(self, 7)
    }
    #[doc = "Bits 9:15 - High, media, low rate boundary"]
    #[inline(always)]
    pub fn bit_low_rate_bdry(&mut self) -> BitLowRateBdryW<'_, RegLtrCtrl2Spec> {
        BitLowRateBdryW::new(self, 9)
    }
    #[doc = "Bits 16:22 - High, media, low rate boundary"]
    #[inline(always)]
    pub fn bit_high_rate_bdry(&mut self) -> BitHighRateBdryW<'_, RegLtrCtrl2Spec> {
        BitHighRateBdryW::new(self, 16)
    }
    #[doc = "Bits 29:31 - When media rate rx packet in , delay rx dma timeout value index. 0: no delay,1: 12.8us; 2: 102.4us; 3: 204.8us; 4: 819.2 us; 5: 1.638ms; 6: 52ms; 7: always block until obff or cpu active window"]
    #[inline(always)]
    pub fn bit_obff_timer_id_high(&mut self) -> BitObffTimerIdHighW<'_, RegLtrCtrl2Spec> {
        BitObffTimerIdHighW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ltr_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ltr_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLtrCtrl2Spec;
impl crate::RegisterSpec for RegLtrCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ltr_ctrl2::R`](R) reader structure"]
impl crate::Readable for RegLtrCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ltr_ctrl2::W`](W) writer structure"]
impl crate::Writable for RegLtrCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LTR_CTRL2 to value 0x8060_88e0"]
impl crate::Resettable for RegLtrCtrl2Spec {
    const RESET_VALUE: u32 = 0x8060_88e0;
}
