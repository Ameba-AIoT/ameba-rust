#[doc = "Register `GRP_WPOFF_E3_CTRL` reader"]
pub type R = crate::R<GrpWpoffE3CtrlSpec>;
#[doc = "Register `GRP_WPOFF_E3_CTRL` writer"]
pub type W = crate::W<GrpWpoffE3CtrlSpec>;
#[doc = "Field `grp_wpoff_e3_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE3LoopModeR = crate::BitReader;
#[doc = "Field `grp_wpoff_e3_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE3LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_wpoff_e3_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE3MselR = crate::FieldReader;
#[doc = "Field `grp_wpoff_e3_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE3MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wpoff_e3_rme` reader - RME"]
pub type WpoffE3RmeR = crate::BitReader;
#[doc = "Field `wpoff_e3_rme` writer - RME"]
pub type WpoffE3RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e3_rm` reader - RM"]
pub type WpoffE3RmR = crate::FieldReader;
#[doc = "Field `wpoff_e3_rm` writer - RM"]
pub type WpoffE3RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpoff_e3_test1` reader - TEST1"]
pub type WpoffE3Test1R = crate::BitReader;
#[doc = "Field `wpoff_e3_test1` writer - TEST1"]
pub type WpoffE3Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e3_loop_mode(&self) -> GrpWpoffE3LoopModeR {
        GrpWpoffE3LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e3_msel(&self) -> GrpWpoffE3MselR {
        GrpWpoffE3MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn wpoff_e3_rme(&self) -> WpoffE3RmeR {
        WpoffE3RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn wpoff_e3_rm(&self) -> WpoffE3RmR {
        WpoffE3RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn wpoff_e3_test1(&self) -> WpoffE3Test1R {
        WpoffE3Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e3_loop_mode(&mut self) -> GrpWpoffE3LoopModeW<'_, GrpWpoffE3CtrlSpec> {
        GrpWpoffE3LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e3_msel(&mut self) -> GrpWpoffE3MselW<'_, GrpWpoffE3CtrlSpec> {
        GrpWpoffE3MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn wpoff_e3_rme(&mut self) -> WpoffE3RmeW<'_, GrpWpoffE3CtrlSpec> {
        WpoffE3RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn wpoff_e3_rm(&mut self) -> WpoffE3RmW<'_, GrpWpoffE3CtrlSpec> {
        WpoffE3RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn wpoff_e3_test1(&mut self) -> WpoffE3Test1W<'_, GrpWpoffE3CtrlSpec> {
        WpoffE3Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_wpoff_e3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_wpoff_e3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpWpoffE3CtrlSpec;
impl crate::RegisterSpec for GrpWpoffE3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_wpoff_e3_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpWpoffE3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_wpoff_e3_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpWpoffE3CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_WPOFF_E3_CTRL to value 0x1300"]
impl crate::Resettable for GrpWpoffE3CtrlSpec {
    const RESET_VALUE: u32 = 0x1300;
}
