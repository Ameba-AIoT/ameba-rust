#[doc = "Register `REG_CAL_PAR` reader"]
pub type R = crate::R<RegCalParSpec>;
#[doc = "Register `REG_CAL_PAR` writer"]
pub type W = crate::W<RegCalParSpec>;
#[doc = "Field `CFG_CAL_N` reader - Initial value N calibrated by software ."]
pub type CfgCalNR = crate::FieldReader;
#[doc = "Field `CFG_CAL_N` writer - Initial value N calibrated by software ."]
pub type CfgCalNW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CFG_CAL_J` reader - Initial value J calibrated by software ."]
pub type CfgCalJR = crate::FieldReader;
#[doc = "Field `CFG_CAL_J` writer - Initial value J calibrated by software ."]
pub type CfgCalJW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_CAL_JMAX` reader - Maximum J . Hardware may adjust J , but will not exceed this value."]
pub type CfgCalJmaxR = crate::FieldReader;
#[doc = "Field `CFG_CAL_JMAX` writer - Maximum J . Hardware may adjust J , but will not exceed this value."]
pub type CfgCalJmaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_CAL_JMIN` reader - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
pub type CfgCalJminR = crate::FieldReader;
#[doc = "Field `CFG_CAL_JMIN` writer - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
pub type CfgCalJminW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Initial value N calibrated by software ."]
    #[inline(always)]
    pub fn cfg_cal_n(&self) -> CfgCalNR {
        CfgCalNR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Initial value J calibrated by software ."]
    #[inline(always)]
    pub fn cfg_cal_j(&self) -> CfgCalJR {
        CfgCalJR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Maximum J . Hardware may adjust J , but will not exceed this value."]
    #[inline(always)]
    pub fn cfg_cal_jmax(&self) -> CfgCalJmaxR {
        CfgCalJmaxR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
    #[inline(always)]
    pub fn cfg_cal_jmin(&self) -> CfgCalJminR {
        CfgCalJminR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Initial value N calibrated by software ."]
    #[inline(always)]
    pub fn cfg_cal_n(&mut self) -> CfgCalNW<'_, RegCalParSpec> {
        CfgCalNW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Initial value J calibrated by software ."]
    #[inline(always)]
    pub fn cfg_cal_j(&mut self) -> CfgCalJW<'_, RegCalParSpec> {
        CfgCalJW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Maximum J . Hardware may adjust J , but will not exceed this value."]
    #[inline(always)]
    pub fn cfg_cal_jmax(&mut self) -> CfgCalJmaxW<'_, RegCalParSpec> {
        CfgCalJmaxW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Minimum J . Hardware may adjust J , if J reach this value , an interrup will be raised to alerm software."]
    #[inline(always)]
    pub fn cfg_cal_jmin(&mut self) -> CfgCalJminW<'_, RegCalParSpec> {
        CfgCalJminW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cal_par::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cal_par::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCalParSpec;
impl crate::RegisterSpec for RegCalParSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cal_par::R`](R) reader structure"]
impl crate::Readable for RegCalParSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cal_par::W`](W) writer structure"]
impl crate::Writable for RegCalParSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CAL_PAR to value 0x020e_0a0f"]
impl crate::Resettable for RegCalParSpec {
    const RESET_VALUE: u32 = 0x020e_0a0f;
}
