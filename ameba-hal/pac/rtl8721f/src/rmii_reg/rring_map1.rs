#[doc = "Register `RRING_MAP1` reader"]
pub type R = crate::R<RringMap1Spec>;
#[doc = "Register `RRING_MAP1` writer"]
pub type W = crate::W<RringMap1Spec>;
#[doc = "Field `Ext1_Pri0_route` reader - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri0RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri0_route` writer - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri0RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext1_Pri1_route` reader - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri1RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri1_route` writer - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri1RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext1_Pri2_route` reader - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri2RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri2_route` writer - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri2RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext1_Pri3_route` reader - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri3RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri3_route` writer - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri3RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext1_Pri4_route` reader - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri4RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri4_route` writer - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri4RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext1_Pri5_route` reader - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri5RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri5_route` writer - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri5RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext1_Pri6_route` reader - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri6RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri6_route` writer - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri6RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext1_Pri7_route` reader - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri7RouteR = crate::FieldReader;
#[doc = "Field `Ext1_Pri7_route` writer - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext1Pri7RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri0_route(&self) -> Ext1Pri0RouteR {
        Ext1Pri0RouteR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri1_route(&self) -> Ext1Pri1RouteR {
        Ext1Pri1RouteR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri2_route(&self) -> Ext1Pri2RouteR {
        Ext1Pri2RouteR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri3_route(&self) -> Ext1Pri3RouteR {
        Ext1Pri3RouteR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri4_route(&self) -> Ext1Pri4RouteR {
        Ext1Pri4RouteR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri5_route(&self) -> Ext1Pri5RouteR {
        Ext1Pri5RouteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri6_route(&self) -> Ext1Pri6RouteR {
        Ext1Pri6RouteR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri7_route(&self) -> Ext1Pri7RouteR {
        Ext1Pri7RouteR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri0_route(&mut self) -> Ext1Pri0RouteW<'_, RringMap1Spec> {
        Ext1Pri0RouteW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri1_route(&mut self) -> Ext1Pri1RouteW<'_, RringMap1Spec> {
        Ext1Pri1RouteW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri2_route(&mut self) -> Ext1Pri2RouteW<'_, RringMap1Spec> {
        Ext1Pri2RouteW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri3_route(&mut self) -> Ext1Pri3RouteW<'_, RringMap1Spec> {
        Ext1Pri3RouteW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri4_route(&mut self) -> Ext1Pri4RouteW<'_, RringMap1Spec> {
        Ext1Pri4RouteW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri5_route(&mut self) -> Ext1Pri5RouteW<'_, RringMap1Spec> {
        Ext1Pri5RouteW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri6_route(&mut self) -> Ext1Pri6RouteW<'_, RringMap1Spec> {
        Ext1Pri6RouteW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext1_pri7_route(&mut self) -> Ext1Pri7RouteW<'_, RringMap1Spec> {
        Ext1Pri7RouteW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rring_map1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rring_map1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RringMap1Spec;
impl crate::RegisterSpec for RringMap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rring_map1::R`](R) reader structure"]
impl crate::Readable for RringMap1Spec {}
#[doc = "`write(|w| ..)` method takes [`rring_map1::W`](W) writer structure"]
impl crate::Writable for RringMap1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRING_MAP1 to value 0"]
impl crate::Resettable for RringMap1Spec {}
