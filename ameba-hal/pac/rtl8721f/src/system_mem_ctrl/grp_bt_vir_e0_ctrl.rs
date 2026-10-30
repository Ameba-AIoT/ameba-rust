#[doc = "Register `GRP_BT_VIR_E0_CTRL` reader"]
pub type R = crate::R<GrpBtVirE0CtrlSpec>;
#[doc = "Register `GRP_BT_VIR_E0_CTRL` writer"]
pub type W = crate::W<GrpBtVirE0CtrlSpec>;
#[doc = "Field `grp_bt_vir_e0_rstn` reader - *0 - Reset mbist circuit of this memory group; *1 - Release mbist circuit of this memory group."]
pub type GrpBtVirE0RstnR = crate::BitReader;
#[doc = "Field `grp_bt_vir_e0_rstn` writer - *0 - Reset mbist circuit of this memory group; *1 - Release mbist circuit of this memory group."]
pub type GrpBtVirE0RstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_bt_vir_e0_loop_mode` reader - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpBtVirE0LoopModeR = crate::BitReader;
#[doc = "Field `grp_bt_vir_e0_loop_mode` writer - *0 - Assist loop mode; *1 - Don't assist loop mode"]
pub type GrpBtVirE0LoopModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `grp_bt_vir_e0_msel` reader - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpBtVirE0MselR = crate::FieldReader;
#[doc = "Field `grp_bt_vir_e0_msel` writer - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
pub type GrpBtVirE0MselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 6 - *0 - Reset mbist circuit of this memory group; *1 - Release mbist circuit of this memory group."]
    #[inline(always)]
    pub fn grp_bt_vir_e0_rstn(&self) -> GrpBtVirE0RstnR {
        GrpBtVirE0RstnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_bt_vir_e0_loop_mode(&self) -> GrpBtVirE0LoopModeR {
        GrpBtVirE0LoopModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_bt_vir_e0_msel(&self) -> GrpBtVirE0MselR {
        GrpBtVirE0MselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - *0 - Reset mbist circuit of this memory group; *1 - Release mbist circuit of this memory group."]
    #[inline(always)]
    pub fn grp_bt_vir_e0_rstn(&mut self) -> GrpBtVirE0RstnW<'_, GrpBtVirE0CtrlSpec> {
        GrpBtVirE0RstnW::new(self, 6)
    }
    #[doc = "Bit 7 - *0 - Assist loop mode; *1 - Don't assist loop mode"]
    #[inline(always)]
    pub fn grp_bt_vir_e0_loop_mode(&mut self) -> GrpBtVirE0LoopModeW<'_, GrpBtVirE0CtrlSpec> {
        GrpBtVirE0LoopModeW::new(self, 7)
    }
    #[doc = "Bits 8:9 - *01 - This memory group can do normal mbist; *10 - This memory group can do DRF mbist; *11 - This memory group can do normal & DRF mbist"]
    #[inline(always)]
    pub fn grp_bt_vir_e0_msel(&mut self) -> GrpBtVirE0MselW<'_, GrpBtVirE0CtrlSpec> {
        GrpBtVirE0MselW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`grp_bt_vir_e0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp_bt_vir_e0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrpBtVirE0CtrlSpec;
impl crate::RegisterSpec for GrpBtVirE0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp_bt_vir_e0_ctrl::R`](R) reader structure"]
impl crate::Readable for GrpBtVirE0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp_bt_vir_e0_ctrl::W`](W) writer structure"]
impl crate::Writable for GrpBtVirE0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP_BT_VIR_E0_CTRL to value 0x0340"]
impl crate::Resettable for GrpBtVirE0CtrlSpec {
    const RESET_VALUE: u32 = 0x0340;
}
