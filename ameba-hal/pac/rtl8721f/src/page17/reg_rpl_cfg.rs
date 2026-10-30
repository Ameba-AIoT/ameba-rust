#[doc = "Register `REG_RPL_CFG` reader"]
pub type R = crate::R<RegRplCfgSpec>;
#[doc = "Register `REG_RPL_CFG` writer"]
pub type W = crate::W<RegRplCfgSpec>;
#[doc = "Field `r_rplcfg_en` reader - enable use r_rplcfg_dbm as rx power level"]
pub type RRplcfgEnR = crate::BitReader;
#[doc = "Field `r_rplcfg_en` writer - enable use r_rplcfg_dbm as rx power level"]
pub type RRplcfgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rplcfg_dbm` reader - rx power level"]
pub type RRplcfgDbmR = crate::FieldReader<u16>;
#[doc = "Field `r_rplcfg_dbm` writer - rx power level"]
pub type RRplcfgDbmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_rplcfg_upper_en` reader - enable use r_rplcfg_upper_dbm as rx power level"]
pub type RRplcfgUpperEnR = crate::BitReader;
#[doc = "Field `r_rplcfg_upper_en` writer - enable use r_rplcfg_upper_dbm as rx power level"]
pub type RRplcfgUpperEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_rplcfg_upper_dbm` reader - rx power upper level"]
pub type RRplcfgUpperDbmR = crate::FieldReader<u16>;
#[doc = "Field `r_rplcfg_upper_dbm` writer - rx power upper level"]
pub type RRplcfgUpperDbmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - enable use r_rplcfg_dbm as rx power level"]
    #[inline(always)]
    pub fn r_rplcfg_en(&self) -> RRplcfgEnR {
        RRplcfgEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:9 - rx power level"]
    #[inline(always)]
    pub fn r_rplcfg_dbm(&self) -> RRplcfgDbmR {
        RRplcfgDbmR::new(((self.bits >> 1) & 0x01ff) as u16)
    }
    #[doc = "Bit 10 - enable use r_rplcfg_upper_dbm as rx power level"]
    #[inline(always)]
    pub fn r_rplcfg_upper_en(&self) -> RRplcfgUpperEnR {
        RRplcfgUpperEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:19 - rx power upper level"]
    #[inline(always)]
    pub fn r_rplcfg_upper_dbm(&self) -> RRplcfgUpperDbmR {
        RRplcfgUpperDbmR::new(((self.bits >> 11) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - enable use r_rplcfg_dbm as rx power level"]
    #[inline(always)]
    pub fn r_rplcfg_en(&mut self) -> RRplcfgEnW<'_, RegRplCfgSpec> {
        RRplcfgEnW::new(self, 0)
    }
    #[doc = "Bits 1:9 - rx power level"]
    #[inline(always)]
    pub fn r_rplcfg_dbm(&mut self) -> RRplcfgDbmW<'_, RegRplCfgSpec> {
        RRplcfgDbmW::new(self, 1)
    }
    #[doc = "Bit 10 - enable use r_rplcfg_upper_dbm as rx power level"]
    #[inline(always)]
    pub fn r_rplcfg_upper_en(&mut self) -> RRplcfgUpperEnW<'_, RegRplCfgSpec> {
        RRplcfgUpperEnW::new(self, 10)
    }
    #[doc = "Bits 11:19 - rx power upper level"]
    #[inline(always)]
    pub fn r_rplcfg_upper_dbm(&mut self) -> RRplcfgUpperDbmW<'_, RegRplCfgSpec> {
        RRplcfgUpperDbmW::new(self, 11)
    }
}
#[doc = "rx receive power level configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rpl_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rpl_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRplCfgSpec;
impl crate::RegisterSpec for RegRplCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rpl_cfg::R`](R) reader structure"]
impl crate::Readable for RegRplCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rpl_cfg::W`](W) writer structure"]
impl crate::Writable for RegRplCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RPL_CFG to value 0"]
impl crate::Resettable for RegRplCfgSpec {}
