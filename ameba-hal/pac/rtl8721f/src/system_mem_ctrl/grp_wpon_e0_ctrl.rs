#[doc = "Register `GRP_WPON_E0_CTRL` reader"]
pub type R = crate::R<GrpWponE0CtrlSpec>;
#[doc = "Register `GRP_WPON_E0_CTRL` writer"]
pub type W = crate::W<GrpWponE0CtrlSpec>;
#[doc = "Field `grp_wpon_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWponE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_wpon_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpWponE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_wpon_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWponE0MselR = crate::FieldReader;
#[doc = "Field `grp_wpon_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpWponE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wpon_e0_dvse` reader - DVSE"]
pub type WponE0DvseR = crate::BitReader;
#[doc = "Field `wpon_e0_dvse` writer - DVSE"]
pub type WponE0DvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpon_e0_dvs` reader - DVS"]
pub type WponE0DvsR = crate::FieldReader;
#[doc = "Field `wpon_e0_dvs` writer - DVS"]
pub type WponE0DvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wpon_e0_tmck` reader - TMCK"]
pub type WponE0TmckR = crate::BitReader;
#[doc = "Field `wpon_e0_tmck` writer - TMCK"]
pub type WponE0TmckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpon_e0_rae` reader - RAE"]
pub type WponE0RaeR = crate::BitReader;
#[doc = "Field `wpon_e0_rae` writer - RAE"]
pub type WponE0RaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpon_e0_wae` reader - WAE"]
pub type WponE0WaeR = crate::BitReader;
#[doc = "Field `wpon_e0_wae` writer - WAE"]
pub type WponE0WaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wpon_e0_waem` reader - WAEM"]
pub type WponE0WaemR = crate::FieldReader;
#[doc = "Field `wpon_e0_waem` writer - WAEM"]
pub type WponE0WaemW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpon_e0_loop_mode(&self) -> GrpWponE0LoopModeR {
        GrpWponE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpon_e0_msel(&self) -> GrpWponE0MselR {
        GrpWponE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn wpon_e0_dvse(&self) -> WponE0DvseR {
        WponE0DvseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn wpon_e0_dvs(&self) -> WponE0DvsR {
        WponE0DvsR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn wpon_e0_tmck(&self) -> WponE0TmckR {
        WponE0TmckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn wpon_e0_rae(&self) -> WponE0RaeR {
        WponE0RaeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn wpon_e0_wae(&self) -> WponE0WaeR {
        WponE0WaeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn wpon_e0_waem(&self) -> WponE0WaemR {
        WponE0WaemR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_wpon_e0_loop_mode(&mut self) -> GrpWponE0LoopModeW<'_, GrpWponE0CtrlSpec> {
        GrpWponE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_wpon_e0_msel(&mut self) -> GrpWponE0MselW<'_, GrpWponE0CtrlSpec> {
        GrpWponE0MselW::new(self, 8)
    }
    #[doc = "Bit 10 - DVSE"]
    #[inline(always)]
    pub fn wpon_e0_dvse(&mut self) -> WponE0DvseW<'_, GrpWponE0CtrlSpec> {
        WponE0DvseW::new(self, 10)
    }
    #[doc = "Bits 11:14 - DVS"]
    #[inline(always)]
    pub fn wpon_e0_dvs(&mut self) -> WponE0DvsW<'_, GrpWponE0CtrlSpec> {
        WponE0DvsW::new(self, 11)
    }
    #[doc = "Bit 15 - TMCK"]
    #[inline(always)]
    pub fn wpon_e0_tmck(&mut self) -> WponE0TmckW<'_, GrpWponE0CtrlSpec> {
        WponE0TmckW::new(self, 15)
    }
    #[doc = "Bit 16 - RAE"]
    #[inline(always)]
    pub fn wpon_e0_rae(&mut self) -> WponE0RaeW<'_, GrpWponE0CtrlSpec> {
        WponE0RaeW::new(self, 16)
    }
    #[doc = "Bit 17 - WAE"]
    #[inline(always)]
    pub fn wpon_e0_wae(&mut self) -> WponE0WaeW<'_, GrpWponE0CtrlSpec> {
        WponE0WaeW::new(self, 17)
    }
    #[doc = "Bits 18:21 - WAEM"]
    #[inline(always)]
    pub fn wpon_e0_waem(&mut self) -> WponE0WaemW<'_, GrpWponE0CtrlSpec> {
        WponE0WaemW::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_wpon_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_wpon_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpWponE0CtrlSpec;
impl crate::RegisterSpec for GrpWponE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_wpon_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpWponE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_wpon_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpWponE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_WPON_E0_CTRL to value 0x002b_0300"]
impl crate::Resettable for GrpWponE0CtrlSpec {
    const RESET_VALUE: u32 = 0x002b_0300;
}
