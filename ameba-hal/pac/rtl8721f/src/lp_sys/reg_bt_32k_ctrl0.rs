#[doc = "Register `REG_BT_32K_CTRL0` reader"]
pub type R = crate::R<RegBt32kCtrl0Spec>;
#[doc = "Register `REG_BT_32K_CTRL0` writer"]
pub type W = crate::W<RegBt32kCtrl0Spec>;
#[doc = "Field `dwtim_srstn_chn` reader - sw reset for bt dw channel ,active 0 default all channel will release after dw timer function enable"]
pub type DwtimSrstnChnR = crate::FieldReader;
#[doc = "Field `dwtim_srstn_chn` writer - sw reset for bt dw channel ,active 0 default all channel will release after dw timer function enable"]
pub type DwtimSrstnChnW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BT_mldo_mode_nm` reader - mem ldo mode normal option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
pub type BtMldoModeNmR = crate::FieldReader;
#[doc = "Field `BT_mldo_mode_nm` writer - mem ldo mode normal option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
pub type BtMldoModeNmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BT_mldo_mode_lps` reader - mem ldo mode slep option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
pub type BtMldoModeLpsR = crate::FieldReader;
#[doc = "Field `BT_mldo_mode_lps` writer - mem ldo mode slep option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
pub type BtMldoModeLpsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BT_mldo_req_trig` reader - level -> pulse. Write to 1 to update BT analog 1.8V LDO request when BT is active (load BT_mldo_mode_nm). need to write 0 before next request"]
pub type BtMldoReqTrigR = crate::BitReader;
#[doc = "Field `BT_mldo_req_trig` writer - level -> pulse. Write to 1 to update BT analog 1.8V LDO request when BT is active (load BT_mldo_mode_nm). need to write 0 before next request"]
pub type BtMldoReqTrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - sw reset for bt dw channel ,active 0 default all channel will release after dw timer function enable"]
    #[inline(always)]
    pub fn dwtim_srstn_chn(&self) -> DwtimSrstnChnR {
        DwtimSrstnChnR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - mem ldo mode normal option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
    #[inline(always)]
    pub fn bt_mldo_mode_nm(&self) -> BtMldoModeNmR {
        BtMldoModeNmR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - mem ldo mode slep option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
    #[inline(always)]
    pub fn bt_mldo_mode_lps(&self) -> BtMldoModeLpsR {
        BtMldoModeLpsR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - level -> pulse. Write to 1 to update BT analog 1.8V LDO request when BT is active (load BT_mldo_mode_nm). need to write 0 before next request"]
    #[inline(always)]
    pub fn bt_mldo_req_trig(&self) -> BtMldoReqTrigR {
        BtMldoReqTrigR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - sw reset for bt dw channel ,active 0 default all channel will release after dw timer function enable"]
    #[inline(always)]
    pub fn dwtim_srstn_chn(&mut self) -> DwtimSrstnChnW<'_, RegBt32kCtrl0Spec> {
        DwtimSrstnChnW::new(self, 0)
    }
    #[doc = "Bits 24:25 - mem ldo mode normal option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
    #[inline(always)]
    pub fn bt_mldo_mode_nm(&mut self) -> BtMldoModeNmW<'_, RegBt32kCtrl0Spec> {
        BtMldoModeNmW::new(self, 24)
    }
    #[doc = "Bits 26:27 - mem ldo mode slep option 00:off 01:sleep 10:normal + dummy 11:normal + no dummy"]
    #[inline(always)]
    pub fn bt_mldo_mode_lps(&mut self) -> BtMldoModeLpsW<'_, RegBt32kCtrl0Spec> {
        BtMldoModeLpsW::new(self, 26)
    }
    #[doc = "Bit 31 - level -> pulse. Write to 1 to update BT analog 1.8V LDO request when BT is active (load BT_mldo_mode_nm). need to write 0 before next request"]
    #[inline(always)]
    pub fn bt_mldo_req_trig(&mut self) -> BtMldoReqTrigW<'_, RegBt32kCtrl0Spec> {
        BtMldoReqTrigW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_32k_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_32k_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBt32kCtrl0Spec;
impl crate::RegisterSpec for RegBt32kCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_32k_ctrl0::R`](R) reader structure"]
impl crate::Readable for RegBt32kCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_32k_ctrl0::W`](W) writer structure"]
impl crate::Writable for RegBt32kCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_32K_CTRL0 to value 0x7f"]
impl crate::Resettable for RegBt32kCtrl0Spec {
    const RESET_VALUE: u32 = 0x7f;
}
