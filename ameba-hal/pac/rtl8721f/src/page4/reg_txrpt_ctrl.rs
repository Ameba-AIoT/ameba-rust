#[doc = "Register `REG_TXRPT_CTRL` reader"]
pub type R = crate::R<RegTxrptCtrlSpec>;
#[doc = "Register `REG_TXRPT_CTRL` writer"]
pub type W = crate::W<RegTxrptCtrlSpec>;
#[doc = "Field `BIT_AMPDU_TXRPT_W_PTR` reader - Write pointer of ampdu txrpt"]
pub type BitAmpduTxrptWPtrR = crate::FieldReader;
#[doc = "Field `BIT_AMPDU_TXRPT_R_PTR` reader - Read pointer of ampdu txrpt"]
pub type BitAmpduTxrptRPtrR = crate::FieldReader;
#[doc = "Field `BIT_AMPDU_TXRPT_R_PTR` writer - Read pointer of ampdu txrpt"]
pub type BitAmpduTxrptRPtrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_AMPDU_TXRPT_LEN_THRS` reader - Length threshold of ampdu txrpt"]
pub type BitAmpduTxrptLenThrsR = crate::FieldReader;
#[doc = "Field `BIT_AMPDU_TXRPT_LEN_THRS` writer - Length threshold of ampdu txrpt"]
pub type BitAmpduTxrptLenThrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_AMPDU_TXRPT_TIME_THRS` reader - Time threshold of ampdu txrpt"]
pub type BitAmpduTxrptTimeThrsR = crate::FieldReader;
#[doc = "Field `BIT_AMPDU_TXRPT_TIME_THRS` writer - Time threshold of ampdu txrpt"]
pub type BitAmpduTxrptTimeThrsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write pointer of ampdu txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_w_ptr(&self) -> BitAmpduTxrptWPtrR {
        BitAmpduTxrptWPtrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read pointer of ampdu txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_r_ptr(&self) -> BitAmpduTxrptRPtrR {
        BitAmpduTxrptRPtrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Length threshold of ampdu txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_len_thrs(&self) -> BitAmpduTxrptLenThrsR {
        BitAmpduTxrptLenThrsR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Time threshold of ampdu txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_time_thrs(&self) -> BitAmpduTxrptTimeThrsR {
        BitAmpduTxrptTimeThrsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Read pointer of ampdu txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_r_ptr(&mut self) -> BitAmpduTxrptRPtrW<'_, RegTxrptCtrlSpec> {
        BitAmpduTxrptRPtrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Length threshold of ampdu txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_len_thrs(&mut self) -> BitAmpduTxrptLenThrsW<'_, RegTxrptCtrlSpec> {
        BitAmpduTxrptLenThrsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Time threshold of ampdu txrpt"]
    #[inline(always)]
    pub fn bit_ampdu_txrpt_time_thrs(&mut self) -> BitAmpduTxrptTimeThrsW<'_, RegTxrptCtrlSpec> {
        BitAmpduTxrptTimeThrsW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txrpt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txrpt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxrptCtrlSpec;
impl crate::RegisterSpec for RegTxrptCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txrpt_ctrl::R`](R) reader structure"]
impl crate::Readable for RegTxrptCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txrpt_ctrl::W`](W) writer structure"]
impl crate::Writable for RegTxrptCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXRPT_CTRL to value 0x1001_0000"]
impl crate::Resettable for RegTxrptCtrlSpec {
    const RESET_VALUE: u32 = 0x1001_0000;
}
