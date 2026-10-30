#[doc = "Register `GRP_WPOFF_E5_CTRL` reader"]
pub type R = crate::R<GrpWpoffE5CtrlSpec>;
#[doc = "Register `GRP_WPOFF_E5_CTRL` writer"]
pub type W = crate::W<GrpWpoffE5CtrlSpec>;
#[doc = "Field `grp_wpoff_e5_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE5LoopModeR = crate::BitReader;
#[doc = "Field `grp_wpoff_e5_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWpoffE5LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_wpoff_e5_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE5MselR = crate::FieldReader;
#[doc = "Field `grp_wpoff_e5_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWpoffE5MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wpoff_e5_rmea` reader - RMEA"]
pub type WpoffE5RmeaR = crate::BitReader;
#[doc = "Field `wpoff_e5_rmea` writer - RMEA"]
pub type WpoffE5RmeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e5_rmeb` reader - RMEB"]
pub type WpoffE5RmebR = crate::BitReader;
#[doc = "Field `wpoff_e5_rmeb` writer - RMEB"]
pub type WpoffE5RmebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e5_rma` reader - RMA"]
pub type WpoffE5RmaR = crate::FieldReader;
#[doc = "Field `wpoff_e5_rma` writer - RMA"]
pub type WpoffE5RmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpoff_e5_rmb` reader - RMB"]
pub type WpoffE5RmbR = crate::FieldReader;
#[doc = "Field `wpoff_e5_rmb` writer - RMB"]
pub type WpoffE5RmbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpoff_e5_test1a` reader - TEST1A"]
pub type WpoffE5Test1aR = crate::BitReader;
#[doc = "Field `wpoff_e5_test1a` writer - TEST1A"]
pub type WpoffE5Test1aW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpoff_e5_test1b` reader - TEST1B"]
pub type WpoffE5Test1bR = crate::BitReader;
#[doc = "Field `wpoff_e5_test1b` writer - TEST1B"]
pub type WpoffE5Test1bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e5_loop_mode(&self) -> GrpWpoffE5LoopModeR {
        GrpWpoffE5LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e5_msel(&self) -> GrpWpoffE5MselR {
        GrpWpoffE5MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn wpoff_e5_rmea(&self) -> WpoffE5RmeaR {
        WpoffE5RmeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn wpoff_e5_rmeb(&self) -> WpoffE5RmebR {
        WpoffE5RmebR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn wpoff_e5_rma(&self) -> WpoffE5RmaR {
        WpoffE5RmaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn wpoff_e5_rmb(&self) -> WpoffE5RmbR {
        WpoffE5RmbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn wpoff_e5_test1a(&self) -> WpoffE5Test1aR {
        WpoffE5Test1aR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn wpoff_e5_test1b(&self) -> WpoffE5Test1bR {
        WpoffE5Test1bR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpoff_e5_loop_mode(&mut self) -> GrpWpoffE5LoopModeW<'_, GrpWpoffE5CtrlSpec> {
        GrpWpoffE5LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpoff_e5_msel(&mut self) -> GrpWpoffE5MselW<'_, GrpWpoffE5CtrlSpec> {
        GrpWpoffE5MselW::new(self, 8)
    }
    #[doc = "Bit 10 - RMEA"]
    #[inline(always)]
    pub fn wpoff_e5_rmea(&mut self) -> WpoffE5RmeaW<'_, GrpWpoffE5CtrlSpec> {
        WpoffE5RmeaW::new(self, 10)
    }
    #[doc = "Bit 11 - RMEB"]
    #[inline(always)]
    pub fn wpoff_e5_rmeb(&mut self) -> WpoffE5RmebW<'_, GrpWpoffE5CtrlSpec> {
        WpoffE5RmebW::new(self, 11)
    }
    #[doc = "Bits 12:15 - RMA"]
    #[inline(always)]
    pub fn wpoff_e5_rma(&mut self) -> WpoffE5RmaW<'_, GrpWpoffE5CtrlSpec> {
        WpoffE5RmaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - RMB"]
    #[inline(always)]
    pub fn wpoff_e5_rmb(&mut self) -> WpoffE5RmbW<'_, GrpWpoffE5CtrlSpec> {
        WpoffE5RmbW::new(self, 16)
    }
    #[doc = "Bit 20 - TEST1A"]
    #[inline(always)]
    pub fn wpoff_e5_test1a(&mut self) -> WpoffE5Test1aW<'_, GrpWpoffE5CtrlSpec> {
        WpoffE5Test1aW::new(self, 20)
    }
    #[doc = "Bit 21 - TEST1B"]
    #[inline(always)]
    pub fn wpoff_e5_test1b(&mut self) -> WpoffE5Test1bW<'_, GrpWpoffE5CtrlSpec> {
        WpoffE5Test1bW::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_wpoff_e5_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_wpoff_e5_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpWpoffE5CtrlSpec;
impl crate::RegisterSpec for GrpWpoffE5CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_wpoff_e5_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpWpoffE5CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_wpoff_e5_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpWpoffE5CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_WPOFF_E5_CTRL to value 0x0002_2300"]
impl crate::Resettable for GrpWpoffE5CtrlSpec {
    const RESET_VALUE: u32 = 0x0002_2300;
}
