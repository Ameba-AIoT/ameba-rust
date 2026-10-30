#[doc = "Register `TWT_BC_RPT1` reader"]
pub type R = crate::R<TwtBcRpt1Spec>;
#[doc = "Register `TWT_BC_RPT1` writer"]
pub type W = crate::W<TwtBcRpt1Spec>;
#[doc = "Field `twt_rpt_1` reader - specified twt id report"]
pub type TwtRpt1R = crate::FieldReader<u16>;
#[doc = "Field `twt_rpt_1` writer - specified twt id report"]
pub type TwtRpt1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `twt_rpt_2` reader - specified twt id report"]
pub type TwtRpt2R = crate::FieldReader<u16>;
#[doc = "Field `twt_rpt_2` writer - specified twt id report"]
pub type TwtRpt2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_1(&self) -> TwtRpt1R {
        TwtRpt1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_2(&self) -> TwtRpt2R {
        TwtRpt2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_1(&mut self) -> TwtRpt1W<'_, TwtBcRpt1Spec> {
        TwtRpt1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_2(&mut self) -> TwtRpt2W<'_, TwtBcRpt1Spec> {
        TwtRpt2W::new(self, 16)
    }
}
#[doc = "TWT IE ID RPT1\n\nYou can [`read`](crate::Reg::read) this register and get [`twt_bc_rpt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twt_bc_rpt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwtBcRpt1Spec;
impl crate::RegisterSpec for TwtBcRpt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twt_bc_rpt1::R`](R) reader structure"]
impl crate::Readable for TwtBcRpt1Spec {}
#[doc = "`write(|w| ..)` method takes [`twt_bc_rpt1::W`](W) writer structure"]
impl crate::Writable for TwtBcRpt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TWT_BC_RPT1 to value 0"]
impl crate::Resettable for TwtBcRpt1Spec {}
