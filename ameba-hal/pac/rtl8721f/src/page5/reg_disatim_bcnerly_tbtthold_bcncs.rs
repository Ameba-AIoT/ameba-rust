#[doc = "Register `REG_DISATIM_BCNERLY_TBTTHOLD_BCNCS` reader"]
pub type R = crate::R<RegDisatimBcnerlyTbttholdBcncsSpec>;
#[doc = "Register `REG_DISATIM_BCNERLY_TBTTHOLD_BCNCS` writer"]
pub type W = crate::W<RegDisatimBcnerlyTbttholdBcncsSpec>;
#[doc = "Field `BIT_DIS_ATIM_ROOT` reader - "]
pub type BitDisAtimRootR = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_ROOT` writer - "]
pub type BitDisAtimRootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ATIM_VAP1` reader - "]
pub type BitDisAtimVap1R = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_VAP1` writer - "]
pub type BitDisAtimVap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ATIM_VAP2` reader - "]
pub type BitDisAtimVap2R = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_VAP2` writer - "]
pub type BitDisAtimVap2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ATIM_VAP3` reader - "]
pub type BitDisAtimVap3R = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_VAP3` writer - "]
pub type BitDisAtimVap3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ATIM_VAP4` reader - "]
pub type BitDisAtimVap4R = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_VAP4` writer - "]
pub type BitDisAtimVap4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ATIM_VAP5` reader - "]
pub type BitDisAtimVap5R = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_VAP5` writer - "]
pub type BitDisAtimVap5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ATIM_VAP6` reader - "]
pub type BitDisAtimVap6R = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_VAP6` writer - "]
pub type BitDisAtimVap6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_ATIM_VAP7` reader - "]
pub type BitDisAtimVap7R = crate::BitReader;
#[doc = "Field `BIT_DIS_ATIM_VAP7` writer - "]
pub type BitDisAtimVap7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EARLY_128US_P0` reader - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us."]
pub type BitEarly128usP0R = crate::FieldReader;
#[doc = "Field `BIT_EARLY_128US_P0` writer - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us."]
pub type BitEarly128usP0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_EARLY_128US_P1` reader - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us. For port 1"]
pub type BitEarly128usP1R = crate::FieldReader;
#[doc = "Field `BIT_EARLY_128US_P1` writer - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us. For port 1"]
pub type BitEarly128usP1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_DIS_BCN_P2_1` reader - set 1 to mask the NAN sync beacon early interrupt of port2"]
pub type BitDisBcnP2_1R = crate::BitReader;
#[doc = "Field `BIT_DIS_BCN_P2_1` writer - set 1 to mask the NAN sync beacon early interrupt of port2"]
pub type BitDisBcnP2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBTT_HOLD_PREDICT` reader - the predicted hold time after tbtt for P0 RX beacon, unit is 1ms"]
pub type TbttHoldPredictR = crate::FieldReader;
#[doc = "Field `TBTT_HOLD_PREDICT` writer - the predicted hold time after tbtt for P0 RX beacon, unit is 1ms"]
pub type TbttHoldPredictW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_DIS_BCN_P0` reader - set 1 to mask the beacon early interrupt of port0"]
pub type BitDisBcnP0R = crate::BitReader;
#[doc = "Field `BIT_DIS_BCN_P0` writer - set 1 to mask the beacon early interrupt of port0"]
pub type BitDisBcnP0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_BCN_P1` reader - set 1 to mask the beacon early interrupt of port1"]
pub type BitDisBcnP1R = crate::BitReader;
#[doc = "Field `BIT_DIS_BCN_P1` writer - set 1 to mask the beacon early interrupt of port1"]
pub type BitDisBcnP1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_BCN_P2_0` reader - set 1 to mask the NAN disc beacon early interrupt of port2"]
pub type BitDisBcnP2_0R = crate::BitReader;
#[doc = "Field `BIT_DIS_BCN_P2_0` writer - set 1 to mask the NAN disc beacon early interrupt of port2"]
pub type BitDisBcnP2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MULTI_BCN_CS` reader - indicate which port is tx beacon"]
pub type BitMultiBcnCsR = crate::FieldReader;
#[doc = "Field `BIT_TSFTR2_RST` writer - write this bit , bcn2 tsft will auto reset"]
pub type BitTsftr2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSFTR2_SYNC_EN` writer - write this bit, PORT2 backup TSFT will sync to PORT2 TSFT without TSFTF_SYN_OFFSET"]
pub type BitTsftr2SyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TSFTR3_SYNC_EN` writer - write this bit, PORT3 will sync to PORT2 with TSFTF_SYN_OFFSET"]
pub type BitTsftr3SyncEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bit_dis_atim_root(&self) -> BitDisAtimRootR {
        BitDisAtimRootR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bit_dis_atim_vap1(&self) -> BitDisAtimVap1R {
        BitDisAtimVap1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bit_dis_atim_vap2(&self) -> BitDisAtimVap2R {
        BitDisAtimVap2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bit_dis_atim_vap3(&self) -> BitDisAtimVap3R {
        BitDisAtimVap3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bit_dis_atim_vap4(&self) -> BitDisAtimVap4R {
        BitDisAtimVap4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bit_dis_atim_vap5(&self) -> BitDisAtimVap5R {
        BitDisAtimVap5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bit_dis_atim_vap6(&self) -> BitDisAtimVap6R {
        BitDisAtimVap6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bit_dis_atim_vap7(&self) -> BitDisAtimVap7R {
        BitDisAtimVap7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us."]
    #[inline(always)]
    pub fn bit_early_128us_p0(&self) -> BitEarly128usP0R {
        BitEarly128usP0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us. For port 1"]
    #[inline(always)]
    pub fn bit_early_128us_p1(&self) -> BitEarly128usP1R {
        BitEarly128usP1R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 15 - set 1 to mask the NAN sync beacon early interrupt of port2"]
    #[inline(always)]
    pub fn bit_dis_bcn_p2_1(&self) -> BitDisBcnP2_1R {
        BitDisBcnP2_1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - the predicted hold time after tbtt for P0 RX beacon, unit is 1ms"]
    #[inline(always)]
    pub fn tbtt_hold_predict(&self) -> TbttHoldPredictR {
        TbttHoldPredictR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - set 1 to mask the beacon early interrupt of port0"]
    #[inline(always)]
    pub fn bit_dis_bcn_p0(&self) -> BitDisBcnP0R {
        BitDisBcnP0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - set 1 to mask the beacon early interrupt of port1"]
    #[inline(always)]
    pub fn bit_dis_bcn_p1(&self) -> BitDisBcnP1R {
        BitDisBcnP1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - set 1 to mask the NAN disc beacon early interrupt of port2"]
    #[inline(always)]
    pub fn bit_dis_bcn_p2_0(&self) -> BitDisBcnP2_0R {
        BitDisBcnP2_0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - indicate which port is tx beacon"]
    #[inline(always)]
    pub fn bit_multi_bcn_cs(&self) -> BitMultiBcnCsR {
        BitMultiBcnCsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bit_dis_atim_root(&mut self) -> BitDisAtimRootW<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimRootW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bit_dis_atim_vap1(&mut self) -> BitDisAtimVap1W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimVap1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn bit_dis_atim_vap2(&mut self) -> BitDisAtimVap2W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimVap2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bit_dis_atim_vap3(&mut self) -> BitDisAtimVap3W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimVap3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bit_dis_atim_vap4(&mut self) -> BitDisAtimVap4W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimVap4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn bit_dis_atim_vap5(&mut self) -> BitDisAtimVap5W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimVap5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bit_dis_atim_vap6(&mut self) -> BitDisAtimVap6W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimVap6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bit_dis_atim_vap7(&mut self) -> BitDisAtimVap7W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisAtimVap7W::new(self, 7)
    }
    #[doc = "Bits 8:10 - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us."]
    #[inline(always)]
    pub fn bit_early_128us_p0(
        &mut self,
    ) -> BitEarly128usP0W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitEarly128usP0W::new(self, 8)
    }
    #[doc = "Bits 11:13 - the BCN EARLY interrupt find tune with unit 128us. The Total Early time of BCN early interrupt is the 0x558 x 1ms + EARLY_128us x 128us. For port 1"]
    #[inline(always)]
    pub fn bit_early_128us_p1(
        &mut self,
    ) -> BitEarly128usP1W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitEarly128usP1W::new(self, 11)
    }
    #[doc = "Bit 15 - set 1 to mask the NAN sync beacon early interrupt of port2"]
    #[inline(always)]
    pub fn bit_dis_bcn_p2_1(&mut self) -> BitDisBcnP2_1W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisBcnP2_1W::new(self, 15)
    }
    #[doc = "Bits 16:20 - the predicted hold time after tbtt for P0 RX beacon, unit is 1ms"]
    #[inline(always)]
    pub fn tbtt_hold_predict(
        &mut self,
    ) -> TbttHoldPredictW<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        TbttHoldPredictW::new(self, 16)
    }
    #[doc = "Bit 21 - set 1 to mask the beacon early interrupt of port0"]
    #[inline(always)]
    pub fn bit_dis_bcn_p0(&mut self) -> BitDisBcnP0W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisBcnP0W::new(self, 21)
    }
    #[doc = "Bit 22 - set 1 to mask the beacon early interrupt of port1"]
    #[inline(always)]
    pub fn bit_dis_bcn_p1(&mut self) -> BitDisBcnP1W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisBcnP1W::new(self, 22)
    }
    #[doc = "Bit 23 - set 1 to mask the NAN disc beacon early interrupt of port2"]
    #[inline(always)]
    pub fn bit_dis_bcn_p2_0(&mut self) -> BitDisBcnP2_0W<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitDisBcnP2_0W::new(self, 23)
    }
    #[doc = "Bit 28 - write this bit , bcn2 tsft will auto reset"]
    #[inline(always)]
    pub fn bit_tsftr2_rst(&mut self) -> BitTsftr2RstW<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitTsftr2RstW::new(self, 28)
    }
    #[doc = "Bit 29 - write this bit, PORT2 backup TSFT will sync to PORT2 TSFT without TSFTF_SYN_OFFSET"]
    #[inline(always)]
    pub fn bit_tsftr2_sync_en(
        &mut self,
    ) -> BitTsftr2SyncEnW<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitTsftr2SyncEnW::new(self, 29)
    }
    #[doc = "Bit 31 - write this bit, PORT3 will sync to PORT2 with TSFTF_SYN_OFFSET"]
    #[inline(always)]
    pub fn bit_tsftr3_sync_en(
        &mut self,
    ) -> BitTsftr3SyncEnW<'_, RegDisatimBcnerlyTbttholdBcncsSpec> {
        BitTsftr3SyncEnW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_disatim_bcnerly_tbtthold_bcncs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_disatim_bcnerly_tbtthold_bcncs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDisatimBcnerlyTbttholdBcncsSpec;
impl crate::RegisterSpec for RegDisatimBcnerlyTbttholdBcncsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_disatim_bcnerly_tbtthold_bcncs::R`](R) reader structure"]
impl crate::Readable for RegDisatimBcnerlyTbttholdBcncsSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_disatim_bcnerly_tbtthold_bcncs::W`](W) writer structure"]
impl crate::Writable for RegDisatimBcnerlyTbttholdBcncsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DISATIM_BCNERLY_TBTTHOLD_BCNCS to value 0x0002_0000"]
impl crate::Resettable for RegDisatimBcnerlyTbttholdBcncsSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
