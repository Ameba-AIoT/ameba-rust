#[doc = "Register `REG_WMAC_OPTION_FUNCTION_3` reader"]
pub type R = crate::R<RegWmacOptionFunction3Spec>;
#[doc = "Register `REG_WMAC_OPTION_FUNCTION_3` writer"]
pub type W = crate::W<RegWmacOptionFunction3Spec>;
#[doc = "Field `BIT_r_WMAC_TXNDP_SIGB` reader - SIGB for NDP packet when BW =40MHz"]
pub type BitRWmacTxndpSigbR = crate::FieldReader<u32>;
#[doc = "Field `BIT_r_WMAC_TXNDP_SIGB` writer - SIGB for NDP packet when BW =40MHz"]
pub type BitRWmacTxndpSigbW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `BIT_r_WMAC_MORE_DATA` reader - more data field in FC for ack/ba/mba"]
pub type BitRWmacMoreDataR = crate::BitReader;
#[doc = "Field `BIT_r_WMAC_MORE_DATA` writer - more data field in FC for ack/ba/mba"]
pub type BitRWmacMoreDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_SIFS_ABORT_TB` reader - sifs_reach check fcs fail abort tx tb option"]
pub type BitRSifsAbortTbR = crate::BitReader;
#[doc = "Field `BIT_r_SIFS_ABORT_TB` writer - sifs_reach check fcs fail abort tx tb option"]
pub type BitRSifsAbortTbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:20 - SIGB for NDP packet when BW =40MHz"]
    #[inline(always)]
    pub fn bit_r_wmac_txndp_sigb(&self) -> BitRWmacTxndpSigbR {
        BitRWmacTxndpSigbR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 21 - more data field in FC for ack/ba/mba"]
    #[inline(always)]
    pub fn bit_r_wmac_more_data(&self) -> BitRWmacMoreDataR {
        BitRWmacMoreDataR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - sifs_reach check fcs fail abort tx tb option"]
    #[inline(always)]
    pub fn bit_r_sifs_abort_tb(&self) -> BitRSifsAbortTbR {
        BitRSifsAbortTbR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:20 - SIGB for NDP packet when BW =40MHz"]
    #[inline(always)]
    pub fn bit_r_wmac_txndp_sigb(&mut self) -> BitRWmacTxndpSigbW<'_, RegWmacOptionFunction3Spec> {
        BitRWmacTxndpSigbW::new(self, 0)
    }
    #[doc = "Bit 21 - more data field in FC for ack/ba/mba"]
    #[inline(always)]
    pub fn bit_r_wmac_more_data(&mut self) -> BitRWmacMoreDataW<'_, RegWmacOptionFunction3Spec> {
        BitRWmacMoreDataW::new(self, 21)
    }
    #[doc = "Bit 22 - sifs_reach check fcs fail abort tx tb option"]
    #[inline(always)]
    pub fn bit_r_sifs_abort_tb(&mut self) -> BitRSifsAbortTbW<'_, RegWmacOptionFunction3Spec> {
        BitRSifsAbortTbW::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_option_function_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_option_function_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacOptionFunction3Spec;
impl crate::RegisterSpec for RegWmacOptionFunction3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_option_function_3::R`](R) reader structure"]
impl crate::Readable for RegWmacOptionFunction3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_option_function_3::W`](W) writer structure"]
impl crate::Writable for RegWmacOptionFunction3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_OPTION_FUNCTION_3 to value 0x0058_45a5"]
impl crate::Resettable for RegWmacOptionFunction3Spec {
    const RESET_VALUE: u32 = 0x0058_45a5;
}
