#[doc = "Register `REG_CFEND_RATE_SC_CTRL` reader"]
pub type R = crate::R<RegCfendRateScCtrlSpec>;
#[doc = "Register `REG_CFEND_RATE_SC_CTRL` writer"]
pub type W = crate::W<RegCfendRateScCtrlSpec>;
#[doc = "Field `BIT_LEAG_RTS_BW_DUP` reader - If this bit is set, leag rate RTS use Duplicate in 40M BW / 80M BW."]
pub type BitLeagRtsBwDupR = crate::BitReader;
#[doc = "Field `BIT_LEAG_RTS_BW_DUP` writer - If this bit is set, leag rate RTS use Duplicate in 40M BW / 80M BW."]
pub type BitLeagRtsBwDupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_Basic_CFEND_RATE` reader - The initial rate for basic CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. OFDM6M"]
pub type BitBasicCfendRateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_Basic_CFEND_RATE` writer - The initial rate for basic CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. OFDM6M"]
pub type BitBasicCfendRateW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_STBC_CFEND_RATE` reader - The initial rate for STBC CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. MCS0"]
pub type BitStbcCfendRateR = crate::FieldReader<u16>;
#[doc = "Field `BIT_STBC_CFEND_RATE` writer - The initial rate for STBC CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. MCS0"]
pub type BitStbcCfendRateW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_TXSC_20M` reader - Value of Data frame Tx sub-carrier mode 20MHz"]
pub type BitTxsc20mR = crate::FieldReader;
#[doc = "Field `BIT_TXSC_20M` writer - Value of Data frame Tx sub-carrier mode 20MHz"]
pub type BitTxsc20mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_TXSC_40M` reader - Value of Data frame Tx sub-carrier mode 40MHz"]
pub type BitTxsc40mR = crate::FieldReader;
#[doc = "Field `BIT_TXSC_40M` writer - Value of Data frame Tx sub-carrier mode 40MHz"]
pub type BitTxsc40mW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 5 - If this bit is set, leag rate RTS use Duplicate in 40M BW / 80M BW."]
    #[inline(always)]
    pub fn bit_leag_rts_bw_dup(&self) -> BitLeagRtsBwDupR {
        BitLeagRtsBwDupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:14 - The initial rate for basic CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. OFDM6M"]
    #[inline(always)]
    pub fn bit_basic_cfend_rate(&self) -> BitBasicCfendRateR {
        BitBasicCfendRateR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 15:23 - The initial rate for STBC CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. MCS0"]
    #[inline(always)]
    pub fn bit_stbc_cfend_rate(&self) -> BitStbcCfendRateR {
        BitStbcCfendRateR::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:27 - Value of Data frame Tx sub-carrier mode 20MHz"]
    #[inline(always)]
    pub fn bit_txsc_20m(&self) -> BitTxsc20mR {
        BitTxsc20mR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Value of Data frame Tx sub-carrier mode 40MHz"]
    #[inline(always)]
    pub fn bit_txsc_40m(&self) -> BitTxsc40mR {
        BitTxsc40mR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - If this bit is set, leag rate RTS use Duplicate in 40M BW / 80M BW."]
    #[inline(always)]
    pub fn bit_leag_rts_bw_dup(&mut self) -> BitLeagRtsBwDupW<'_, RegCfendRateScCtrlSpec> {
        BitLeagRtsBwDupW::new(self, 5)
    }
    #[doc = "Bits 6:14 - The initial rate for basic CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. OFDM6M"]
    #[inline(always)]
    pub fn bit_basic_cfend_rate(&mut self) -> BitBasicCfendRateW<'_, RegCfendRateScCtrlSpec> {
        BitBasicCfendRateW::new(self, 6)
    }
    #[doc = "Bits 15:23 - The initial rate for STBC CFEND frame transmitted as EDCA originator. The response control frame shall be selected by the definition of 802.11n. MCS0"]
    #[inline(always)]
    pub fn bit_stbc_cfend_rate(&mut self) -> BitStbcCfendRateW<'_, RegCfendRateScCtrlSpec> {
        BitStbcCfendRateW::new(self, 15)
    }
    #[doc = "Bits 24:27 - Value of Data frame Tx sub-carrier mode 20MHz"]
    #[inline(always)]
    pub fn bit_txsc_20m(&mut self) -> BitTxsc20mW<'_, RegCfendRateScCtrlSpec> {
        BitTxsc20mW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Value of Data frame Tx sub-carrier mode 40MHz"]
    #[inline(always)]
    pub fn bit_txsc_40m(&mut self) -> BitTxsc40mW<'_, RegCfendRateScCtrlSpec> {
        BitTxsc40mW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cfend_rate_sc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cfend_rate_sc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCfendRateScCtrlSpec;
impl crate::RegisterSpec for RegCfendRateScCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cfend_rate_sc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCfendRateScCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cfend_rate_sc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCfendRateScCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CFEND_RATE_SC_CTRL to value 0x0040_0100"]
impl crate::Resettable for RegCfendRateScCtrlSpec {
    const RESET_VALUE: u32 = 0x0040_0100;
}
