#[doc = "Register `REG_SL1_CAL_CTRL` reader"]
pub type R = crate::R<RegSl1CalCtrlSpec>;
#[doc = "Register `REG_SL1_CAL_CTRL` writer"]
pub type W = crate::W<RegSl1CalCtrlSpec>;
#[doc = "Field `SL1_CFG_CAL_EN` reader - Hardware auto calibration enable , if disable , N,J will be fixed ."]
pub type Sl1CfgCalEnR = crate::BitReader;
#[doc = "Field `SL1_CFG_CAL_EN` writer - Hardware auto calibration enable , if disable , N,J will be fixed ."]
pub type Sl1CfgCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SL1_CFG_CAL_INTR_MASK` reader - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
pub type Sl1CfgCalIntrMaskR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_INTR_MASK` writer - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
pub type Sl1CfgCalIntrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SL1_CFG_CAL_ERR_ALMOST` reader - The current burst receive correct , but next burst will set J = Jmin . Cleared by write 1 ."]
pub type Sl1CfgCalErrAlmostR = crate::BitReader;
#[doc = "Field `SL1_CFG_CAL_ERR_ALREADY` reader - The Pre N/J may cause current burst receive error , even change J to Jmin , the error still remain . Cleared by write 1 ."]
pub type Sl1CfgCalErrAlreadyR = crate::BitReader;
#[doc = "Field `SL1_CFG_CAL_ERR_UNSURE` reader - The Pre N/J may cause current burst receive error , but it is not sure because the burst len is too small. Cleared by write 1 ."]
pub type Sl1CfgCalErrUnsureR = crate::BitReader;
#[doc = "Field `SL1_CFG_CAL_NBDR_ALMOST` reader - The N-J or N+J reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
pub type Sl1CfgCalNbdrAlmostR = crate::BitReader;
#[doc = "Field `SL1_CFG_CAL_NBDR_OVF` reader - The N reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
pub type Sl1CfgCalNbdrOvfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Hardware auto calibration enable , if disable , N,J will be fixed ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_en(&self) -> Sl1CfgCalEnR {
        Sl1CfgCalEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
    #[inline(always)]
    pub fn sl1_cfg_cal_intr_mask(&self) -> Sl1CfgCalIntrMaskR {
        Sl1CfgCalIntrMaskR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - The current burst receive correct , but next burst will set J = Jmin . Cleared by write 1 ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_err_almost(&self) -> Sl1CfgCalErrAlmostR {
        Sl1CfgCalErrAlmostR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The Pre N/J may cause current burst receive error , even change J to Jmin , the error still remain . Cleared by write 1 ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_err_already(&self) -> Sl1CfgCalErrAlreadyR {
        Sl1CfgCalErrAlreadyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The Pre N/J may cause current burst receive error , but it is not sure because the burst len is too small. Cleared by write 1 ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_err_unsure(&self) -> Sl1CfgCalErrUnsureR {
        Sl1CfgCalErrUnsureR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The N-J or N+J reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_nbdr_almost(&self) -> Sl1CfgCalNbdrAlmostR {
        Sl1CfgCalNbdrAlmostR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The N reachs the boundary , which is 0 or Nmax . Cleared by write 1 ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_nbdr_ovf(&self) -> Sl1CfgCalNbdrOvfR {
        Sl1CfgCalNbdrOvfR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware auto calibration enable , if disable , N,J will be fixed ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_en(&mut self) -> Sl1CfgCalEnW<'_, RegSl1CalCtrlSpec> {
        Sl1CfgCalEnW::new(self, 0)
    }
    #[doc = "Bits 4:8 - Calibration fail interrupt mask , every bit control one event above (1: enable , 0: disable) \\[0\\] : err almost \\[1\\] : err already \\[2\\] : err unsure \\[3\\] : nbdr almost \\[4\\] : nbdr ovf"]
    #[inline(always)]
    pub fn sl1_cfg_cal_intr_mask(&mut self) -> Sl1CfgCalIntrMaskW<'_, RegSl1CalCtrlSpec> {
        Sl1CfgCalIntrMaskW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sl1_cal_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sl1_cal_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSl1CalCtrlSpec;
impl crate::RegisterSpec for RegSl1CalCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sl1_cal_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSl1CalCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sl1_cal_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSl1CalCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SL1_CAL_CTRL to value 0x31"]
impl crate::Resettable for RegSl1CalCtrlSpec {
    const RESET_VALUE: u32 = 0x31;
}
