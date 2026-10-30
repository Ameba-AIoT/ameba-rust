#[doc = "Register `REG_SL1_CAL_PAR` reader"]
pub type R = crate::R<RegSl1CalParSpec>;
#[doc = "Register `REG_SL1_CAL_PAR` writer"]
pub type W = crate::W<RegSl1CalParSpec>;
#[doc = "Field `SL1_CFG_CAL_N` reader - Initial value N calibrated by software ."]
pub type Sl1CfgCalNR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_N` writer - Initial value N calibrated by software ."]
pub type Sl1CfgCalNW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SL1_CFG_CAL_J` reader - Initial value J calibrated by software ."]
pub type Sl1CfgCalJR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_J` writer - Initial value J calibrated by software ."]
pub type Sl1CfgCalJW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SL1_CFG_CAL_JMAX` reader - Maximum J . Hardware may adjust J , but will not exceed this value."]
pub type Sl1CfgCalJmaxR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_JMAX` writer - Maximum J . Hardware may adjust J , but will not exceed this value."]
pub type Sl1CfgCalJmaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SL1_CFG_CAL_JMIN` reader - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
pub type Sl1CfgCalJminR = crate::FieldReader;
#[doc = "Field `SL1_CFG_CAL_JMIN` writer - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
pub type Sl1CfgCalJminW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Initial value N calibrated by software ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_n(&self) -> Sl1CfgCalNR {
        Sl1CfgCalNR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Initial value J calibrated by software ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_j(&self) -> Sl1CfgCalJR {
        Sl1CfgCalJR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Maximum J . Hardware may adjust J , but will not exceed this value."]
    #[inline(always)]
    pub fn sl1_cfg_cal_jmax(&self) -> Sl1CfgCalJmaxR {
        Sl1CfgCalJmaxR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
    #[inline(always)]
    pub fn sl1_cfg_cal_jmin(&self) -> Sl1CfgCalJminR {
        Sl1CfgCalJminR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Initial value N calibrated by software ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_n(&mut self) -> Sl1CfgCalNW<'_, RegSl1CalParSpec> {
        Sl1CfgCalNW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Initial value J calibrated by software ."]
    #[inline(always)]
    pub fn sl1_cfg_cal_j(&mut self) -> Sl1CfgCalJW<'_, RegSl1CalParSpec> {
        Sl1CfgCalJW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Maximum J . Hardware may adjust J , but will not exceed this value."]
    #[inline(always)]
    pub fn sl1_cfg_cal_jmax(&mut self) -> Sl1CfgCalJmaxW<'_, RegSl1CalParSpec> {
        Sl1CfgCalJmaxW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
    #[inline(always)]
    pub fn sl1_cfg_cal_jmin(&mut self) -> Sl1CfgCalJminW<'_, RegSl1CalParSpec> {
        Sl1CfgCalJminW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sl1_cal_par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sl1_cal_par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSl1CalParSpec;
impl crate::RegisterSpec for RegSl1CalParSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sl1_cal_par::R`](R) reader structure"]
impl crate::Readable for RegSl1CalParSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sl1_cal_par::W`](W) writer structure"]
impl crate::Writable for RegSl1CalParSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SL1_CAL_PAR to value 0x020e_0a0f"]
impl crate::Resettable for RegSl1CalParSpec {
    const RESET_VALUE: u32 = 0x020e_0a0f;
}
