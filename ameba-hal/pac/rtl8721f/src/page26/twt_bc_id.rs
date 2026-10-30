#[doc = "Register `TWT_BC_ID` reader"]
pub type R = crate::R<TwtBcIdSpec>;
#[doc = "Register `TWT_BC_ID` writer"]
pub type W = crate::W<TwtBcIdSpec>;
#[doc = "Field `r_BC_TWT_ID` reader - TWT ID bitmap, each bit corresponding to a ID value"]
pub type RBcTwtIdR = crate::FieldReader<u32>;
#[doc = "Field `r_BC_TWT_ID` writer - TWT ID bitmap, each bit corresponding to a ID value"]
pub type RBcTwtIdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TWT ID bitmap, each bit corresponding to a ID value"]
    #[inline(always)]
    pub fn r_bc_twt_id(&self) -> RBcTwtIdR {
        RBcTwtIdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TWT ID bitmap, each bit corresponding to a ID value"]
    #[inline(always)]
    pub fn r_bc_twt_id(&mut self) -> RBcTwtIdW<'_, TwtBcIdSpec> {
        RBcTwtIdW::new(self, 0)
    }
}
#[doc = "TWT IE ID BITMAP\n\nYou can [`read`](crate::Reg::read) this register and get [`twt_bc_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twt_bc_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TwtBcIdSpec;
impl crate::RegisterSpec for TwtBcIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twt_bc_id::R`](R) reader structure"]
impl crate::Readable for TwtBcIdSpec {}
#[doc = "`write(|w| ..)` method takes [`twt_bc_id::W`](W) writer structure"]
impl crate::Writable for TwtBcIdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TWT_BC_ID to value 0"]
impl crate::Resettable for TwtBcIdSpec {}
