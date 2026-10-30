#[doc = "Register `REG_ATIMWND6_7_URGENT_NOLIMTHGQ` reader"]
pub type R = crate::R<RegAtimwnd6_7UrgentNolimthgqSpec>;
#[doc = "Register `REG_ATIMWND6_7_URGENT_NOLIMTHGQ` writer"]
pub type W = crate::W<RegAtimwnd6_7UrgentNolimthgqSpec>;
#[doc = "Field `BIT_ATIMWND6` reader - ATIM window width of VAP6 in units of TU."]
pub type BitAtimwnd6R = crate::FieldReader;
#[doc = "Field `BIT_ATIMWND6` writer - ATIM window width of VAP6 in units of TU."]
pub type BitAtimwnd6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ATIMWND7` reader - ATIM window width of VAP7 in units of TU."]
pub type BitAtimwnd7R = crate::FieldReader;
#[doc = "Field `BIT_ATIMWND7` writer - ATIM window width of VAP7 in units of TU."]
pub type BitAtimwnd7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_ATIM_URGENT` reader - Indicates the time before ATIM Window end . When BSSID\\[n\\] (n=0~7) has HIQ packet to send, HW only DMA BSSID\\[n\\]'s HIQ packet in this interval. (unit : TU)"]
pub type BitAtimUrgentR = crate::FieldReader;
#[doc = "Field `BIT_ATIM_URGENT` writer - Indicates the time before ATIM Window end . When BSSID\\[n\\] (n=0~7) has HIQ packet to send, HW only DMA BSSID\\[n\\]'s HIQ packet in this interval. (unit : TU)"]
pub type BitAtimUrgentW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_ROOT` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnRootR = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_ROOT` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnRootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP1` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap1R = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP1` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP2` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap2R = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP2` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP3` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap3R = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP3` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP4` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap4R = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP4` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP5` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap5R = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP5` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP6` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap6R = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP6` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP7` reader - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap7R = crate::BitReader;
#[doc = "Field `BIT_HIQ_NO_LMT_EN_VAP7` writer - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
pub type BitHiqNoLmtEnVap7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - ATIM window width of VAP6 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd6(&self) -> BitAtimwnd6R {
        BitAtimwnd6R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATIM window width of VAP7 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd7(&self) -> BitAtimwnd7R {
        BitAtimwnd7R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates the time before ATIM Window end . When BSSID\\[n\\] (n=0~7) has HIQ packet to send, HW only DMA BSSID\\[n\\]'s HIQ packet in this interval. (unit : TU)"]
    #[inline(always)]
    pub fn bit_atim_urgent(&self) -> BitAtimUrgentR {
        BitAtimUrgentR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_root(&self) -> BitHiqNoLmtEnRootR {
        BitHiqNoLmtEnRootR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap1(&self) -> BitHiqNoLmtEnVap1R {
        BitHiqNoLmtEnVap1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap2(&self) -> BitHiqNoLmtEnVap2R {
        BitHiqNoLmtEnVap2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap3(&self) -> BitHiqNoLmtEnVap3R {
        BitHiqNoLmtEnVap3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap4(&self) -> BitHiqNoLmtEnVap4R {
        BitHiqNoLmtEnVap4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap5(&self) -> BitHiqNoLmtEnVap5R {
        BitHiqNoLmtEnVap5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap6(&self) -> BitHiqNoLmtEnVap6R {
        BitHiqNoLmtEnVap6R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap7(&self) -> BitHiqNoLmtEnVap7R {
        BitHiqNoLmtEnVap7R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATIM window width of VAP6 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd6(&mut self) -> BitAtimwnd6W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitAtimwnd6W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ATIM window width of VAP7 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd7(&mut self) -> BitAtimwnd7W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitAtimwnd7W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Indicates the time before ATIM Window end . When BSSID\\[n\\] (n=0~7) has HIQ packet to send, HW only DMA BSSID\\[n\\]'s HIQ packet in this interval. (unit : TU)"]
    #[inline(always)]
    pub fn bit_atim_urgent(&mut self) -> BitAtimUrgentW<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitAtimUrgentW::new(self, 16)
    }
    #[doc = "Bit 24 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_root(
        &mut self,
    ) -> BitHiqNoLmtEnRootW<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnRootW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap1(
        &mut self,
    ) -> BitHiqNoLmtEnVap1W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnVap1W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap2(
        &mut self,
    ) -> BitHiqNoLmtEnVap2W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnVap2W::new(self, 26)
    }
    #[doc = "Bit 27 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap3(
        &mut self,
    ) -> BitHiqNoLmtEnVap3W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnVap3W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap4(
        &mut self,
    ) -> BitHiqNoLmtEnVap4W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnVap4W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap5(
        &mut self,
    ) -> BitHiqNoLmtEnVap5W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnVap5W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap6(
        &mut self,
    ) -> BitHiqNoLmtEnVap6W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnVap6W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: Packet in Hi Queue Tx immediately (No constraint for ATIM Period) 0: Disable"]
    #[inline(always)]
    pub fn bit_hiq_no_lmt_en_vap7(
        &mut self,
    ) -> BitHiqNoLmtEnVap7W<'_, RegAtimwnd6_7UrgentNolimthgqSpec> {
        BitHiqNoLmtEnVap7W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_atimwnd6_7_urgent_nolimthgq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_atimwnd6_7_urgent_nolimthgq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAtimwnd6_7UrgentNolimthgqSpec;
impl crate::RegisterSpec for RegAtimwnd6_7UrgentNolimthgqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_atimwnd6_7_urgent_nolimthgq::R`](R) reader structure"]
impl crate::Readable for RegAtimwnd6_7UrgentNolimthgqSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_atimwnd6_7_urgent_nolimthgq::W`](W) writer structure"]
impl crate::Writable for RegAtimwnd6_7UrgentNolimthgqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ATIMWND6_7_URGENT_NOLIMTHGQ to value 0x0001_0202"]
impl crate::Resettable for RegAtimwnd6_7UrgentNolimthgqSpec {
    const RESET_VALUE: u32 = 0x0001_0202;
}
