#[doc = "Register `GRP_WPOFF_E2_CTRL` reader"]
pub type R = crate::R<GrpWpoffE2CtrlSpec>;
#[doc = "Register `GRP_WPOFF_E2_CTRL` writer"]
pub type W = crate::W<GrpWpoffE2CtrlSpec>;
#[doc = "Field `grp_wpoff_e2_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE2LoopModeR = crate::BitReader;
#[doc = "Field `grp_wpoff_e2_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE2LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_wpoff_e2_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE2MselR = crate::FieldReader;
#[doc = "Field `grp_wpoff_e2_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE2MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wpoff_e2_rmea` reader - RMEA"]
pub type WpoffE2RmeaR = crate::BitReader;
#[doc = "Field `wpoff_e2_rmea` writer - RMEA"]
pub type WpoffE2RmeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e2_rmeb` reader - RMEB"]
pub type WpoffE2RmebR = crate::BitReader;
#[doc = "Field `wpoff_e2_rmeb` writer - RMEB"]
pub type WpoffE2RmebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e2_rma` reader - RMA"]
pub type WpoffE2RmaR = crate::FieldReader;
#[doc = "Field `wpoff_e2_rma` writer - RMA"]
pub type WpoffE2RmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpoff_e2_rmb` reader - RMB"]
pub type WpoffE2RmbR = crate::FieldReader;
#[doc = "Field `wpoff_e2_rmb` writer - RMB"]
pub type WpoffE2RmbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpoff_e2_test1a` reader - TEST1A"]
pub type WpoffE2Test1aR = crate::BitReader;
#[doc = "Field `wpoff_e2_test1a` writer - TEST1A"]
pub type WpoffE2Test1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e2_test1b` reader - TEST1B"]
pub type WpoffE2Test1bR = crate::BitReader;
#[doc = "Field `wpoff_e2_test1b` writer - TEST1B"]
pub type WpoffE2Test1bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e2_loop_mode(&self) -> GrpWpoffE2LoopModeR {
        GrpWpoffE2LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e2_msel(&self) -> GrpWpoffE2MselR {
        GrpWpoffE2MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn wpoff_e2_rmea(&self) -> WpoffE2RmeaR {
        WpoffE2RmeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn wpoff_e2_rmeb(&self) -> WpoffE2RmebR {
        WpoffE2RmebR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn wpoff_e2_rma(&self) -> WpoffE2RmaR {
        WpoffE2RmaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn wpoff_e2_rmb(&self) -> WpoffE2RmbR {
        WpoffE2RmbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn wpoff_e2_test1a(&self) -> WpoffE2Test1aR {
        WpoffE2Test1aR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn wpoff_e2_test1b(&self) -> WpoffE2Test1bR {
        WpoffE2Test1bR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e2_loop_mode(&mut self) -> GrpWpoffE2LoopModeW<'_, GrpWpoffE2CtrlSpec> {
        GrpWpoffE2LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e2_msel(&mut self) -> GrpWpoffE2MselW<'_, GrpWpoffE2CtrlSpec> {
        GrpWpoffE2MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn wpoff_e2_rmea(&mut self) -> WpoffE2RmeaW<'_, GrpWpoffE2CtrlSpec> {
        WpoffE2RmeaW::new(self, 10)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn wpoff_e2_rmeb(&mut self) -> WpoffE2RmebW<'_, GrpWpoffE2CtrlSpec> {
        WpoffE2RmebW::new(self, 11)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn wpoff_e2_rma(&mut self) -> WpoffE2RmaW<'_, GrpWpoffE2CtrlSpec> {
        WpoffE2RmaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn wpoff_e2_rmb(&mut self) -> WpoffE2RmbW<'_, GrpWpoffE2CtrlSpec> {
        WpoffE2RmbW::new(self, 16)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn wpoff_e2_test1a(&mut self) -> WpoffE2Test1aW<'_, GrpWpoffE2CtrlSpec> {
        WpoffE2Test1aW::new(self, 20)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn wpoff_e2_test1b(&mut self) -> WpoffE2Test1bW<'_, GrpWpoffE2CtrlSpec> {
        WpoffE2Test1bW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_wpoff_e2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_wpoff_e2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpWpoffE2CtrlSpec;
impl crate::RegisterSpec for GrpWpoffE2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_wpoff_e2_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpWpoffE2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_wpoff_e2_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpWpoffE2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_WPOFF_E2_CTRL to value 0x0002_2300"]
impl crate::Resettable for GrpWpoffE2CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_2300;
}
