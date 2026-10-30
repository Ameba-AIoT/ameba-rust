#[doc = "Register `REG_UVC_CTRL` reader"]
pub type R = crate::R<RegUvcCtrlSpec>;
#[doc = "Register `REG_UVC_CTRL` writer"]
pub type W = crate::W<RegUvcCtrlSpec>;
#[doc = "Field `ch0_concat_en` reader - "]
pub type Ch0ConcatEnR = crate::BitReader;
#[doc = "Field `ch0_concat_en` writer - "]
pub type Ch0ConcatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_concat_en` reader - "]
pub type Ch1ConcatEnR = crate::BitReader;
#[doc = "Field `ch1_concat_en` writer - "]
pub type Ch1ConcatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_frm_done_opt` reader - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
pub type Ch0FrmDoneOptR = crate::BitReader;
#[doc = "Field `ch0_frm_done_opt` writer - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
pub type Ch0FrmDoneOptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch1_frm_done_opt` reader - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
pub type Ch1FrmDoneOptR = crate::BitReader;
#[doc = "Field `ch1_frm_done_opt` writer - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
pub type Ch1FrmDoneOptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch0_ep_num` reader - "]
pub type Ch0EpNumR = crate::FieldReader;
#[doc = "Field `ch0_ep_num` writer - "]
pub type Ch0EpNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ch1_ep_num` reader - "]
pub type Ch1EpNumR = crate::FieldReader;
#[doc = "Field `ch1_ep_num` writer - "]
pub type Ch1EpNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_concat_en(&self) -> Ch0ConcatEnR {
        Ch0ConcatEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1_concat_en(&self) -> Ch1ConcatEnR {
        Ch1ConcatEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
    #[inline(always)]
    pub fn ch0_frm_done_opt(&self) -> Ch0FrmDoneOptR {
        Ch0FrmDoneOptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
    #[inline(always)]
    pub fn ch1_frm_done_opt(&self) -> Ch1FrmDoneOptR {
        Ch1FrmDoneOptR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn ch0_ep_num(&self) -> Ch0EpNumR {
        Ch0EpNumR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ch1_ep_num(&self) -> Ch1EpNumR {
        Ch1EpNumR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_concat_en(&mut self) -> Ch0ConcatEnW<'_, RegUvcCtrlSpec> {
        Ch0ConcatEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1_concat_en(&mut self) -> Ch1ConcatEnW<'_, RegUvcCtrlSpec> {
        Ch1ConcatEnW::new(self, 1)
    }
    #[doc = "Bit 4 - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
    #[inline(always)]
    pub fn ch0_frm_done_opt(&mut self) -> Ch0FrmDoneOptW<'_, RegUvcCtrlSpec> {
        Ch0FrmDoneOptW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: frame_done when xfercomp at EOF flag 0: frame_done when frame id changing"]
    #[inline(always)]
    pub fn ch1_frm_done_opt(&mut self) -> Ch1FrmDoneOptW<'_, RegUvcCtrlSpec> {
        Ch1FrmDoneOptW::new(self, 5)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn ch0_ep_num(&mut self) -> Ch0EpNumW<'_, RegUvcCtrlSpec> {
        Ch0EpNumW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ch1_ep_num(&mut self) -> Ch1EpNumW<'_, RegUvcCtrlSpec> {
        Ch1EpNumW::new(self, 20)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_uvc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_uvc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegUvcCtrlSpec;
impl crate::RegisterSpec for RegUvcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_uvc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegUvcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_uvc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegUvcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_UVC_CTRL to value 0"]
impl crate::Resettable for RegUvcCtrlSpec {}
