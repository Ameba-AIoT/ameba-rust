#[doc = "Register `MBIST_CTRL_ALL` reader"]
pub type R = crate::R<MbistCtrlAllSpec>;
#[doc = "Register `MBIST_CTRL_ALL` writer"]
pub type W = crate::W<MbistCtrlAllSpec>;
#[doc = "Field `mbist_ctrl` reader - Total mbist control \\[0\\] MBIST_CLK_EN_ALL BIS_CLK enable of all system memory groups. \\[1\\] MBIST_RSTB_ALL Reset controll mbist circuit of all system memory groups. \\[3:2\\]MBIST_MSEL_ALL mbist method selection of all memory groups: 2'b01: enable normal mode mbist 2'b10: enable DRF mode mbist 2'b11: enable normal&DRF mode mbist \\[4\\]MBIST_RESUME \\[5\\]MBIST_RESUME_PS \\[6\\]MBIST_DYN_READ_EN If MBIST_MSEL_ALL\\[1\\]=1 & MBIST_DYN_READ_EN=1, run VDDR mode mbist. 7\\]MBIST_PHYCLK_STOP"]
pub type MbistCtrlR = crate::FieldReader;
#[doc = "Field `mbist_ctrl` writer - Total mbist control \\[0\\] MBIST_CLK_EN_ALL BIS_CLK enable of all system memory groups. \\[1\\] MBIST_RSTB_ALL Reset controll mbist circuit of all system memory groups. \\[3:2\\]MBIST_MSEL_ALL mbist method selection of all memory groups: 2'b01: enable normal mode mbist 2'b10: enable DRF mode mbist 2'b11: enable normal&DRF mode mbist \\[4\\]MBIST_RESUME \\[5\\]MBIST_RESUME_PS \\[6\\]MBIST_DYN_READ_EN If MBIST_MSEL_ALL\\[1\\]=1 & MBIST_DYN_READ_EN=1, run VDDR mode mbist. 7\\]MBIST_PHYCLK_STOP"]
pub type MbistCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `mbist_sta` reader - Total mbist status \\[16\\] MBIST DRF_START_PAUSE \\[17\\] MBIST NRML_DONE_ALL \\[18\\] MBIST NRML_FAIL_ALL \\[19\\] MBIST DRF_DONE_ALL \\[20\\] MBIST DRF_FAIL_ALL"]
pub type MbistStaR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Total mbist control \\[0\\] MBIST_CLK_EN_ALL BIS_CLK enable of all system memory groups. \\[1\\] MBIST_RSTB_ALL Reset controll mbist circuit of all system memory groups. \\[3:2\\]MBIST_MSEL_ALL mbist method selection of all memory groups: 2'b01: enable normal mode mbist 2'b10: enable DRF mode mbist 2'b11: enable normal&DRF mode mbist \\[4\\]MBIST_RESUME \\[5\\]MBIST_RESUME_PS \\[6\\]MBIST_DYN_READ_EN If MBIST_MSEL_ALL\\[1\\]=1 & MBIST_DYN_READ_EN=1, run VDDR mode mbist. 7\\]MBIST_PHYCLK_STOP"]
    #[inline(always)]
    pub fn mbist_ctrl(&self) -> MbistCtrlR {
        MbistCtrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Total mbist status \\[16\\] MBIST DRF_START_PAUSE \\[17\\] MBIST NRML_DONE_ALL \\[18\\] MBIST NRML_FAIL_ALL \\[19\\] MBIST DRF_DONE_ALL \\[20\\] MBIST DRF_FAIL_ALL"]
    #[inline(always)]
    pub fn mbist_sta(&self) -> MbistStaR {
        MbistStaR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Total mbist control \\[0\\] MBIST_CLK_EN_ALL BIS_CLK enable of all system memory groups. \\[1\\] MBIST_RSTB_ALL Reset controll mbist circuit of all system memory groups. \\[3:2\\]MBIST_MSEL_ALL mbist method selection of all memory groups: 2'b01: enable normal mode mbist 2'b10: enable DRF mode mbist 2'b11: enable normal&DRF mode mbist \\[4\\]MBIST_RESUME \\[5\\]MBIST_RESUME_PS \\[6\\]MBIST_DYN_READ_EN If MBIST_MSEL_ALL\\[1\\]=1 & MBIST_DYN_READ_EN=1, run VDDR mode mbist. 7\\]MBIST_PHYCLK_STOP"]
    #[inline(always)]
    pub fn mbist_ctrl(&mut self) -> MbistCtrlW<'_, MbistCtrlAllSpec> {
        MbistCtrlW::new(self, 0)
    }
}
#[doc = "MEMx MBIST control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_ctrl_all::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbist_ctrl_all::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbistCtrlAllSpec;
impl crate::RegisterSpec for MbistCtrlAllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbist_ctrl_all::R`](R) reader structure"]
impl crate::Readable for MbistCtrlAllSpec {}
#[doc = "`write(|w| ..)` method takes [`mbist_ctrl_all::W`](W) writer structure"]
impl crate::Writable for MbistCtrlAllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBIST_CTRL_ALL to value 0"]
impl crate::Resettable for MbistCtrlAllSpec {}
