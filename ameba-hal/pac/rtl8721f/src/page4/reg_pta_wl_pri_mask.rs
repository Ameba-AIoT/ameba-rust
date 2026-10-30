#[doc = "Register `REG_PTA_WL_PRI_MASK` reader"]
pub type R = crate::R<RegPtaWlPriMaskSpec>;
#[doc = "Register `REG_PTA_WL_PRI_MASK` writer"]
pub type W = crate::W<RegPtaWlPriMaskSpec>;
#[doc = "Field `BIT_pta_wl_pri_mask_VO` reader - PTCL send wl_pri to PTA when pktbuf has VOQ packet"]
pub type BitPtaWlPriMaskVoR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_VO` writer - PTCL send wl_pri to PTA when pktbuf has VOQ packet"]
pub type BitPtaWlPriMaskVoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_pri_mask_VI` reader - PTCL send wl_pri to PTA when pktbuf has VIQ packet"]
pub type BitPtaWlPriMaskViR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_VI` writer - PTCL send wl_pri to PTA when pktbuf has VIQ packet"]
pub type BitPtaWlPriMaskViW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_pri_mask_BE` reader - PTCL send wl_pri to PTA when pktbuf has BEQ packet"]
pub type BitPtaWlPriMaskBeR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_BE` writer - PTCL send wl_pri to PTA when pktbuf has BEQ packet"]
pub type BitPtaWlPriMaskBeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_pri_mask_BK` reader - PTCL send wl_pri to PTA when pktbuf has BKQ packet"]
pub type BitPtaWlPriMaskBkR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_BK` writer - PTCL send wl_pri to PTA when pktbuf has BKQ packet"]
pub type BitPtaWlPriMaskBkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_pri_mask_MGQ` reader - PTCL send wl_pri to PTA when pktbuf has MGQ packet"]
pub type BitPtaWlPriMaskMgqR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_MGQ` writer - PTCL send wl_pri to PTA when pktbuf has MGQ packet"]
pub type BitPtaWlPriMaskMgqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_pri_mask_HIQ` reader - PTCL send wl_pri to PTA when pktbuf has HIQ packet"]
pub type BitPtaWlPriMaskHiqR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_HIQ` writer - PTCL send wl_pri to PTA when pktbuf has HIQ packet"]
pub type BitPtaWlPriMaskHiqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_pri_mask_BCNQ` reader - PTCL send wl_pri to PTA when pktbuf has BCNQ packet"]
pub type BitPtaWlPriMaskBcnqR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_BCNQ` writer - PTCL send wl_pri to PTA when pktbuf has BCNQ packet"]
pub type BitPtaWlPriMaskBcnqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_pta_wl_pri_mask_CPU_MGQ` reader - PTCL send wl_pri to PTA when pktbuf has CPU_MGQ packet"]
pub type BitPtaWlPriMaskCpuMgqR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_CPU_MGQ` writer - PTCL send wl_pri to PTA when pktbuf has CPU_MGQ packet"]
pub type BitPtaWlPriMaskCpuMgqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - PTCL send wl_pri to PTA when pktbuf has VOQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_vo(&self) -> BitPtaWlPriMaskVoR {
        BitPtaWlPriMaskVoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PTCL send wl_pri to PTA when pktbuf has VIQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_vi(&self) -> BitPtaWlPriMaskViR {
        BitPtaWlPriMaskViR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PTCL send wl_pri to PTA when pktbuf has BEQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_be(&self) -> BitPtaWlPriMaskBeR {
        BitPtaWlPriMaskBeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PTCL send wl_pri to PTA when pktbuf has BKQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_bk(&self) -> BitPtaWlPriMaskBkR {
        BitPtaWlPriMaskBkR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PTCL send wl_pri to PTA when pktbuf has MGQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_mgq(&self) -> BitPtaWlPriMaskMgqR {
        BitPtaWlPriMaskMgqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PTCL send wl_pri to PTA when pktbuf has HIQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_hiq(&self) -> BitPtaWlPriMaskHiqR {
        BitPtaWlPriMaskHiqR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PTCL send wl_pri to PTA when pktbuf has BCNQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_bcnq(&self) -> BitPtaWlPriMaskBcnqR {
        BitPtaWlPriMaskBcnqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PTCL send wl_pri to PTA when pktbuf has CPU_MGQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_cpu_mgq(&self) -> BitPtaWlPriMaskCpuMgqR {
        BitPtaWlPriMaskCpuMgqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - PTCL send wl_pri to PTA when pktbuf has VOQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_vo(&mut self) -> BitPtaWlPriMaskVoW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskVoW::new(self, 24)
    }
    #[doc = "Bit 25 - PTCL send wl_pri to PTA when pktbuf has VIQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_vi(&mut self) -> BitPtaWlPriMaskViW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskViW::new(self, 25)
    }
    #[doc = "Bit 26 - PTCL send wl_pri to PTA when pktbuf has BEQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_be(&mut self) -> BitPtaWlPriMaskBeW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskBeW::new(self, 26)
    }
    #[doc = "Bit 27 - PTCL send wl_pri to PTA when pktbuf has BKQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_bk(&mut self) -> BitPtaWlPriMaskBkW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskBkW::new(self, 27)
    }
    #[doc = "Bit 28 - PTCL send wl_pri to PTA when pktbuf has MGQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_mgq(&mut self) -> BitPtaWlPriMaskMgqW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskMgqW::new(self, 28)
    }
    #[doc = "Bit 29 - PTCL send wl_pri to PTA when pktbuf has HIQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_hiq(&mut self) -> BitPtaWlPriMaskHiqW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskHiqW::new(self, 29)
    }
    #[doc = "Bit 30 - PTCL send wl_pri to PTA when pktbuf has BCNQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_bcnq(&mut self) -> BitPtaWlPriMaskBcnqW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskBcnqW::new(self, 30)
    }
    #[doc = "Bit 31 - PTCL send wl_pri to PTA when pktbuf has CPU_MGQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_cpu_mgq(
        &mut self,
    ) -> BitPtaWlPriMaskCpuMgqW<'_, RegPtaWlPriMaskSpec> {
        BitPtaWlPriMaskCpuMgqW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pta_wl_pri_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pta_wl_pri_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPtaWlPriMaskSpec;
impl crate::RegisterSpec for RegPtaWlPriMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pta_wl_pri_mask::R`](R) reader structure"]
impl crate::Readable for RegPtaWlPriMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pta_wl_pri_mask::W`](W) writer structure"]
impl crate::Writable for RegPtaWlPriMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PTA_WL_PRI_MASK to value 0"]
impl crate::Resettable for RegPtaWlPriMaskSpec {}
