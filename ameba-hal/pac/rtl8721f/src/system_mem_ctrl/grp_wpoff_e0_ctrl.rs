#[doc = "Register `GRP_WPOFF_E0_CTRL` reader"]
pub type R = crate::R<GrpWpoffE0CtrlSpec>;
#[doc = "Register `GRP_WPOFF_E0_CTRL` writer"]
pub type W = crate::W<GrpWpoffE0CtrlSpec>;
#[doc = "Field `grp_wpoff_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_wpoff_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_wpoff_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE0MselR = crate::FieldReader;
#[doc = "Field `grp_wpoff_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wpoff_e0_rme` reader - RME"]
pub type WpoffE0RmeR = crate::BitReader;
#[doc = "Field `wpoff_e0_rme` writer - RME"]
pub type WpoffE0RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e0_rm` reader - RM"]
pub type WpoffE0RmR = crate::FieldReader;
#[doc = "Field `wpoff_e0_rm` writer - RM"]
pub type WpoffE0RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpoff_e0_test1` reader - TEST1"]
pub type WpoffE0Test1R = crate::BitReader;
#[doc = "Field `wpoff_e0_test1` writer - TEST1"]
pub type WpoffE0Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e0_loop_mode(&self) -> GrpWpoffE0LoopModeR {
        GrpWpoffE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e0_msel(&self) -> GrpWpoffE0MselR {
        GrpWpoffE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn wpoff_e0_rme(&self) -> WpoffE0RmeR {
        WpoffE0RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn wpoff_e0_rm(&self) -> WpoffE0RmR {
        WpoffE0RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn wpoff_e0_test1(&self) -> WpoffE0Test1R {
        WpoffE0Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e0_loop_mode(&mut self) -> GrpWpoffE0LoopModeW<'_, GrpWpoffE0CtrlSpec> {
        GrpWpoffE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e0_msel(&mut self) -> GrpWpoffE0MselW<'_, GrpWpoffE0CtrlSpec> {
        GrpWpoffE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn wpoff_e0_rme(&mut self) -> WpoffE0RmeW<'_, GrpWpoffE0CtrlSpec> {
        WpoffE0RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn wpoff_e0_rm(&mut self) -> WpoffE0RmW<'_, GrpWpoffE0CtrlSpec> {
        WpoffE0RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn wpoff_e0_test1(&mut self) -> WpoffE0Test1W<'_, GrpWpoffE0CtrlSpec> {
        WpoffE0Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_wpoff_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_wpoff_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpWpoffE0CtrlSpec;
impl crate::RegisterSpec for GrpWpoffE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_wpoff_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpWpoffE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_wpoff_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpWpoffE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_WPOFF_E0_CTRL to value 0x1300"]
impl crate::Resettable for GrpWpoffE0CtrlSpec {
    const RESET_VALUE: u32 = 0x1300;
}
