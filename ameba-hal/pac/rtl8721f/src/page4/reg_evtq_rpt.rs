#[doc = "Register `REG_EVTQ_RPT` reader"]
pub type R = crate::R<RegEvtqRptSpec>;
#[doc = "Register `REG_EVTQ_RPT` writer"]
pub type W = crate::W<RegEvtqRptSpec>;
#[doc = "Field `BIT_pta_wl_pri_mask_EVT` reader - PTCL send wl_pri to PTA when pktbuf has EVTQ packet"]
pub type BitPtaWlPriMaskEvtR = crate::BitReader;
#[doc = "Field `BIT_pta_wl_pri_mask_EVT` writer - PTCL send wl_pri to PTA when pktbuf has EVTQ packet"]
pub type BitPtaWlPriMaskEvtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_evtq_tx_rpt` reader - *000: nothing *100: pktok *010: pktfail / timeout *001: evtq_tx_area_75"]
pub type BitEvtqTxRptR = crate::FieldReader;
impl R {
    #[doc = "Bit 25 - PTCL send wl_pri to PTA when pktbuf has EVTQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_evt(&self) -> BitPtaWlPriMaskEvtR {
        BitPtaWlPriMaskEvtR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 27:29 - *000: nothing *100: pktok *010: pktfail / timeout *001: evtq_tx_area_75"]
    #[inline(always)]
    pub fn bit_evtq_tx_rpt(&self) -> BitEvtqTxRptR {
        BitEvtqTxRptR::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - PTCL send wl_pri to PTA when pktbuf has EVTQ packet"]
    #[inline(always)]
    pub fn bit_pta_wl_pri_mask_evt(&mut self) -> BitPtaWlPriMaskEvtW<'_, RegEvtqRptSpec> {
        BitPtaWlPriMaskEvtW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_evtq_rpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_evtq_rpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegEvtqRptSpec;
impl crate::RegisterSpec for RegEvtqRptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_evtq_rpt::R`](R) reader structure"]
impl crate::Readable for RegEvtqRptSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_evtq_rpt::W`](W) writer structure"]
impl crate::Writable for RegEvtqRptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_EVTQ_RPT to value 0"]
impl crate::Resettable for RegEvtqRptSpec {}
