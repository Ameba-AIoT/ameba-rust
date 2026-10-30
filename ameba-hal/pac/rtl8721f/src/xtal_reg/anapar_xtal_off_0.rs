#[doc = "Register `ANAPAR_XTAL_OFF_0` reader"]
pub type R = crate::R<AnaparXtalOff0Spec>;
#[doc = "Register `ANAPAR_XTAL_OFF_0` writer"]
pub type W = crate::W<AnaparXtalOff0Spec>;
#[doc = "Field `GATED_XTAL_OK0` reader - 1:Gated xtal OK0"]
pub type GatedXtalOk0R = crate::BitReader;
#[doc = "Field `GATED_XTAL_OK0` writer - 1:Gated xtal OK0"]
pub type GatedXtalOk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XQSEL` reader - 0: Schmitt trigger, 1:replica INV(for AFE)"]
pub type XqselR = crate::BitReader;
#[doc = "Field `XQSEL` writer - 0: Schmitt trigger, 1:replica INV(for AFE)"]
pub type XqselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XQSEL_RF_INITIAL` reader - 0: Schmitt trigger, 1:replica INV(for OK0=0) 22C OK0=0, RF no CLK"]
pub type XqselRfInitialR = crate::BitReader;
#[doc = "Field `XQSEL_RF_INITIAL` writer - 0: Schmitt trigger, 1:replica INV(for OK0=0) 22C OK0=0, RF no CLK"]
pub type XqselRfInitialW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XQSEL_RF_AWAKE` reader - 0: Schmitt trigger, 1:replica INV(for OK0=1)"]
pub type XqselRfAwakeR = crate::BitReader;
#[doc = "Field `XQSEL_RF_AWAKE` writer - 0: Schmitt trigger, 1:replica INV(for OK0=1)"]
pub type XqselRfAwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_GM_SEP` reader - 0: GMP=GMN"]
pub type XtalGmSepR = crate::BitReader;
#[doc = "Field `XTAL_GM_SEP` writer - 0: GMP=GMN"]
pub type XtalGmSepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_DRV_RF_LATCH` reader - 1: latch enable"]
pub type XtalDrvRfLatchR = crate::BitReader;
#[doc = "Field `XTAL_DRV_RF_LATCH` writer - 1: latch enable"]
pub type XtalDrvRfLatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_DRV_RF1` reader - Dummy Controlled by RFC"]
pub type XtalDrvRf1R = crate::FieldReader;
#[doc = "Field `XTAL_DRV_RF1` writer - Dummy Controlled by RFC"]
pub type XtalDrvRf1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_DRV_RF2` reader - Driving control of RF2 clock buffer, 11:large current, 00: small current"]
pub type XtalDrvRf2R = crate::FieldReader;
#[doc = "Field `XTAL_DRV_RF2` writer - Driving control of RF2 clock buffer, 11:large current, 00: small current"]
pub type XtalDrvRf2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_DRV_AFE` reader - Driving control of AFE clock buffer, 11:large current, 00: small current"]
pub type XtalDrvAfeR = crate::FieldReader;
#[doc = "Field `XTAL_DRV_AFE` writer - Driving control of AFE clock buffer, 11:large current, 00: small current"]
pub type XtalDrvAfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_DRV_USB` reader - Driving control of USB clock buffer, 11:large current, 00: small current"]
pub type XtalDrvUsbR = crate::FieldReader;
#[doc = "Field `XTAL_DRV_USB` writer - Driving control of USB clock buffer, 11:large current, 00: small current"]
pub type XtalDrvUsbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_DRV_DIGI` reader - Driving control of DIGI/LPS clock buffer, 11:large current, 00: small current"]
pub type XtalDrvDigiR = crate::FieldReader;
#[doc = "Field `XTAL_DRV_DIGI` writer - Driving control of DIGI/LPS clock buffer, 11:large current, 00: small current"]
pub type XtalDrvDigiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_DELAY_AFE` reader - 0: short, 1:long"]
pub type XtalDelayAfeR = crate::BitReader;
#[doc = "Field `XTAL_DELAY_AFE` writer - 0: short, 1:long"]
pub type XtalDelayAfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_DELAY_USB` reader - 0: short, 1:long"]
pub type XtalDelayUsbR = crate::BitReader;
#[doc = "Field `XTAL_DELAY_USB` writer - 0: short, 1:long"]
pub type XtalDelayUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_DELAY_DIGI` reader - 1: turn on XI/XO Cfix, single side 8pF"]
pub type XtalDelayDigiR = crate::BitReader;
#[doc = "Field `XTAL_DELAY_DIGI` writer - 1: turn on XI/XO Cfix, single side 8pF"]
pub type XtalDelayDigiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_LPMODE` reader - 1: low power mode enable 22C turn off replica path, RF1 no CLK RF2 sweep to LPS buf"]
pub type XtalLpmodeR = crate::BitReader;
#[doc = "Field `XTAL_LPMODE` writer - 1: low power mode enable 22C turn off replica path, RF1 no CLK RF2 sweep to LPS buf"]
pub type XtalLpmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_LDO_NC` reader - 1: low current normal mode (22C) 0:high current normal mode (this pin must be pulled to BT control)"]
pub type XtalLdoNcR = crate::BitReader;
#[doc = "Field `XTAL_LDO_NC` writer - 1: low current normal mode (22C) 0:high current normal mode (this pin must be pulled to BT control)"]
pub type XtalLdoNcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_LDO_OPVB_SEL` reader - 0: OP bias from IBG, 1: OP bias from Res divider"]
pub type XtalLdoOpvbSelR = crate::BitReader;
#[doc = "Field `XTAL_LDO_OPVB_SEL` writer - 0: OP bias from IBG, 1: OP bias from Res divider"]
pub type XtalLdoOpvbSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_AAC_PKDET` reader - 1: enable xtal aac peak detector"]
pub type EnXtalAacPkdetR = crate::BitReader;
#[doc = "Field `EN_XTAL_AAC_PKDET` writer - 1: enable xtal aac peak detector"]
pub type EnXtalAacPkdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_AACK_PK_MANU` reader - 1: manual for peak detector peak selection"]
pub type XtalAackPkManuR = crate::BitReader;
#[doc = "Field `XTAL_AACK_PK_MANU` writer - 1: manual for peak detector peak selection"]
pub type XtalAackPkManuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_MANU_PK_SEL` reader - XI Vpk-to-Vpk 00: 0.7V 01: 0.8V 10: 0.9V 11: 1V"]
pub type XtalManuPkSelR = crate::FieldReader;
#[doc = "Field `XTAL_MANU_PK_SEL` writer - XI Vpk-to-Vpk 00: 0.7V 01: 0.8V 10: 0.9V 11: 1V"]
pub type XtalManuPkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTAL_PK_SEL_OFFSET` reader - 1: XI Vpk-to-Vpk offset to 00: 0.4V 01: 0.5V 10: 0.6V 11: 0.7V"]
pub type XtalPkSelOffsetR = crate::BitReader;
#[doc = "Field `XTAL_PK_SEL_OFFSET` writer - 1: XI Vpk-to-Vpk offset to 00: 0.4V 01: 0.5V 10: 0.6V 11: 0.7V"]
pub type XtalPkSelOffsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_SEL_PWR` reader - 1: external 0.9V, 0: internal 0.9V (FIB)"]
pub type XtalSelPwrR = crate::BitReader;
#[doc = "Field `XTAL_SEL_PWR` writer - 1: external 0.9V, 0: internal 0.9V (FIB)"]
pub type XtalSelPwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_XTAL_PDCK_VREF` reader - 1: analog AAC VREF from PDCK digital circuit 0: analog AAC VREF from XTAL_VREF_SEL for BT only mode"]
pub type EnXtalPdckVrefR = crate::BitReader;
#[doc = "Field `EN_XTAL_PDCK_VREF` writer - 1: analog AAC VREF from PDCK digital circuit 0: analog AAC VREF from XTAL_VREF_SEL for BT only mode"]
pub type EnXtalPdckVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_PDCK_OK_MANU` reader - 1: force PDCK_OK=1 for PDCK manual"]
pub type XtalPdckOkManuR = crate::BitReader;
#[doc = "Field `XTAL_PDCK_OK_MANU` writer - 1: force PDCK_OK=1 for PDCK manual"]
pub type XtalPdckOkManuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:Gated xtal OK0"]
    #[inline(always)]
    pub fn gated_xtal_ok0(&self) -> GatedXtalOk0R {
        GatedXtalOk0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: Schmitt trigger, 1:replica INV(for AFE)"]
    #[inline(always)]
    pub fn xqsel(&self) -> XqselR {
        XqselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Schmitt trigger, 1:replica INV(for OK0=0) 22C OK0=0, RF no CLK"]
    #[inline(always)]
    pub fn xqsel_rf_initial(&self) -> XqselRfInitialR {
        XqselRfInitialR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: Schmitt trigger, 1:replica INV(for OK0=1)"]
    #[inline(always)]
    pub fn xqsel_rf_awake(&self) -> XqselRfAwakeR {
        XqselRfAwakeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: GMP=GMN"]
    #[inline(always)]
    pub fn xtal_gm_sep(&self) -> XtalGmSepR {
        XtalGmSepR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: latch enable"]
    #[inline(always)]
    pub fn xtal_drv_rf_latch(&self) -> XtalDrvRfLatchR {
        XtalDrvRfLatchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Dummy Controlled by RFC"]
    #[inline(always)]
    pub fn xtal_drv_rf1(&self) -> XtalDrvRf1R {
        XtalDrvRf1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Driving control of RF2 clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_rf2(&self) -> XtalDrvRf2R {
        XtalDrvRf2R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Driving control of AFE clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_afe(&self) -> XtalDrvAfeR {
        XtalDrvAfeR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Driving control of USB clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_usb(&self) -> XtalDrvUsbR {
        XtalDrvUsbR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - Driving control of DIGI/LPS clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_digi(&self) -> XtalDrvDigiR {
        XtalDrvDigiR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - 0: short, 1:long"]
    #[inline(always)]
    pub fn xtal_delay_afe(&self) -> XtalDelayAfeR {
        XtalDelayAfeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0: short, 1:long"]
    #[inline(always)]
    pub fn xtal_delay_usb(&self) -> XtalDelayUsbR {
        XtalDelayUsbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: turn on XI/XO Cfix, single side 8pF"]
    #[inline(always)]
    pub fn xtal_delay_digi(&self) -> XtalDelayDigiR {
        XtalDelayDigiR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: low power mode enable 22C turn off replica path, RF1 no CLK RF2 sweep to LPS buf"]
    #[inline(always)]
    pub fn xtal_lpmode(&self) -> XtalLpmodeR {
        XtalLpmodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: low current normal mode (22C) 0:high current normal mode (this pin must be pulled to BT control)"]
    #[inline(always)]
    pub fn xtal_ldo_nc(&self) -> XtalLdoNcR {
        XtalLdoNcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 0: OP bias from IBG, 1: OP bias from Res divider"]
    #[inline(always)]
    pub fn xtal_ldo_opvb_sel(&self) -> XtalLdoOpvbSelR {
        XtalLdoOpvbSelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: enable xtal aac peak detector"]
    #[inline(always)]
    pub fn en_xtal_aac_pkdet(&self) -> EnXtalAacPkdetR {
        EnXtalAacPkdetR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: manual for peak detector peak selection"]
    #[inline(always)]
    pub fn xtal_aack_pk_manu(&self) -> XtalAackPkManuR {
        XtalAackPkManuR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - XI Vpk-to-Vpk 00: 0.7V 01: 0.8V 10: 0.9V 11: 1V"]
    #[inline(always)]
    pub fn xtal_manu_pk_sel(&self) -> XtalManuPkSelR {
        XtalManuPkSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 1: XI Vpk-to-Vpk offset to 00: 0.4V 01: 0.5V 10: 0.6V 11: 0.7V"]
    #[inline(always)]
    pub fn xtal_pk_sel_offset(&self) -> XtalPkSelOffsetR {
        XtalPkSelOffsetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: external 0.9V, 0: internal 0.9V (FIB)"]
    #[inline(always)]
    pub fn xtal_sel_pwr(&self) -> XtalSelPwrR {
        XtalSelPwrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: analog AAC VREF from PDCK digital circuit 0: analog AAC VREF from XTAL_VREF_SEL for BT only mode"]
    #[inline(always)]
    pub fn en_xtal_pdck_vref(&self) -> EnXtalPdckVrefR {
        EnXtalPdckVrefR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: force PDCK_OK=1 for PDCK manual"]
    #[inline(always)]
    pub fn xtal_pdck_ok_manu(&self) -> XtalPdckOkManuR {
        XtalPdckOkManuR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:Gated xtal OK0"]
    #[inline(always)]
    pub fn gated_xtal_ok0(&mut self) -> GatedXtalOk0W<'_, AnaparXtalOff0Spec> {
        GatedXtalOk0W::new(self, 1)
    }
    #[doc = "Bit 2 - 0: Schmitt trigger, 1:replica INV(for AFE)"]
    #[inline(always)]
    pub fn xqsel(&mut self) -> XqselW<'_, AnaparXtalOff0Spec> {
        XqselW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: Schmitt trigger, 1:replica INV(for OK0=0) 22C OK0=0, RF no CLK"]
    #[inline(always)]
    pub fn xqsel_rf_initial(&mut self) -> XqselRfInitialW<'_, AnaparXtalOff0Spec> {
        XqselRfInitialW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: Schmitt trigger, 1:replica INV(for OK0=1)"]
    #[inline(always)]
    pub fn xqsel_rf_awake(&mut self) -> XqselRfAwakeW<'_, AnaparXtalOff0Spec> {
        XqselRfAwakeW::new(self, 4)
    }
    #[doc = "Bit 5 - 0: GMP=GMN"]
    #[inline(always)]
    pub fn xtal_gm_sep(&mut self) -> XtalGmSepW<'_, AnaparXtalOff0Spec> {
        XtalGmSepW::new(self, 5)
    }
    #[doc = "Bit 6 - 1: latch enable"]
    #[inline(always)]
    pub fn xtal_drv_rf_latch(&mut self) -> XtalDrvRfLatchW<'_, AnaparXtalOff0Spec> {
        XtalDrvRfLatchW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Dummy Controlled by RFC"]
    #[inline(always)]
    pub fn xtal_drv_rf1(&mut self) -> XtalDrvRf1W<'_, AnaparXtalOff0Spec> {
        XtalDrvRf1W::new(self, 7)
    }
    #[doc = "Bits 9:10 - Driving control of RF2 clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_rf2(&mut self) -> XtalDrvRf2W<'_, AnaparXtalOff0Spec> {
        XtalDrvRf2W::new(self, 9)
    }
    #[doc = "Bits 11:12 - Driving control of AFE clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_afe(&mut self) -> XtalDrvAfeW<'_, AnaparXtalOff0Spec> {
        XtalDrvAfeW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Driving control of USB clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_usb(&mut self) -> XtalDrvUsbW<'_, AnaparXtalOff0Spec> {
        XtalDrvUsbW::new(self, 13)
    }
    #[doc = "Bits 15:16 - Driving control of DIGI/LPS clock buffer, 11:large current, 00: small current"]
    #[inline(always)]
    pub fn xtal_drv_digi(&mut self) -> XtalDrvDigiW<'_, AnaparXtalOff0Spec> {
        XtalDrvDigiW::new(self, 15)
    }
    #[doc = "Bit 17 - 0: short, 1:long"]
    #[inline(always)]
    pub fn xtal_delay_afe(&mut self) -> XtalDelayAfeW<'_, AnaparXtalOff0Spec> {
        XtalDelayAfeW::new(self, 17)
    }
    #[doc = "Bit 18 - 0: short, 1:long"]
    #[inline(always)]
    pub fn xtal_delay_usb(&mut self) -> XtalDelayUsbW<'_, AnaparXtalOff0Spec> {
        XtalDelayUsbW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: turn on XI/XO Cfix, single side 8pF"]
    #[inline(always)]
    pub fn xtal_delay_digi(&mut self) -> XtalDelayDigiW<'_, AnaparXtalOff0Spec> {
        XtalDelayDigiW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: low power mode enable 22C turn off replica path, RF1 no CLK RF2 sweep to LPS buf"]
    #[inline(always)]
    pub fn xtal_lpmode(&mut self) -> XtalLpmodeW<'_, AnaparXtalOff0Spec> {
        XtalLpmodeW::new(self, 20)
    }
    #[doc = "Bit 21 - 1: low current normal mode (22C) 0:high current normal mode (this pin must be pulled to BT control)"]
    #[inline(always)]
    pub fn xtal_ldo_nc(&mut self) -> XtalLdoNcW<'_, AnaparXtalOff0Spec> {
        XtalLdoNcW::new(self, 21)
    }
    #[doc = "Bit 22 - 0: OP bias from IBG, 1: OP bias from Res divider"]
    #[inline(always)]
    pub fn xtal_ldo_opvb_sel(&mut self) -> XtalLdoOpvbSelW<'_, AnaparXtalOff0Spec> {
        XtalLdoOpvbSelW::new(self, 22)
    }
    #[doc = "Bit 24 - 1: enable xtal aac peak detector"]
    #[inline(always)]
    pub fn en_xtal_aac_pkdet(&mut self) -> EnXtalAacPkdetW<'_, AnaparXtalOff0Spec> {
        EnXtalAacPkdetW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: manual for peak detector peak selection"]
    #[inline(always)]
    pub fn xtal_aack_pk_manu(&mut self) -> XtalAackPkManuW<'_, AnaparXtalOff0Spec> {
        XtalAackPkManuW::new(self, 25)
    }
    #[doc = "Bits 26:27 - XI Vpk-to-Vpk 00: 0.7V 01: 0.8V 10: 0.9V 11: 1V"]
    #[inline(always)]
    pub fn xtal_manu_pk_sel(&mut self) -> XtalManuPkSelW<'_, AnaparXtalOff0Spec> {
        XtalManuPkSelW::new(self, 26)
    }
    #[doc = "Bit 28 - 1: XI Vpk-to-Vpk offset to 00: 0.4V 01: 0.5V 10: 0.6V 11: 0.7V"]
    #[inline(always)]
    pub fn xtal_pk_sel_offset(&mut self) -> XtalPkSelOffsetW<'_, AnaparXtalOff0Spec> {
        XtalPkSelOffsetW::new(self, 28)
    }
    #[doc = "Bit 29 - 1: external 0.9V, 0: internal 0.9V (FIB)"]
    #[inline(always)]
    pub fn xtal_sel_pwr(&mut self) -> XtalSelPwrW<'_, AnaparXtalOff0Spec> {
        XtalSelPwrW::new(self, 29)
    }
    #[doc = "Bit 30 - 1: analog AAC VREF from PDCK digital circuit 0: analog AAC VREF from XTAL_VREF_SEL for BT only mode"]
    #[inline(always)]
    pub fn en_xtal_pdck_vref(&mut self) -> EnXtalPdckVrefW<'_, AnaparXtalOff0Spec> {
        EnXtalPdckVrefW::new(self, 30)
    }
    #[doc = "Bit 31 - 1: force PDCK_OK=1 for PDCK manual"]
    #[inline(always)]
    pub fn xtal_pdck_ok_manu(&mut self) -> XtalPdckOkManuW<'_, AnaparXtalOff0Spec> {
        XtalPdckOkManuW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`anapar_xtal_off_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anapar_xtal_off_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaparXtalOff0Spec;
impl crate::RegisterSpec for AnaparXtalOff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anapar_xtal_off_0::R`](R) reader structure"]
impl crate::Readable for AnaparXtalOff0Spec {}
#[doc = "`write(|w| ..)` method takes [`anapar_xtal_off_0::W`](W) writer structure"]
impl crate::Writable for AnaparXtalOff0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPAR_XTAL_OFF_0 to value 0xcc0f_ff90"]
impl crate::Resettable for AnaparXtalOff0Spec {
    const RESET_VALUE: u32 = 0xcc0f_ff90;
}
