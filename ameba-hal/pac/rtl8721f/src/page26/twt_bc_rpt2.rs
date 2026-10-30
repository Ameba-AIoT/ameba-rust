#[doc = "Register `TWT_BC_RPT2` reader"]
pub type R = crate::R<TwtBcRpt2Spec>;
#[doc = "Register `TWT_BC_RPT2` writer"]
pub type W = crate::W<TwtBcRpt2Spec>;
#[doc = "Field `twt_rpt_3` reader - specified twt id report"]
pub type TwtRpt3R = crate::FieldReader<u16>;
#[doc = "Field `twt_rpt_3` writer - specified twt id report"]
pub type TwtRpt3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `twt_rpt_4` reader - specified twt id report"]
pub type TwtRpt4R = crate::FieldReader<u16>;
#[doc = "Field `twt_rpt_4` writer - specified twt id report"]
pub type TwtRpt4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_3(&self) -> TwtRpt3R {
        TwtRpt3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_4(&self) -> TwtRpt4R {
        TwtRpt4R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_3(&mut self) -> TwtRpt3W<'_, TwtBcRpt2Spec> {
        TwtRpt3W::new(self, 0)
    }
    #[doc = "Bits 16:31 - specified twt id report"]
    #[inline(always)]
    pub fn twt_rpt_4(&mut self) -> TwtRpt4W<'_, TwtBcRpt2Spec> {
        TwtRpt4W::new(self, 16)
    }
}
#[doc = "TWT IE ID RPT2\n\nYou can [`read`](crate::Reg::read) this register and get [`twt_bc_rpt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twt_bc_rpt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwtBcRpt2Spec;
impl crate::RegisterSpec for TwtBcRpt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twt_bc_rpt2::R`](R) reader structure"]
impl crate::Readable for TwtBcRpt2Spec {}
#[doc = "`write(|w| ..)` method takes [`twt_bc_rpt2::W`](W) writer structure"]
impl crate::Writable for TwtBcRpt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TWT_BC_RPT2 to value 0"]
impl crate::Resettable for TwtBcRpt2Spec {}
