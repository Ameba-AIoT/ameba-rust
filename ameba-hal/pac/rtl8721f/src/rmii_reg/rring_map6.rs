#[doc = "Register `RRING_MAP6` reader"]
pub type R = crate::R<RringMap6Spec>;
#[doc = "Register `RRING_MAP6` writer"]
pub type W = crate::W<RringMap6Spec>;
#[doc = "Field `Ext6_Pri0_route` reader - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri0RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri0_route` writer - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri0RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext6_Pri1_route` reader - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri1RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri1_route` writer - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri1RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext6_Pri2_route` reader - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri2RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri2_route` writer - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri2RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext6_Pri3_route` reader - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri3RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri3_route` writer - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri3RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext6_Pri4_route` reader - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri4RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri4_route` writer - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri4RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext6_Pri5_route` reader - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri5RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri5_route` writer - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri5RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext6_Pri6_route` reader - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri6RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri6_route` writer - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri6RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Ext6_Pri7_route` reader - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri7RouteR = crate::FieldReader;
#[doc = "Field `Ext6_Pri7_route` writer - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
pub type Ext6Pri7RouteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri0_route(&self) -> Ext6Pri0RouteR {
        Ext6Pri0RouteR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri1_route(&self) -> Ext6Pri1RouteR {
        Ext6Pri1RouteR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri2_route(&self) -> Ext6Pri2RouteR {
        Ext6Pri2RouteR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri3_route(&self) -> Ext6Pri3RouteR {
        Ext6Pri3RouteR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri4_route(&self) -> Ext6Pri4RouteR {
        Ext6Pri4RouteR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri5_route(&self) -> Ext6Pri5RouteR {
        Ext6Pri5RouteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri6_route(&self) -> Ext6Pri6RouteR {
        Ext6Pri6RouteR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri7_route(&self) -> Ext6Pri7RouteR {
        Ext6Pri7RouteR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cputag.priority = 0 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri0_route(&mut self) -> Ext6Pri0RouteW<'_, RringMap6Spec> {
        Ext6Pri0RouteW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Cputag.priority = 1 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri1_route(&mut self) -> Ext6Pri1RouteW<'_, RringMap6Spec> {
        Ext6Pri1RouteW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Cputag.priority = 2 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri2_route(&mut self) -> Ext6Pri2RouteW<'_, RringMap6Spec> {
        Ext6Pri2RouteW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Cputag.priority = 3 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri3_route(&mut self) -> Ext6Pri3RouteW<'_, RringMap6Spec> {
        Ext6Pri3RouteW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Cputag.priority = 4 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri4_route(&mut self) -> Ext6Pri4RouteW<'_, RringMap6Spec> {
        Ext6Pri4RouteW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Cputag.priority = 5 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri5_route(&mut self) -> Ext6Pri5RouteW<'_, RringMap6Spec> {
        Ext6Pri5RouteW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Cputag.priority = 6 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri6_route(&mut self) -> Ext6Pri6RouteW<'_, RringMap6Spec> {
        Ext6Pri6RouteW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Cputag.priority = 7 route select. {0,1,7} ring1, {2} ring2, {3} ring3,..."]
    #[inline(always)]
    pub fn ext6_pri7_route(&mut self) -> Ext6Pri7RouteW<'_, RringMap6Spec> {
        Ext6Pri7RouteW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rring_map6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rring_map6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RringMap6Spec;
impl crate::RegisterSpec for RringMap6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rring_map6::R`](R) reader structure"]
impl crate::Readable for RringMap6Spec {}
#[doc = "`write(|w| ..)` method takes [`rring_map6::W`](W) writer structure"]
impl crate::Writable for RringMap6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRING_MAP6 to value 0"]
impl crate::Resettable for RringMap6Spec {}
