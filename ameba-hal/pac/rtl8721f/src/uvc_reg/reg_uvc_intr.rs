#[doc = "Register `REG_UVC_INTR` reader"]
pub type R = crate::R<RegUvcIntrSpec>;
#[doc = "Register `REG_UVC_INTR` writer"]
pub type W = crate::W<RegUvcIntrSpec>;
#[doc = "Field `ch0_buf0_frm_done` reader - "]
pub type Ch0Buf0FrmDoneR = crate::BitReader;
#[doc = "Field `ch0_buf1_frm_done` reader - "]
pub type Ch0Buf1FrmDoneR = crate::BitReader;
#[doc = "Field `ch0_buf0_frm_oversize` reader - "]
pub type Ch0Buf0FrmOversizeR = crate::BitReader;
#[doc = "Field `ch0_buf1_frm_oversize` reader - "]
pub type Ch0Buf1FrmOversizeR = crate::BitReader;
#[doc = "Field `ch0_buf0_header_err` reader - "]
pub type Ch0Buf0HeaderErrR = crate::BitReader;
#[doc = "Field `ch0_buf1_header_err` reader - "]
pub type Ch0Buf1HeaderErrR = crate::BitReader;
#[doc = "Field `ch1_buf0_frm_done` reader - "]
pub type Ch1Buf0FrmDoneR = crate::BitReader;
#[doc = "Field `ch1_buf1_frm_done` reader - "]
pub type Ch1Buf1FrmDoneR = crate::BitReader;
#[doc = "Field `ch1_buf0_frm_oversize` reader - "]
pub type Ch1Buf0FrmOversizeR = crate::BitReader;
#[doc = "Field `ch1_buf1_frm_oversize` reader - "]
pub type Ch1Buf1FrmOversizeR = crate::BitReader;
#[doc = "Field `ch1_buf0_header_err` reader - "]
pub type Ch1Buf0HeaderErrR = crate::BitReader;
#[doc = "Field `ch1_buf1_header_err` reader - "]
pub type Ch1Buf1HeaderErrR = crate::BitReader;
#[doc = "Field `ch0_buf0_frm_done_mask` reader - "]
pub type Ch0Buf0FrmDoneMaskR = crate::BitReader;
#[doc = "Field `ch0_buf0_frm_done_mask` writer - "]
pub type Ch0Buf0FrmDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_buf1_frm_done_mask` reader - "]
pub type Ch0Buf1FrmDoneMaskR = crate::BitReader;
#[doc = "Field `ch0_buf1_frm_done_mask` writer - "]
pub type Ch0Buf1FrmDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_buf0_frm_oversize_mask` reader - "]
pub type Ch0Buf0FrmOversizeMaskR = crate::BitReader;
#[doc = "Field `ch0_buf0_frm_oversize_mask` writer - "]
pub type Ch0Buf0FrmOversizeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_buf1_frm_oversize_mask` reader - "]
pub type Ch0Buf1FrmOversizeMaskR = crate::BitReader;
#[doc = "Field `ch0_buf1_frm_oversize_mask` writer - "]
pub type Ch0Buf1FrmOversizeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_buf0_header_err_mask` reader - "]
pub type Ch0Buf0HeaderErrMaskR = crate::BitReader;
#[doc = "Field `ch0_buf0_header_err_mask` writer - "]
pub type Ch0Buf0HeaderErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_buf1_header_err_mask` reader - "]
pub type Ch0Buf1HeaderErrMaskR = crate::BitReader;
#[doc = "Field `ch0_buf1_header_err_mask` writer - "]
pub type Ch0Buf1HeaderErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_buf0_frm_done_mask` reader - "]
pub type Ch1Buf0FrmDoneMaskR = crate::BitReader;
#[doc = "Field `ch1_buf0_frm_done_mask` writer - "]
pub type Ch1Buf0FrmDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_buf1_frm_done_mask` reader - "]
pub type Ch1Buf1FrmDoneMaskR = crate::BitReader;
#[doc = "Field `ch1_buf1_frm_done_mask` writer - "]
pub type Ch1Buf1FrmDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_buf0_frm_oversize_mask` reader - "]
pub type Ch1Buf0FrmOversizeMaskR = crate::BitReader;
#[doc = "Field `ch1_buf0_frm_oversize_mask` writer - "]
pub type Ch1Buf0FrmOversizeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_buf1_frm_oversize_mask` reader - "]
pub type Ch1Buf1FrmOversizeMaskR = crate::BitReader;
#[doc = "Field `ch1_buf1_frm_oversize_mask` writer - "]
pub type Ch1Buf1FrmOversizeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_buf0_header_err_mask` reader - "]
pub type Ch1Buf0HeaderErrMaskR = crate::BitReader;
#[doc = "Field `ch1_buf0_header_err_mask` writer - "]
pub type Ch1Buf0HeaderErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_buf1_header_err_mask` reader - "]
pub type Ch1Buf1HeaderErrMaskR = crate::BitReader;
#[doc = "Field `ch1_buf1_header_err_mask` writer - "]
pub type Ch1Buf1HeaderErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_buf0_frm_done(&self) -> Ch0Buf0FrmDoneR {
        Ch0Buf0FrmDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch0_buf1_frm_done(&self) -> Ch0Buf1FrmDoneR {
        Ch0Buf1FrmDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch0_buf0_frm_oversize(&self) -> Ch0Buf0FrmOversizeR {
        Ch0Buf0FrmOversizeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch0_buf1_frm_oversize(&self) -> Ch0Buf1FrmOversizeR {
        Ch0Buf1FrmOversizeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch0_buf0_header_err(&self) -> Ch0Buf0HeaderErrR {
        Ch0Buf0HeaderErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch0_buf1_header_err(&self) -> Ch0Buf1HeaderErrR {
        Ch0Buf1HeaderErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch1_buf0_frm_done(&self) -> Ch1Buf0FrmDoneR {
        Ch1Buf0FrmDoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch1_buf1_frm_done(&self) -> Ch1Buf1FrmDoneR {
        Ch1Buf1FrmDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch1_buf0_frm_oversize(&self) -> Ch1Buf0FrmOversizeR {
        Ch1Buf0FrmOversizeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch1_buf1_frm_oversize(&self) -> Ch1Buf1FrmOversizeR {
        Ch1Buf1FrmOversizeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch1_buf0_header_err(&self) -> Ch1Buf0HeaderErrR {
        Ch1Buf0HeaderErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch1_buf1_header_err(&self) -> Ch1Buf1HeaderErrR {
        Ch1Buf1HeaderErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ch0_buf0_frm_done_mask(&self) -> Ch0Buf0FrmDoneMaskR {
        Ch0Buf0FrmDoneMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ch0_buf1_frm_done_mask(&self) -> Ch0Buf1FrmDoneMaskR {
        Ch0Buf1FrmDoneMaskR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ch0_buf0_frm_oversize_mask(&self) -> Ch0Buf0FrmOversizeMaskR {
        Ch0Buf0FrmOversizeMaskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ch0_buf1_frm_oversize_mask(&self) -> Ch0Buf1FrmOversizeMaskR {
        Ch0Buf1FrmOversizeMaskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ch0_buf0_header_err_mask(&self) -> Ch0Buf0HeaderErrMaskR {
        Ch0Buf0HeaderErrMaskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ch0_buf1_header_err_mask(&self) -> Ch0Buf1HeaderErrMaskR {
        Ch0Buf1HeaderErrMaskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ch1_buf0_frm_done_mask(&self) -> Ch1Buf0FrmDoneMaskR {
        Ch1Buf0FrmDoneMaskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ch1_buf1_frm_done_mask(&self) -> Ch1Buf1FrmDoneMaskR {
        Ch1Buf1FrmDoneMaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ch1_buf0_frm_oversize_mask(&self) -> Ch1Buf0FrmOversizeMaskR {
        Ch1Buf0FrmOversizeMaskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ch1_buf1_frm_oversize_mask(&self) -> Ch1Buf1FrmOversizeMaskR {
        Ch1Buf1FrmOversizeMaskR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ch1_buf0_header_err_mask(&self) -> Ch1Buf0HeaderErrMaskR {
        Ch1Buf0HeaderErrMaskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ch1_buf1_header_err_mask(&self) -> Ch1Buf1HeaderErrMaskR {
        Ch1Buf1HeaderErrMaskR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ch0_buf0_frm_done_mask(&mut self) -> Ch0Buf0FrmDoneMaskW<'_, RegUvcIntrSpec> {
        Ch0Buf0FrmDoneMaskW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ch0_buf1_frm_done_mask(&mut self) -> Ch0Buf1FrmDoneMaskW<'_, RegUvcIntrSpec> {
        Ch0Buf1FrmDoneMaskW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ch0_buf0_frm_oversize_mask(&mut self) -> Ch0Buf0FrmOversizeMaskW<'_, RegUvcIntrSpec> {
        Ch0Buf0FrmOversizeMaskW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ch0_buf1_frm_oversize_mask(&mut self) -> Ch0Buf1FrmOversizeMaskW<'_, RegUvcIntrSpec> {
        Ch0Buf1FrmOversizeMaskW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ch0_buf0_header_err_mask(&mut self) -> Ch0Buf0HeaderErrMaskW<'_, RegUvcIntrSpec> {
        Ch0Buf0HeaderErrMaskW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ch0_buf1_header_err_mask(&mut self) -> Ch0Buf1HeaderErrMaskW<'_, RegUvcIntrSpec> {
        Ch0Buf1HeaderErrMaskW::new(self, 21)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ch1_buf0_frm_done_mask(&mut self) -> Ch1Buf0FrmDoneMaskW<'_, RegUvcIntrSpec> {
        Ch1Buf0FrmDoneMaskW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ch1_buf1_frm_done_mask(&mut self) -> Ch1Buf1FrmDoneMaskW<'_, RegUvcIntrSpec> {
        Ch1Buf1FrmDoneMaskW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ch1_buf0_frm_oversize_mask(&mut self) -> Ch1Buf0FrmOversizeMaskW<'_, RegUvcIntrSpec> {
        Ch1Buf0FrmOversizeMaskW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ch1_buf1_frm_oversize_mask(&mut self) -> Ch1Buf1FrmOversizeMaskW<'_, RegUvcIntrSpec> {
        Ch1Buf1FrmOversizeMaskW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ch1_buf0_header_err_mask(&mut self) -> Ch1Buf0HeaderErrMaskW<'_, RegUvcIntrSpec> {
        Ch1Buf0HeaderErrMaskW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ch1_buf1_header_err_mask(&mut self) -> Ch1Buf1HeaderErrMaskW<'_, RegUvcIntrSpec> {
        Ch1Buf1HeaderErrMaskW::new(self, 29)
    }
}
#[doc = "Interrupt Mask and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_uvc_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_uvc_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUvcIntrSpec;
impl crate::RegisterSpec for RegUvcIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_uvc_intr::R`](R) reader structure"]
impl crate::Readable for RegUvcIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_uvc_intr::W`](W) writer structure"]
impl crate::Writable for RegUvcIntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UVC_INTR to value 0"]
impl crate::Resettable for RegUvcIntrSpec {}
