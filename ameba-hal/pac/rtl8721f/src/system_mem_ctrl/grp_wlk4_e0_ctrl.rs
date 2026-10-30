#[doc = "Register `GRP_WLK4_E0_CTRL` reader"]
pub type R = crate::R<GrpWlk4E0CtrlSpec>;
#[doc = "Register `GRP_WLK4_E0_CTRL` writer"]
pub type W = crate::W<GrpWlk4E0CtrlSpec>;
#[doc = "Field `grp_wlk4_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWlk4E0LoopModeR = crate::BitReader;
#[doc = "Field `grp_wlk4_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWlk4E0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_wlk4_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWlk4E0MselR = crate::FieldReader;
#[doc = "Field `grp_wlk4_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWlk4E0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wlk4_e0_dvse` reader - DVSE"]
pub type Wlk4E0DvseR = crate::BitReader;
#[doc = "Field `wlk4_e0_dvse` writer - DVSE"]
pub type Wlk4E0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wlk4_e0_dvs` reader - DVS"]
pub type Wlk4E0DvsR = crate::FieldReader;
#[doc = "Field `wlk4_e0_dvs` writer - DVS"]
pub type Wlk4E0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wlk4_e0_tmck` reader - TMCK"]
pub type Wlk4E0TmckR = crate::BitReader;
#[doc = "Field `wlk4_e0_tmck` writer - TMCK"]
pub type Wlk4E0TmckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wlk4_e0_loop_mode(&self) -> GrpWlk4E0LoopModeR {
        GrpWlk4E0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wlk4_e0_msel(&self) -> GrpWlk4E0MselR {
        GrpWlk4E0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn wlk4_e0_dvse(&self) -> Wlk4E0DvseR {
        Wlk4E0DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn wlk4_e0_dvs(&self) -> Wlk4E0DvsR {
        Wlk4E0DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn wlk4_e0_tmck(&self) -> Wlk4E0TmckR {
        Wlk4E0TmckR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wlk4_e0_loop_mode(&mut self) -> GrpWlk4E0LoopModeW<'_, GrpWlk4E0CtrlSpec> {
        GrpWlk4E0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wlk4_e0_msel(&mut self) -> GrpWlk4E0MselW<'_, GrpWlk4E0CtrlSpec> {
        GrpWlk4E0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn wlk4_e0_dvse(&mut self) -> Wlk4E0DvseW<'_, GrpWlk4E0CtrlSpec> {
        Wlk4E0DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn wlk4_e0_dvs(&mut self) -> Wlk4E0DvsW<'_, GrpWlk4E0CtrlSpec> {
        Wlk4E0DvsW::new(self, 11)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn wlk4_e0_tmck(&mut self) -> Wlk4E0TmckW<'_, GrpWlk4E0CtrlSpec> {
        Wlk4E0TmckW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_wlk4_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_wlk4_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpWlk4E0CtrlSpec;
impl crate::RegisterSpec for GrpWlk4E0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_wlk4_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpWlk4E0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_wlk4_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpWlk4E0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_WLK4_E0_CTRL to value 0x2b00"]
impl crate::Resettable for GrpWlk4E0CtrlSpec {
    const RESET_VALUE: u32 = 0x2b00;
}
