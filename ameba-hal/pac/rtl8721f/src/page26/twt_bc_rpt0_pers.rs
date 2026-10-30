#[doc = "Register `TWT_BC_RPT0_PERS` reader"]
pub type R = crate::R<TwtBcRpt0PersSpec>;
#[doc = "Register `TWT_BC_RPT0_PERS` writer"]
pub type W = crate::W<TwtBcRpt0PersSpec>;
#[doc = "Field `twt_rpt_0` reader - specified twt id report"]
pub type TwtRpt0R = crate::FieldReader<u16>;
#[doc = "Field `twt_rpt_0` writer - specified twt id report"]
pub type TwtRpt0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `r_TWT_PERSISTENCE` reader - persistence vlaue, if persistence value in TWT IE is smaller than the reference, bcnpsf will lat the twt id rpt"]
pub type RTwtPersistenceR = crate::FieldReader;
#[doc = "Field `r_TWT_PERSISTENCE` writer - persistence vlaue, if persistence value in TWT IE is smaller than the reference, bcnpsf will lat the twt id rpt"]
pub type RTwtPersistenceW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_WMAC_BCN_TWT_PSR` reader - enable bcn twt bc ie parser"]
pub type RWmacBcnTwtPsrR = crate::BitReader;
#[doc = "Field `r_WMAC_BCN_TWT_PSR` writer - enable bcn twt bc ie parser"]
pub type RWmacBcnTwtPsrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_0(&self) -> TwtRpt0R {
        TwtRpt0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 18:25 - persistence vlaue, if persistence value in TWT IE is smaller than the reference, bcnpsf will lat the twt id rpt"]
    #[inline(always)]
    pub fn r_twt_persistence(&self) -> RTwtPersistenceR {
        RTwtPersistenceR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 26 - enable bcn twt bc ie parser"]
    #[inline(always)]
    pub fn r_wmac_bcn_twt_psr(&self) -> RWmacBcnTwtPsrR {
        RWmacBcnTwtPsrR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_0(&mut self) -> TwtRpt0W<'_, TwtBcRpt0PersSpec> {
        TwtRpt0W::new(self, 0)
    }
    #[doc = "Bits 18:25 - persistence vlaue, if persistence value in TWT IE is smaller than the reference, bcnpsf will lat the twt id rpt"]
    #[inline(always)]
    pub fn r_twt_persistence(&mut self) -> RTwtPersistenceW<'_, TwtBcRpt0PersSpec> {
        RTwtPersistenceW::new(self, 18)
    }
    #[doc = "Bit 26 - enable bcn twt bc ie parser"]
    #[inline(always)]
    pub fn r_wmac_bcn_twt_psr(&mut self) -> RWmacBcnTwtPsrW<'_, TwtBcRpt0PersSpec> {
        RWmacBcnTwtPsrW::new(self, 26)
    }
}
#[doc = "TWT IE persistence reference and RPT0\n\nYou can [`read`](crate::Reg::read) this register and get [`twt_bc_rpt0_pers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twt_bc_rpt0_pers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwtBcRpt0PersSpec;
impl crate::RegisterSpec for TwtBcRpt0PersSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twt_bc_rpt0_pers::R`](R) reader structure"]
impl crate::Readable for TwtBcRpt0PersSpec {}
#[doc = "`write(|w| ..)` method takes [`twt_bc_rpt0_pers::W`](W) writer structure"]
impl crate::Writable for TwtBcRpt0PersSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TWT_BC_RPT0_PERS to value 0"]
impl crate::Resettable for TwtBcRpt0PersSpec {}
