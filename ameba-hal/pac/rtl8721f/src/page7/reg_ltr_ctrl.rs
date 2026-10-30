#[doc = "Register `REG_LTR_CTRL` reader"]
pub type R = crate::R<RegLtrCtrlSpec>;
#[doc = "Register `REG_LTR_CTRL` writer"]
pub type W = crate::W<RegLtrCtrlSpec>;
#[doc = "Field `BIT_WMAC_INI_SIG` reader - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
pub type BitWmacIniSigR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_INI_SIG` writer - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
pub type BitWmacIniSigW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WMAC_OPT_SIG1` reader - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
pub type BitWmacOptSig1R = crate::FieldReader;
#[doc = "Field `BIT_WMAC_OPT_SIG1` writer - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
pub type BitWmacOptSig1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_WMAC_OPT_SIG2` reader - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
pub type BitWmacOptSig2R = crate::FieldReader;
#[doc = "Field `BIT_WMAC_OPT_SIG2` writer - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
pub type BitWmacOptSig2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_TX_VENDOR_ADV_EN` reader - when set, tx vendor adv function is true"]
pub type BitTxVendorAdvEnR = crate::BitReader;
#[doc = "Field `BIT_TX_VENDOR_ADV_EN` writer - when set, tx vendor adv function is true"]
pub type BitTxVendorAdvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RX_VENDOR_ADV_EN` reader - when set, rx vendor adv function is true"]
pub type BitRxVendorAdvEnR = crate::BitReader;
#[doc = "Field `BIT_RX_VENDOR_ADV_EN` writer - when set, rx vendor adv function is true"]
pub type BitRxVendorAdvEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
    #[inline(always)]
    pub fn bit_wmac_ini_sig(&self) -> BitWmacIniSigR {
        BitWmacIniSigR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
    #[inline(always)]
    pub fn bit_wmac_opt_sig1(&self) -> BitWmacOptSig1R {
        BitWmacOptSig1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
    #[inline(always)]
    pub fn bit_wmac_opt_sig2(&self) -> BitWmacOptSig2R {
        BitWmacOptSig2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - when set, tx vendor adv function is true"]
    #[inline(always)]
    pub fn bit_tx_vendor_adv_en(&self) -> BitTxVendorAdvEnR {
        BitTxVendorAdvEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - when set, rx vendor adv function is true"]
    #[inline(always)]
    pub fn bit_rx_vendor_adv_en(&self) -> BitRxVendorAdvEnR {
        BitRxVendorAdvEnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
    #[inline(always)]
    pub fn bit_wmac_ini_sig(&mut self) -> BitWmacIniSigW<'_, RegLtrCtrlSpec> {
        BitWmacIniSigW::new(self, 0)
    }
    #[doc = "Bits 8:15 - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
    #[inline(always)]
    pub fn bit_wmac_opt_sig1(&mut self) -> BitWmacOptSig1W<'_, RegLtrCtrlSpec> {
        BitWmacOptSig1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - For rx ofdm ppdu lsig signal filed check when rx vendor adv is true"]
    #[inline(always)]
    pub fn bit_wmac_opt_sig2(&mut self) -> BitWmacOptSig2W<'_, RegLtrCtrlSpec> {
        BitWmacOptSig2W::new(self, 16)
    }
    #[doc = "Bit 24 - when set, tx vendor adv function is true"]
    #[inline(always)]
    pub fn bit_tx_vendor_adv_en(&mut self) -> BitTxVendorAdvEnW<'_, RegLtrCtrlSpec> {
        BitTxVendorAdvEnW::new(self, 24)
    }
    #[doc = "Bit 25 - when set, rx vendor adv function is true"]
    #[inline(always)]
    pub fn bit_rx_vendor_adv_en(&mut self) -> BitRxVendorAdvEnW<'_, RegLtrCtrlSpec> {
        BitRxVendorAdvEnW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ltr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ltr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLtrCtrlSpec;
impl crate::RegisterSpec for RegLtrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ltr_ctrl::R`](R) reader structure"]
impl crate::Readable for RegLtrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ltr_ctrl::W`](W) writer structure"]
impl crate::Writable for RegLtrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LTR_CTRL to value 0x03ec_ecec"]
impl crate::Resettable for RegLtrCtrlSpec {
    const RESET_VALUE: u32 = 0x03ec_ecec;
}
