#[doc = "Register `INT_R_REG` reader"]
pub type R = crate::R<IntRRegSpec>;
#[doc = "Register `INT_R_REG` writer"]
pub type W = crate::W<IntRRegSpec>;
#[doc = "Field `rr1_int_routing` reader - "]
pub type Rr1IntRoutingR = crate::BitReader;
#[doc = "Field `rr1_int_routing` writer - "]
pub type Rr1IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rr2_int_routing` reader - "]
pub type Rr2IntRoutingR = crate::BitReader;
#[doc = "Field `rr2_int_routing` writer - "]
pub type Rr2IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rr3_int_routing` reader - "]
pub type Rr3IntRoutingR = crate::BitReader;
#[doc = "Field `rr3_int_routing` writer - "]
pub type Rr3IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rr4_int_routing` reader - "]
pub type Rr4IntRoutingR = crate::BitReader;
#[doc = "Field `rr4_int_routing` writer - "]
pub type Rr4IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rr5_int_routing` reader - "]
pub type Rr5IntRoutingR = crate::BitReader;
#[doc = "Field `rr5_int_routing` writer - "]
pub type Rr5IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rr6_int_routing` reader - "]
pub type Rr6IntRoutingR = crate::BitReader;
#[doc = "Field `rr6_int_routing` writer - "]
pub type Rr6IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tr1_int_routing` reader - "]
pub type Tr1IntRoutingR = crate::BitReader;
#[doc = "Field `tr1_int_routing` writer - "]
pub type Tr1IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tr2_int_routing` reader - "]
pub type Tr2IntRoutingR = crate::BitReader;
#[doc = "Field `tr2_int_routing` writer - "]
pub type Tr2IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tr3_int_routing` reader - "]
pub type Tr3IntRoutingR = crate::BitReader;
#[doc = "Field `tr3_int_routing` writer - "]
pub type Tr3IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tr4_int_routing` reader - "]
pub type Tr4IntRoutingR = crate::BitReader;
#[doc = "Field `tr4_int_routing` writer - "]
pub type Tr4IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tr5_int_routing` reader - "]
pub type Tr5IntRoutingR = crate::BitReader;
#[doc = "Field `tr5_int_routing` writer - "]
pub type Tr5IntRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rr1_int_routing(&self) -> Rr1IntRoutingR {
        Rr1IntRoutingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rr2_int_routing(&self) -> Rr2IntRoutingR {
        Rr2IntRoutingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rr3_int_routing(&self) -> Rr3IntRoutingR {
        Rr3IntRoutingR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rr4_int_routing(&self) -> Rr4IntRoutingR {
        Rr4IntRoutingR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rr5_int_routing(&self) -> Rr5IntRoutingR {
        Rr5IntRoutingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rr6_int_routing(&self) -> Rr6IntRoutingR {
        Rr6IntRoutingR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tr1_int_routing(&self) -> Tr1IntRoutingR {
        Tr1IntRoutingR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tr2_int_routing(&self) -> Tr2IntRoutingR {
        Tr2IntRoutingR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tr3_int_routing(&self) -> Tr3IntRoutingR {
        Tr3IntRoutingR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tr4_int_routing(&self) -> Tr4IntRoutingR {
        Tr4IntRoutingR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tr5_int_routing(&self) -> Tr5IntRoutingR {
        Tr5IntRoutingR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rr1_int_routing(&mut self) -> Rr1IntRoutingW<'_, IntRRegSpec> {
        Rr1IntRoutingW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rr2_int_routing(&mut self) -> Rr2IntRoutingW<'_, IntRRegSpec> {
        Rr2IntRoutingW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rr3_int_routing(&mut self) -> Rr3IntRoutingW<'_, IntRRegSpec> {
        Rr3IntRoutingW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rr4_int_routing(&mut self) -> Rr4IntRoutingW<'_, IntRRegSpec> {
        Rr4IntRoutingW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rr5_int_routing(&mut self) -> Rr5IntRoutingW<'_, IntRRegSpec> {
        Rr5IntRoutingW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rr6_int_routing(&mut self) -> Rr6IntRoutingW<'_, IntRRegSpec> {
        Rr6IntRoutingW::new(self, 10)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tr1_int_routing(&mut self) -> Tr1IntRoutingW<'_, IntRRegSpec> {
        Tr1IntRoutingW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tr2_int_routing(&mut self) -> Tr2IntRoutingW<'_, IntRRegSpec> {
        Tr2IntRoutingW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tr3_int_routing(&mut self) -> Tr3IntRoutingW<'_, IntRRegSpec> {
        Tr3IntRoutingW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tr4_int_routing(&mut self) -> Tr4IntRoutingW<'_, IntRRegSpec> {
        Tr4IntRoutingW::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tr5_int_routing(&mut self) -> Tr5IntRoutingW<'_, IntRRegSpec> {
        Tr5IntRoutingW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_r_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_r_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRRegSpec;
impl crate::RegisterSpec for IntRRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_r_reg::R`](R) reader structure"]
impl crate::Readable for IntRRegSpec {}
#[doc = "`write(|w| ..)` method takes [`int_r_reg::W`](W) writer structure"]
impl crate::Writable for IntRRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_R_REG to value 0"]
impl crate::Resettable for IntRRegSpec {}
