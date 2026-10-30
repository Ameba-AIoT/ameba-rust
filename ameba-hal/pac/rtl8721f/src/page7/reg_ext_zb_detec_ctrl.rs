#[doc = "Register `REG_EXT_ZB_DETEC_CTRL` reader"]
pub type R = crate::R<RegExtZbDetecCtrlSpec>;
#[doc = "Register `REG_EXT_ZB_DETEC_CTRL` writer"]
pub type W = crate::W<RegExtZbDetecCtrlSpec>;
#[doc = "Field `BIT_ZB_ARB_TIME` reader - 8 bit timer register : Arbitration window of ZB trx request (unit: us)"]
pub type BitZbArbTimeR = crate::FieldReader;
#[doc = "Field `BIT_ZB_ARB_TIME` writer - 8 bit timer register : Arbitration window of ZB trx request (unit: us)"]
pub type BitZbArbTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ZB_CNT_THR` reader - ZB count threshold, unit: us"]
pub type BitZbCntThrR = crate::FieldReader;
#[doc = "Field `BIT_ZB_CNT_THR` writer - ZB count threshold, unit: us"]
pub type BitZbCntThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ZB_PRI_DETECT_TIME` reader - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_REQUEST asserting, if ZG_PRIORITY is asserted within this ZB_PRI_DETECT_TIME duration, ZG will initiate high priority activities; otherwise, if ZB_PRIORITY is not asserted within this duration, it would be low priority activities. Units 1us. (0-255us)"]
pub type BitZbPriDetectTimeR = crate::FieldReader;
#[doc = "Field `BIT_ZB_PRI_DETECT_TIME` writer - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_REQUEST asserting, if ZG_PRIORITY is asserted within this ZB_PRI_DETECT_TIME duration, ZG will initiate high priority activities; otherwise, if ZB_PRIORITY is not asserted within this duration, it would be low priority activities. Units 1us. (0-255us)"]
pub type BitZbPriDetectTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ZB_TRX_DETECT_TIME` reader - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_PRI_DETECT_TIME duration, if ZB_PRIORITY is low or goes low during ZB_TRX_INIT_DETECT_TIME, Zigbee will receive packets; otherwise, if ZG_PRIORITY is high or goes high, ZB will TX data. Unit 1us."]
pub type BitZbTrxDetectTimeR = crate::FieldReader;
#[doc = "Field `BIT_ZB_TRX_DETECT_TIME` writer - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_PRI_DETECT_TIME duration, if ZB_PRIORITY is low or goes low during ZB_TRX_INIT_DETECT_TIME, Zigbee will receive packets; otherwise, if ZG_PRIORITY is high or goes high, ZB will TX data. Unit 1us."]
pub type BitZbTrxDetectTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 bit timer register : Arbitration window of ZB trx request (unit: us)"]
    #[inline(always)]
    pub fn bit_zb_arb_time(&self) -> BitZbArbTimeR {
        BitZbArbTimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ZB count threshold, unit: us"]
    #[inline(always)]
    pub fn bit_zb_cnt_thr(&self) -> BitZbCntThrR {
        BitZbCntThrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_REQUEST asserting, if ZG_PRIORITY is asserted within this ZB_PRI_DETECT_TIME duration, ZG will initiate high priority activities; otherwise, if ZB_PRIORITY is not asserted within this duration, it would be low priority activities. Units 1us. (0-255us)"]
    #[inline(always)]
    pub fn bit_zb_pri_detect_time(&self) -> BitZbPriDetectTimeR {
        BitZbPriDetectTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_PRI_DETECT_TIME duration, if ZB_PRIORITY is low or goes low during ZB_TRX_INIT_DETECT_TIME, Zigbee will receive packets; otherwise, if ZG_PRIORITY is high or goes high, ZB will TX data. Unit 1us."]
    #[inline(always)]
    pub fn bit_zb_trx_detect_time(&self) -> BitZbTrxDetectTimeR {
        BitZbTrxDetectTimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 bit timer register : Arbitration window of ZB trx request (unit: us)"]
    #[inline(always)]
    pub fn bit_zb_arb_time(&mut self) -> BitZbArbTimeW<'_, RegExtZbDetecCtrlSpec> {
        BitZbArbTimeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - ZB count threshold, unit: us"]
    #[inline(always)]
    pub fn bit_zb_cnt_thr(&mut self) -> BitZbCntThrW<'_, RegExtZbDetecCtrlSpec> {
        BitZbCntThrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_REQUEST asserting, if ZG_PRIORITY is asserted within this ZB_PRI_DETECT_TIME duration, ZG will initiate high priority activities; otherwise, if ZB_PRIORITY is not asserted within this duration, it would be low priority activities. Units 1us. (0-255us)"]
    #[inline(always)]
    pub fn bit_zb_pri_detect_time(&mut self) -> BitZbPriDetectTimeW<'_, RegExtZbDetecCtrlSpec> {
        BitZbPriDetectTimeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Valid if BIT_ZIGBEE_MODE = 1 & BIT_EXT_ZIGBEE_MODE = 0 (external mode, directional PRIORITY) After ZB_PRI_DETECT_TIME duration, if ZB_PRIORITY is low or goes low during ZB_TRX_INIT_DETECT_TIME, Zigbee will receive packets; otherwise, if ZG_PRIORITY is high or goes high, ZB will TX data. Unit 1us."]
    #[inline(always)]
    pub fn bit_zb_trx_detect_time(&mut self) -> BitZbTrxDetectTimeW<'_, RegExtZbDetecCtrlSpec> {
        BitZbTrxDetectTimeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ext_zb_detec_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ext_zb_detec_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegExtZbDetecCtrlSpec;
impl crate::RegisterSpec for RegExtZbDetecCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ext_zb_detec_ctrl::R`](R) reader structure"]
impl crate::Readable for RegExtZbDetecCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ext_zb_detec_ctrl::W`](W) writer structure"]
impl crate::Writable for RegExtZbDetecCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_EXT_ZB_DETEC_CTRL to value 0x40c8_0000"]
impl crate::Resettable for RegExtZbDetecCtrlSpec {
    const RESET_VALUE: u32 = 0x40c8_0000;
}
