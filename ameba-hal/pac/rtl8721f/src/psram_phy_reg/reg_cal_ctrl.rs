#[doc = "Register `REG_CAL_CTRL` reader"]
pub type R = crate::R<RegCalCtrlSpec>;
#[doc = "Register `REG_CAL_CTRL` writer"]
pub type W = crate::W<RegCalCtrlSpec>;
#[doc = "Field `CFG_CAL_EN` reader - Hardware auto calibration enable , if disable , N,J will be fixed ."]
pub type CfgCalEnR = crate::BitReader;
#[doc = "Field `CFG_CAL_EN` writer - Hardware auto calibration enable , if disable , N,J will be fixed ."]
pub type CfgCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFG_CAL_INTR_MASK` reader - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
pub type CfgCalIntrMaskR = crate::FieldReader;
#[doc = "Field `CFG_CAL_INTR_MASK` writer - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
pub type CfgCalIntrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CFG_CAL_ERR_ALMOST` reader - The current burst receive correct , but next burst will set J = Jmin . Cleared by write 1 ."]
pub type CfgCalErrAlmostR = crate::BitReader;
#[doc = "Field `CFG_CAL_ERR_ALREADY` reader - The Pre N/J may cause current burst receive error , even change J to Jmin , the error still remain . Cleared by write 1 ."]
pub type CfgCalErrAlreadyR = crate::BitReader;
#[doc = "Field `CFG_CAL_ERR_UNSURE` reader - The Pre N/J may cause current burst receive error , but it is not sure because the burst len is too small. Cleared by write 1 ."]
pub type CfgCalErrUnsureR = crate::BitReader;
#[doc = "Field `CFG_CAL_NBDR_ALMOST` reader - The N-J or N+J reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
pub type CfgCalNbdrAlmostR = crate::BitReader;
#[doc = "Field `CFG_CAL_NBDR_OVF` reader - The N reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
pub type CfgCalNbdrOvfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Hardware auto calibration enable , if disable , N,J will be fixed ."]
    #[inline(always)]
    pub fn cfg_cal_en(&self) -> CfgCalEnR {
        CfgCalEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
    #[inline(always)]
    pub fn cfg_cal_intr_mask(&self) -> CfgCalIntrMaskR {
        CfgCalIntrMaskR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - The current burst receive correct , but next burst will set J = Jmin . Cleared by write 1 ."]
    #[inline(always)]
    pub fn cfg_cal_err_almost(&self) -> CfgCalErrAlmostR {
        CfgCalErrAlmostR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The Pre N/J may cause current burst receive error , even change J to Jmin , the error still remain . Cleared by write 1 ."]
    #[inline(always)]
    pub fn cfg_cal_err_already(&self) -> CfgCalErrAlreadyR {
        CfgCalErrAlreadyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The Pre N/J may cause current burst receive error , but it is not sure because the burst len is too small. Cleared by write 1 ."]
    #[inline(always)]
    pub fn cfg_cal_err_unsure(&self) -> CfgCalErrUnsureR {
        CfgCalErrUnsureR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The N-J or N+J reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
    #[inline(always)]
    pub fn cfg_cal_nbdr_almost(&self) -> CfgCalNbdrAlmostR {
        CfgCalNbdrAlmostR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The N reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
    #[inline(always)]
    pub fn cfg_cal_nbdr_ovf(&self) -> CfgCalNbdrOvfR {
        CfgCalNbdrOvfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware auto calibration enable , if disable , N,J will be fixed ."]
    #[inline(always)]
    pub fn cfg_cal_en(&mut self) -> CfgCalEnW<'_, RegCalCtrlSpec> {
        CfgCalEnW::new(self, 0)
    }
    #[doc = "Bits 4:8 - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
    #[inline(always)]
    pub fn cfg_cal_intr_mask(&mut self) -> CfgCalIntrMaskW<'_, RegCalCtrlSpec> {
        CfgCalIntrMaskW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cal_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cal_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCalCtrlSpec;
impl crate::RegisterSpec for RegCalCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cal_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCalCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cal_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCalCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAL_CTRL to value 0x31"]
impl crate::Resettable for RegCalCtrlSpec {
    const RESET_VALUE: u32 = 0x31;
}
