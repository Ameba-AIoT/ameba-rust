#[doc = "Register `RRING_MAP5` reader"]
pub type R = crate::R<RringMap5Spec>;
#[doc = "Register `RRING_MAP5` writer"]
pub type W = crate::W<RringMap5Spec>;
#[doc = "Field `Ext5_Pri0_route` reader - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri0RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri0_route` writer - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri0RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext5_Pri1_route` reader - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri1RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri1_route` writer - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri1RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext5_Pri2_route` reader - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri2RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri2_route` writer - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri2RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext5_Pri3_route` reader - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri3RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri3_route` writer - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri3RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext5_Pri4_route` reader - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri4RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri4_route` writer - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri4RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext5_Pri5_route` reader - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri5RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri5_route` writer - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri5RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext5_Pri6_route` reader - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri6RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri6_route` writer - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri6RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext5_Pri7_route` reader - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri7RouteR = crate::FieldReader;
#[doc = "Field `Ext5_Pri7_route` writer - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext5Pri7RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri0_route(&self) -> Ext5Pri0RouteR {
        Ext5Pri0RouteR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri1_route(&self) -> Ext5Pri1RouteR {
        Ext5Pri1RouteR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri2_route(&self) -> Ext5Pri2RouteR {
        Ext5Pri2RouteR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri3_route(&self) -> Ext5Pri3RouteR {
        Ext5Pri3RouteR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri4_route(&self) -> Ext5Pri4RouteR {
        Ext5Pri4RouteR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri5_route(&self) -> Ext5Pri5RouteR {
        Ext5Pri5RouteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri6_route(&self) -> Ext5Pri6RouteR {
        Ext5Pri6RouteR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri7_route(&self) -> Ext5Pri7RouteR {
        Ext5Pri7RouteR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri0_route(&mut self) -> Ext5Pri0RouteW<'_, RringMap5Spec> {
        Ext5Pri0RouteW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri1_route(&mut self) -> Ext5Pri1RouteW<'_, RringMap5Spec> {
        Ext5Pri1RouteW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri2_route(&mut self) -> Ext5Pri2RouteW<'_, RringMap5Spec> {
        Ext5Pri2RouteW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri3_route(&mut self) -> Ext5Pri3RouteW<'_, RringMap5Spec> {
        Ext5Pri3RouteW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri4_route(&mut self) -> Ext5Pri4RouteW<'_, RringMap5Spec> {
        Ext5Pri4RouteW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri5_route(&mut self) -> Ext5Pri5RouteW<'_, RringMap5Spec> {
        Ext5Pri5RouteW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri6_route(&mut self) -> Ext5Pri6RouteW<'_, RringMap5Spec> {
        Ext5Pri6RouteW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext5_pri7_route(&mut self) -> Ext5Pri7RouteW<'_, RringMap5Spec> {
        Ext5Pri7RouteW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rring_map5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rring_map5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RringMap5Spec;
impl crate::RegisterSpec for RringMap5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rring_map5::R`](R) reader structure"]
impl crate::Readable for RringMap5Spec {}
#[doc = "`write(|w| ..)` method takes [`rring_map5::W`](W) writer structure"]
impl crate::Writable for RringMap5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRING_MAP5 to value 0"]
impl crate::Resettable for RringMap5Spec {}
