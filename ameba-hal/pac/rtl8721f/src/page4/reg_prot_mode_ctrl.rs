#[doc = "Register `REG_PROT_MODE_CTRL` reader"]
pub type R = crate::R<RegProtModeCtrlSpec>;
#[doc = "Register `REG_PROT_MODE_CTRL` writer"]
pub type W = crate::W<RegProtModeCtrlSpec>;
#[doc = "Field `BIT_RTS_LEN_TH` reader - RTS packet Length threshold, unit is 128 bytes"]
pub type BitRtsLenThR = crate::FieldReader;
#[doc = "Field `BIT_RTS_LEN_TH` writer - RTS packet Length threshold, unit is 128 bytes"]
pub type BitRtsLenThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RTS_TXTIME_TH` reader - RTS packet txtime threshold, unit is 64 us"]
pub type BitRtsTxtimeThR = crate::FieldReader;
#[doc = "Field `BIT_RTS_TXTIME_TH` writer - RTS packet txtime threshold, unit is 64 us"]
pub type BitRtsTxtimeThW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_MAX_AGG_NUM` reader - the maximum aggregation packet number limit,unit is byte"]
pub type BitMaxAggNumR = crate::FieldReader;
#[doc = "Field `BIT_MAX_AGG_NUM` writer - the maximum aggregation packet number limit,unit is byte"]
pub type BitMaxAggNumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_RTS_MAX_AGG_NUM` reader - In RTS frame exchange sequence, the maximum aggregation packet number limit is defined by this register, unit is byte"]
pub type BitRtsMaxAggNumR = crate::FieldReader;
#[doc = "Field `BIT_RTS_MAX_AGG_NUM` writer - In RTS frame exchange sequence, the maximum aggregation packet number limit is defined by this register, unit is byte"]
pub type BitRtsMaxAggNumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - RTS packet Length threshold, unit is 128 bytes"]
    #[inline(always)]
    pub fn bit_rts_len_th(&self) -> BitRtsLenThR {
        BitRtsLenThR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RTS packet txtime threshold, unit is 64 us"]
    #[inline(always)]
    pub fn bit_rts_txtime_th(&self) -> BitRtsTxtimeThR {
        BitRtsTxtimeThR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - the maximum aggregation packet number limit,unit is byte"]
    #[inline(always)]
    pub fn bit_max_agg_num(&self) -> BitMaxAggNumR {
        BitMaxAggNumR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - In RTS frame exchange sequence, the maximum aggregation packet number limit is defined by this register, unit is byte"]
    #[inline(always)]
    pub fn bit_rts_max_agg_num(&self) -> BitRtsMaxAggNumR {
        BitRtsMaxAggNumR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RTS packet Length threshold, unit is 128 bytes"]
    #[inline(always)]
    pub fn bit_rts_len_th(&mut self) -> BitRtsLenThW<'_, RegProtModeCtrlSpec> {
        BitRtsLenThW::new(self, 0)
    }
    #[doc = "Bits 8:15 - RTS packet txtime threshold, unit is 64 us"]
    #[inline(always)]
    pub fn bit_rts_txtime_th(&mut self) -> BitRtsTxtimeThW<'_, RegProtModeCtrlSpec> {
        BitRtsTxtimeThW::new(self, 8)
    }
    #[doc = "Bits 16:21 - the maximum aggregation packet number limit,unit is byte"]
    #[inline(always)]
    pub fn bit_max_agg_num(&mut self) -> BitMaxAggNumW<'_, RegProtModeCtrlSpec> {
        BitMaxAggNumW::new(self, 16)
    }
    #[doc = "Bits 24:29 - In RTS frame exchange sequence, the maximum aggregation packet number limit is defined by this register, unit is byte"]
    #[inline(always)]
    pub fn bit_rts_max_agg_num(&mut self) -> BitRtsMaxAggNumW<'_, RegProtModeCtrlSpec> {
        BitRtsMaxAggNumW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prot_mode_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prot_mode_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegProtModeCtrlSpec;
impl crate::RegisterSpec for RegProtModeCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_prot_mode_ctrl::R`](R) reader structure"]
impl crate::Readable for RegProtModeCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_prot_mode_ctrl::W`](W) writer structure"]
impl crate::Writable for RegProtModeCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PROT_MODE_CTRL to value 0x0c14_0101"]
impl crate::Resettable for RegProtModeCtrlSpec {
    const RESET_VALUE: u32 = 0x0c14_0101;
}
