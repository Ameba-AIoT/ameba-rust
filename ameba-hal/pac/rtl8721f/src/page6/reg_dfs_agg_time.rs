#[doc = "Register `REG_DFS_AGG_TIME` reader"]
pub type R = crate::R<RegDfsAggTimeSpec>;
#[doc = "Register `REG_DFS_AGG_TIME` writer"]
pub type W = crate::W<RegDfsAggTimeSpec>;
#[doc = "Field `r_dfs_agg_num` reader - aggregation DFS packet number selection. When reach the target, will inform host by interrupt, unit 4bytes"]
pub type RDfsAggNumR = crate::FieldReader<u16>;
#[doc = "Field `r_dfs_agg_num` writer - aggregation DFS packet number selection. When reach the target, will inform host by interrupt, unit 4bytes"]
pub type RDfsAggNumW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `r_dfs_agg_time` reader - DFS packet aggregation time. When reach the target, will inform host by interrupt, unit ms"]
pub type RDfsAggTimeR = crate::FieldReader;
#[doc = "Field `r_dfs_agg_time` writer - DFS packet aggregation time. When reach the target, will inform host by interrupt, unit ms"]
pub type RDfsAggTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_rxtrig_type` reader - mapping to frame control type and subtype filed, for LA trigger select"]
pub type RRxtrigTypeR = crate::FieldReader;
#[doc = "Field `r_rxtrig_type` writer - mapping to frame control type and subtype filed, for LA trigger select"]
pub type RRxtrigTypeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_txtrig_type` reader - mapping to frame control type and subtype filed, for LA trigger select"]
pub type RTxtrigTypeR = crate::FieldReader;
#[doc = "Field `r_txtrig_type` writer - mapping to frame control type and subtype filed, for LA trigger select"]
pub type RTxtrigTypeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:8 - aggregation DFS packet number selection. When reach the target, will inform host by interrupt, unit 4bytes"]
    #[inline(always)]
    pub fn r_dfs_agg_num(&self) -> RDfsAggNumR {
        RDfsAggNumR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - DFS packet aggregation time. When reach the target, will inform host by interrupt, unit ms"]
    #[inline(always)]
    pub fn r_dfs_agg_time(&self) -> RDfsAggTimeR {
        RDfsAggTimeR::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:22 - mapping to frame control type and subtype filed, for LA trigger select"]
    #[inline(always)]
    pub fn r_rxtrig_type(&self) -> RRxtrigTypeR {
        RRxtrigTypeR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - mapping to frame control type and subtype filed, for LA trigger select"]
    #[inline(always)]
    pub fn r_txtrig_type(&self) -> RTxtrigTypeR {
        RTxtrigTypeR::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - aggregation DFS packet number selection. When reach the target, will inform host by interrupt, unit 4bytes"]
    #[inline(always)]
    pub fn r_dfs_agg_num(&mut self) -> RDfsAggNumW<'_, RegDfsAggTimeSpec> {
        RDfsAggNumW::new(self, 0)
    }
    #[doc = "Bits 9:16 - DFS packet aggregation time. When reach the target, will inform host by interrupt, unit ms"]
    #[inline(always)]
    pub fn r_dfs_agg_time(&mut self) -> RDfsAggTimeW<'_, RegDfsAggTimeSpec> {
        RDfsAggTimeW::new(self, 9)
    }
    #[doc = "Bits 17:22 - mapping to frame control type and subtype filed, for LA trigger select"]
    #[inline(always)]
    pub fn r_rxtrig_type(&mut self) -> RRxtrigTypeW<'_, RegDfsAggTimeSpec> {
        RRxtrigTypeW::new(self, 17)
    }
    #[doc = "Bits 23:28 - mapping to frame control type and subtype filed, for LA trigger select"]
    #[inline(always)]
    pub fn r_txtrig_type(&mut self) -> RTxtrigTypeW<'_, RegDfsAggTimeSpec> {
        RTxtrigTypeW::new(self, 23)
    }
    #[doc = "Bits 29:31 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegDfsAggTimeSpec> {
        DummyW::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dfs_agg_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dfs_agg_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDfsAggTimeSpec;
impl crate::RegisterSpec for RegDfsAggTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dfs_agg_time::R`](R) reader structure"]
impl crate::Readable for RegDfsAggTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dfs_agg_time::W`](W) writer structure"]
impl crate::Writable for RegDfsAggTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DFS_AGG_TIME to value 0"]
impl crate::Resettable for RegDfsAggTimeSpec {}
