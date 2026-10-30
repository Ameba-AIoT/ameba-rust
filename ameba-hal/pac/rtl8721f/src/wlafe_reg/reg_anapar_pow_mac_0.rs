#[doc = "Register `REG_ANAPAR_POW_MAC_0` reader"]
pub type R = crate::R<RegAnaparPowMac0Spec>;
#[doc = "Register `REG_ANAPAR_POW_MAC_0` writer"]
pub type W = crate::W<RegAnaparPowMac0Spec>;
#[doc = "Field `pow_ldo` reader - Power on AFE power cut and pll ldo"]
pub type PowLdoR = crate::BitReader;
#[doc = "Field `pow_ldo` writer - Power on AFE power cut and pll ldo"]
pub type PowLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pow_pll` reader - Power on pll"]
pub type PowPllR = crate::BitReader;
#[doc = "Field `pow_pll` writer - Power on pll"]
pub type PowPllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pow_ldo_dac` reader - Power on ldo for dac"]
pub type PowLdoDacR = crate::BitReader;
#[doc = "Field `pow_ldo_dac` writer - Power on ldo for dac"]
pub type PowLdoDacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pow_ldo_adc` reader - Power on ldo for adc"]
pub type PowLdoAdcR = crate::BitReader;
#[doc = "Field `pow_ldo_adc` writer - Power on ldo for adc"]
pub type PowLdoAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pow_ldo_rssi_adc` reader - Power on ldo for rssi adc"]
pub type PowLdoRssiAdcR = crate::BitReader;
#[doc = "Field `pow_ldo_rssi_adc` writer - Power on ldo for rssi adc"]
pub type PowLdoRssiAdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hw_en_rfafe` reader - core power reset of wl rfafe"]
pub type HwEnRfafeR = crate::BitReader;
#[doc = "Field `hw_en_rfafe` writer - core power reset of wl rfafe"]
pub type HwEnRfafeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hw_en_rfafe_2regu` reader - control 3.3V power reset of wl rfafe, need level shift to 3.3V by regu"]
pub type HwEnRfafe2reguR = crate::BitReader;
#[doc = "Field `hw_en_rfafe_2regu` writer - control 3.3V power reset of wl rfafe, need level shift to 3.3V by regu"]
pub type HwEnRfafe2reguW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_WLAFE` reader - isolation signal"]
pub type IsoWlafeR = crate::BitReader;
#[doc = "Field `ISO_WLAFE` writer - isolation signal"]
pub type IsoWlafeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlafe_iso_sel` reader - Option to enable isolation of wlafe when power off WLAFE 1: enable ,0x00\\[7\\]will be written 1 when wlafe power off 0: disable ,0x00\\[7\\]will keep 0 when wlafe power off"]
pub type WlafeIsoSelR = crate::BitReader;
#[doc = "Field `wlafe_iso_sel` writer - Option to enable isolation of wlafe when power off WLAFE 1: enable ,0x00\\[7\\]will be written 1 when wlafe power off 0: disable ,0x00\\[7\\]will keep 0 when wlafe power off"]
pub type WlafeIsoSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlafe_pof_sel` reader - Option to power off WLAFE 1: enable ,power off wlafe when bt&wl request off 0: disable ,keep wlafe power on when bt&wl request off"]
pub type WlafePofSelR = crate::BitReader;
#[doc = "Field `wlafe_pof_sel` writer - Option to power off WLAFE 1: enable ,power off wlafe when bt&wl request off 0: disable ,keep wlafe power on when bt&wl request off"]
pub type WlafePofSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power on AFE power cut and pll ldo"]
    #[inline(always)]
    pub fn pow_ldo(&self) -> PowLdoR {
        PowLdoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power on pll"]
    #[inline(always)]
    pub fn pow_pll(&self) -> PowPllR {
        PowPllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power on ldo for dac"]
    #[inline(always)]
    pub fn pow_ldo_dac(&self) -> PowLdoDacR {
        PowLdoDacR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power on ldo for adc"]
    #[inline(always)]
    pub fn pow_ldo_adc(&self) -> PowLdoAdcR {
        PowLdoAdcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power on ldo for rssi adc"]
    #[inline(always)]
    pub fn pow_ldo_rssi_adc(&self) -> PowLdoRssiAdcR {
        PowLdoRssiAdcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - core power reset of wl rfafe"]
    #[inline(always)]
    pub fn hw_en_rfafe(&self) -> HwEnRfafeR {
        HwEnRfafeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - control 3.3V power reset of wl rfafe, need level shift to 3.3V by regu"]
    #[inline(always)]
    pub fn hw_en_rfafe_2regu(&self) -> HwEnRfafe2reguR {
        HwEnRfafe2reguR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - isolation signal"]
    #[inline(always)]
    pub fn iso_wlafe(&self) -> IsoWlafeR {
        IsoWlafeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Option to enable isolation of wlafe when power off WLAFE 1: enable ,0x00\\[7\\]will be written 1 when wlafe power off 0: disable ,0x00\\[7\\]will keep 0 when wlafe power off"]
    #[inline(always)]
    pub fn wlafe_iso_sel(&self) -> WlafeIsoSelR {
        WlafeIsoSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Option to power off WLAFE 1: enable ,power off wlafe when bt&wl request off 0: disable ,keep wlafe power on when bt&wl request off"]
    #[inline(always)]
    pub fn wlafe_pof_sel(&self) -> WlafePofSelR {
        WlafePofSelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on AFE power cut and pll ldo"]
    #[inline(always)]
    pub fn pow_ldo(&mut self) -> PowLdoW<'_, RegAnaparPowMac0Spec> {
        PowLdoW::new(self, 0)
    }
    #[doc = "Bit 1 - Power on pll"]
    #[inline(always)]
    pub fn pow_pll(&mut self) -> PowPllW<'_, RegAnaparPowMac0Spec> {
        PowPllW::new(self, 1)
    }
    #[doc = "Bit 2 - Power on ldo for dac"]
    #[inline(always)]
    pub fn pow_ldo_dac(&mut self) -> PowLdoDacW<'_, RegAnaparPowMac0Spec> {
        PowLdoDacW::new(self, 2)
    }
    #[doc = "Bit 3 - Power on ldo for adc"]
    #[inline(always)]
    pub fn pow_ldo_adc(&mut self) -> PowLdoAdcW<'_, RegAnaparPowMac0Spec> {
        PowLdoAdcW::new(self, 3)
    }
    #[doc = "Bit 4 - Power on ldo for rssi adc"]
    #[inline(always)]
    pub fn pow_ldo_rssi_adc(&mut self) -> PowLdoRssiAdcW<'_, RegAnaparPowMac0Spec> {
        PowLdoRssiAdcW::new(self, 4)
    }
    #[doc = "Bit 5 - core power reset of wl rfafe"]
    #[inline(always)]
    pub fn hw_en_rfafe(&mut self) -> HwEnRfafeW<'_, RegAnaparPowMac0Spec> {
        HwEnRfafeW::new(self, 5)
    }
    #[doc = "Bit 6 - control 3.3V power reset of wl rfafe, need level shift to 3.3V by regu"]
    #[inline(always)]
    pub fn hw_en_rfafe_2regu(&mut self) -> HwEnRfafe2reguW<'_, RegAnaparPowMac0Spec> {
        HwEnRfafe2reguW::new(self, 6)
    }
    #[doc = "Bit 7 - isolation signal"]
    #[inline(always)]
    pub fn iso_wlafe(&mut self) -> IsoWlafeW<'_, RegAnaparPowMac0Spec> {
        IsoWlafeW::new(self, 7)
    }
    #[doc = "Bit 10 - Option to enable isolation of wlafe when power off WLAFE 1: enable ,0x00\\[7\\]will be written 1 when wlafe power off 0: disable ,0x00\\[7\\]will keep 0 when wlafe power off"]
    #[inline(always)]
    pub fn wlafe_iso_sel(&mut self) -> WlafeIsoSelW<'_, RegAnaparPowMac0Spec> {
        WlafeIsoSelW::new(self, 10)
    }
    #[doc = "Bit 11 - Option to power off WLAFE 1: enable ,power off wlafe when bt&wl request off 0: disable ,keep wlafe power on when bt&wl request off"]
    #[inline(always)]
    pub fn wlafe_pof_sel(&mut self) -> WlafePofSelW<'_, RegAnaparPowMac0Spec> {
        WlafePofSelW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_pow_mac_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_pow_mac_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparPowMac0Spec;
impl crate::RegisterSpec for RegAnaparPowMac0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_pow_mac_0::R`](R) reader structure"]
impl crate::Readable for RegAnaparPowMac0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_pow_mac_0::W`](W) writer structure"]
impl crate::Writable for RegAnaparPowMac0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_POW_MAC_0 to value 0x0c80"]
impl crate::Resettable for RegAnaparPowMac0Spec {
    const RESET_VALUE: u32 = 0x0c80;
}
