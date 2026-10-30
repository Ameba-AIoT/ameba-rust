#[doc = "Register `GRP_WPOFF_E1_CTRL` reader"]
pub type R = crate::R<GrpWpoffE1CtrlSpec>;
#[doc = "Register `GRP_WPOFF_E1_CTRL` writer"]
pub type W = crate::W<GrpWpoffE1CtrlSpec>;
#[doc = "Field `grp_wpoff_e1_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE1LoopModeR = crate::BitReader;
#[doc = "Field `grp_wpoff_e1_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE1LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_wpoff_e1_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE1MselR = crate::FieldReader;
#[doc = "Field `grp_wpoff_e1_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE1MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wpoff_e1_rme` reader - RME"]
pub type WpoffE1RmeR = crate::BitReader;
#[doc = "Field `wpoff_e1_rme` writer - RME"]
pub type WpoffE1RmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e1_rm` reader - RM"]
pub type WpoffE1RmR = crate::FieldReader;
#[doc = "Field `wpoff_e1_rm` writer - RM"]
pub type WpoffE1RmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpoff_e1_test1` reader - TEST1"]
pub type WpoffE1Test1R = crate::BitReader;
#[doc = "Field `wpoff_e1_test1` writer - TEST1"]
pub type WpoffE1Test1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e1_loop_mode(&self) -> GrpWpoffE1LoopModeR {
        GrpWpoffE1LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e1_msel(&self) -> GrpWpoffE1MselR {
        GrpWpoffE1MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn wpoff_e1_rme(&self) -> WpoffE1RmeR {
        WpoffE1RmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn wpoff_e1_rm(&self) -> WpoffE1RmR {
        WpoffE1RmR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn wpoff_e1_test1(&self) -> WpoffE1Test1R {
        WpoffE1Test1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e1_loop_mode(&mut self) -> GrpWpoffE1LoopModeW<'_, GrpWpoffE1CtrlSpec> {
        GrpWpoffE1LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e1_msel(&mut self) -> GrpWpoffE1MselW<'_, GrpWpoffE1CtrlSpec> {
        GrpWpoffE1MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RME"]
    #[inline(always)]
    pub fn wpoff_e1_rme(&mut self) -> WpoffE1RmeW<'_, GrpWpoffE1CtrlSpec> {
        WpoffE1RmeW::new(self, 10)
    }
    #[doc = "Bits 11:14 - RM"]
    #[inline(always)]
    pub fn wpoff_e1_rm(&mut self) -> WpoffE1RmW<'_, GrpWpoffE1CtrlSpec> {
        WpoffE1RmW::new(self, 11)
    }
    #[doc = "Bit 15 - TEST1"]
    #[inline(always)]
    pub fn wpoff_e1_test1(&mut self) -> WpoffE1Test1W<'_, GrpWpoffE1CtrlSpec> {
        WpoffE1Test1W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_wpoff_e1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_wpoff_e1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpWpoffE1CtrlSpec;
impl crate::RegisterSpec for GrpWpoffE1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_wpoff_e1_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpWpoffE1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_wpoff_e1_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpWpoffE1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_WPOFF_E1_CTRL to value 0x1300"]
impl crate::Resettable for GrpWpoffE1CtrlSpec {
    const RESET_VALUE: u32 = 0x1300;
}
