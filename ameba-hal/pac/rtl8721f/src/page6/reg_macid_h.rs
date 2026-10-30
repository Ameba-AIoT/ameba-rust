#[doc = "Register `REG_MACID_H` reader"]
pub type R = crate::R<RegMacidHSpec>;
#[doc = "Register `REG_MACID_H` writer"]
pub type W = crate::W<RegMacidHSpec>;
#[doc = "Field `BIT_MACID_H` reader - MAC Identification Number Register BIT\\[47:32\\]"]
pub type BitMacidHR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MACID_H` writer - MAC Identification Number Register BIT\\[47:32\\]"]
pub type BitMacidHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_EN_PORT0` reader - When set 1,enable PORT0"]
pub type BitEnPort0R = crate::BitReader;
#[doc = "Field `BIT_EN_PORT0` writer - When set 1,enable PORT0"]
pub type BitEnPort0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_PORT1` reader - When set 1,enable PORT1"]
pub type BitEnPort1R = crate::BitReader;
#[doc = "Field `BIT_EN_PORT1` writer - When set 1,enable PORT1"]
pub type BitEnPort1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHKBSSID_MGNT_P1` reader - When set,enable check port1 mgnt frame bssid"]
pub type BitChkbssidMgntP1R = crate::BitReader;
#[doc = "Field `BIT_CHKBSSID_MGNT_P1` writer - When set,enable check port1 mgnt frame bssid"]
pub type BitChkbssidMgntP1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHKBSSID_CTL_P1` reader - When set,enable check port1 bssid of trigger/vhtndpa/hendpa/ba"]
pub type BitChkbssidCtlP1R = crate::BitReader;
#[doc = "Field `BIT_CHKBSSID_CTL_P1` writer - When set,enable check port1 bssid of trigger/vhtndpa/hendpa/ba"]
pub type BitChkbssidCtlP1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CHKBSSID_DATA_P1` reader - When set,enable check port1 data frame bssid"]
pub type BitChkbssidDataP1R = crate::BitReader;
#[doc = "Field `BIT_CHKBSSID_DATA_P1` writer - When set,enable check port1 data frame bssid"]
pub type BitChkbssidDataP1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_AP_BSSID_FIT_UC` reader - When set 1,bssid fit when AP port0 rx unciast data packet"]
pub type RApBssidFitUcR = crate::BitReader;
#[doc = "Field `r_AP_BSSID_FIT_UC` writer - When set 1,bssid fit when AP port0 rx unciast data packet"]
pub type RApBssidFitUcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EN_PORT2` reader - When set 1,enable NAN PORT2"]
pub type BitEnPort2R = crate::BitReader;
#[doc = "Field `BIT_EN_PORT2` writer - When set 1,enable NAN PORT2"]
pub type BitEnPort2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_chkbssid_nan_p2` reader - When set 1,enable check port2 bssid."]
pub type RChkbssidNanP2R = crate::BitReader;
#[doc = "Field `r_chkbssid_nan_p2` writer - When set 1,enable check port2 bssid."]
pub type RChkbssidNanP2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_macrx_reset` writer - set 1 to reset macrx.One pulse"]
pub type RMacrxResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Identification Number Register BIT\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_macid_h(&self) -> BitMacidHR {
        BitMacidHR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - When set 1,enable PORT0"]
    #[inline(always)]
    pub fn bit_en_port0(&self) -> BitEnPort0R {
        BitEnPort0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When set 1,enable PORT1"]
    #[inline(always)]
    pub fn bit_en_port1(&self) -> BitEnPort1R {
        BitEnPort1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When set,enable check port1 mgnt frame bssid"]
    #[inline(always)]
    pub fn bit_chkbssid_mgnt_p1(&self) -> BitChkbssidMgntP1R {
        BitChkbssidMgntP1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When set,enable check port1 bssid of trigger/vhtndpa/hendpa/ba"]
    #[inline(always)]
    pub fn bit_chkbssid_ctl_p1(&self) -> BitChkbssidCtlP1R {
        BitChkbssidCtlP1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When set,enable check port1 data frame bssid"]
    #[inline(always)]
    pub fn bit_chkbssid_data_p1(&self) -> BitChkbssidDataP1R {
        BitChkbssidDataP1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When set 1,bssid fit when AP port0 rx unciast data packet"]
    #[inline(always)]
    pub fn r_ap_bssid_fit_uc(&self) -> RApBssidFitUcR {
        RApBssidFitUcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When set 1,enable NAN PORT2"]
    #[inline(always)]
    pub fn bit_en_port2(&self) -> BitEnPort2R {
        BitEnPort2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When set 1,enable check port2 bssid."]
    #[inline(always)]
    pub fn r_chkbssid_nan_p2(&self) -> RChkbssidNanP2R {
        RChkbssidNanP2R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Identification Number Register BIT\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_macid_h(&mut self) -> BitMacidHW<'_, RegMacidHSpec> {
        BitMacidHW::new(self, 0)
    }
    #[doc = "Bit 16 - When set 1,enable PORT0"]
    #[inline(always)]
    pub fn bit_en_port0(&mut self) -> BitEnPort0W<'_, RegMacidHSpec> {
        BitEnPort0W::new(self, 16)
    }
    #[doc = "Bit 17 - When set 1,enable PORT1"]
    #[inline(always)]
    pub fn bit_en_port1(&mut self) -> BitEnPort1W<'_, RegMacidHSpec> {
        BitEnPort1W::new(self, 17)
    }
    #[doc = "Bit 18 - When set,enable check port1 mgnt frame bssid"]
    #[inline(always)]
    pub fn bit_chkbssid_mgnt_p1(&mut self) -> BitChkbssidMgntP1W<'_, RegMacidHSpec> {
        BitChkbssidMgntP1W::new(self, 18)
    }
    #[doc = "Bit 19 - When set,enable check port1 bssid of trigger/vhtndpa/hendpa/ba"]
    #[inline(always)]
    pub fn bit_chkbssid_ctl_p1(&mut self) -> BitChkbssidCtlP1W<'_, RegMacidHSpec> {
        BitChkbssidCtlP1W::new(self, 19)
    }
    #[doc = "Bit 20 - When set,enable check port1 data frame bssid"]
    #[inline(always)]
    pub fn bit_chkbssid_data_p1(&mut self) -> BitChkbssidDataP1W<'_, RegMacidHSpec> {
        BitChkbssidDataP1W::new(self, 20)
    }
    #[doc = "Bit 21 - When set 1,bssid fit when AP port0 rx unciast data packet"]
    #[inline(always)]
    pub fn r_ap_bssid_fit_uc(&mut self) -> RApBssidFitUcW<'_, RegMacidHSpec> {
        RApBssidFitUcW::new(self, 21)
    }
    #[doc = "Bit 22 - When set 1,enable NAN PORT2"]
    #[inline(always)]
    pub fn bit_en_port2(&mut self) -> BitEnPort2W<'_, RegMacidHSpec> {
        BitEnPort2W::new(self, 22)
    }
    #[doc = "Bit 23 - When set 1,enable check port2 bssid."]
    #[inline(always)]
    pub fn r_chkbssid_nan_p2(&mut self) -> RChkbssidNanP2W<'_, RegMacidHSpec> {
        RChkbssidNanP2W::new(self, 23)
    }
    #[doc = "Bit 25 - set 1 to reset macrx.One pulse"]
    #[inline(always)]
    pub fn r_macrx_reset(&mut self) -> RMacrxResetW<'_, RegMacidHSpec> {
        RMacrxResetW::new(self, 25)
    }
}
#[doc = "MAC ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidHSpec;
impl crate::RegisterSpec for RegMacidHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_h::R`](R) reader structure"]
impl crate::Readable for RegMacidHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_h::W`](W) writer structure"]
impl crate::Writable for RegMacidHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_H to value 0x0023_0000"]
impl crate::Resettable for RegMacidHSpec {
    const RESET_VALUE: u32 = 0x0023_0000;
}
