#[doc = "Register `REG_BEAMFORMING_INFO_NSARP` reader"]
pub type R = crate::R<RegBeamformingInfoNsarpSpec>;
#[doc = "Register `REG_BEAMFORMING_INFO_NSARP` writer"]
pub type W = crate::W<RegBeamformingInfoNsarpSpec>;
#[doc = "Field `BIT_UPD_BFMEE_NC` reader - Nc Index of MIMO control field \\[2:0\\] for VHT (B2-0) , \\[1:0\\] for HT mode (B1-0)"]
pub type BitUpdBfmeeNcR = crate::FieldReader;
#[doc = "Field `BIT_UPD_BFMEE_NC` writer - Nc Index of MIMO control field \\[2:0\\] for VHT (B2-0) , \\[1:0\\] for HT mode (B1-0)"]
pub type BitUpdBfmeeNcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_UPD_BFMEE_NR` reader - Nr Index of MIMO control field \\[5:3\\] for VHT (B5-3) ,\\[4:3\\] for HT mode (B3-2)"]
pub type BitUpdBfmeeNrR = crate::FieldReader;
#[doc = "Field `BIT_UPD_BFMEE_NR` writer - Nr Index of MIMO control field \\[5:3\\] for VHT (B5-3) ,\\[4:3\\] for HT mode (B3-2)"]
pub type BitUpdBfmeeNrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIT_UPD_BFMEE_NG` reader - Grouping of MIMO control field \\[7:6\\] for VHT (B9-8) and (B6-5) when HT mode"]
pub type BitUpdBfmeeNgR = crate::FieldReader;
#[doc = "Field `BIT_UPD_BFMEE_NG` writer - Grouping of MIMO control field \\[7:6\\] for VHT (B9-8) and (B6-5) when HT mode"]
pub type BitUpdBfmeeNgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_UPD_BFMEE_CB` reader - Codebook Information of MIMO control field \\[8\\] for VHT (B10) and (B10-9) when HT mode"]
pub type BitUpdBfmeeCbR = crate::FieldReader;
#[doc = "Field `BIT_UPD_BFMEE_CB` writer - Codebook Information of MIMO control field \\[8\\] for VHT (B10) and (B10-9) when HT mode"]
pub type BitUpdBfmeeCbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_UPD_BFMEE_BW` reader - Channel Width of VHT MIMO control field (B7-6) and (B4) when HT mode"]
pub type BitUpdBfmeeBwR = crate::FieldReader;
#[doc = "Field `BIT_UPD_BFMEE_BW` writer - Channel Width of VHT MIMO control field (B7-6) and (B4) when HT mode"]
pub type BitUpdBfmeeBwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_UPD_BFMEE_FBTP` reader - Feedback Type(B11) of VHT MIMO Control field, HT is no used"]
pub type BitUpdBfmeeFbtpR = crate::BitReader;
#[doc = "Field `BIT_UPD_BFMEE_FBTP` writer - Feedback Type(B11) of VHT MIMO Control field, HT is no used"]
pub type BitUpdBfmeeFbtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_UPD_BFMEE_USERID` reader - Select BFMEE User ID"]
pub type BitUpdBfmeeUseridR = crate::FieldReader;
#[doc = "Field `BIT_UPD_BFMEE_USERID` writer - Select BFMEE User ID"]
pub type BitUpdBfmeeUseridW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Nc Index of MIMO control field \\[2:0\\] for VHT (B2-0) , \\[1:0\\] for HT mode (B1-0)"]
    #[inline(always)]
    pub fn bit_upd_bfmee_nc(&self) -> BitUpdBfmeeNcR {
        BitUpdBfmeeNcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Nr Index of MIMO control field \\[5:3\\] for VHT (B5-3) ,\\[4:3\\] for HT mode (B3-2)"]
    #[inline(always)]
    pub fn bit_upd_bfmee_nr(&self) -> BitUpdBfmeeNrR {
        BitUpdBfmeeNrR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Grouping of MIMO control field \\[7:6\\] for VHT (B9-8) and (B6-5) when HT mode"]
    #[inline(always)]
    pub fn bit_upd_bfmee_ng(&self) -> BitUpdBfmeeNgR {
        BitUpdBfmeeNgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Codebook Information of MIMO control field \\[8\\] for VHT (B10) and (B10-9) when HT mode"]
    #[inline(always)]
    pub fn bit_upd_bfmee_cb(&self) -> BitUpdBfmeeCbR {
        BitUpdBfmeeCbR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel Width of VHT MIMO control field (B7-6) and (B4) when HT mode"]
    #[inline(always)]
    pub fn bit_upd_bfmee_bw(&self) -> BitUpdBfmeeBwR {
        BitUpdBfmeeBwR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Feedback Type(B11) of VHT MIMO Control field, HT is no used"]
    #[inline(always)]
    pub fn bit_upd_bfmee_fbtp(&self) -> BitUpdBfmeeFbtpR {
        BitUpdBfmeeFbtpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Select BFMEE User ID"]
    #[inline(always)]
    pub fn bit_upd_bfmee_userid(&self) -> BitUpdBfmeeUseridR {
        BitUpdBfmeeUseridR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Nc Index of MIMO control field \\[2:0\\] for VHT (B2-0) , \\[1:0\\] for HT mode (B1-0)"]
    #[inline(always)]
    pub fn bit_upd_bfmee_nc(&mut self) -> BitUpdBfmeeNcW<'_, RegBeamformingInfoNsarpSpec> {
        BitUpdBfmeeNcW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Nr Index of MIMO control field \\[5:3\\] for VHT (B5-3) ,\\[4:3\\] for HT mode (B3-2)"]
    #[inline(always)]
    pub fn bit_upd_bfmee_nr(&mut self) -> BitUpdBfmeeNrW<'_, RegBeamformingInfoNsarpSpec> {
        BitUpdBfmeeNrW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Grouping of MIMO control field \\[7:6\\] for VHT (B9-8) and (B6-5) when HT mode"]
    #[inline(always)]
    pub fn bit_upd_bfmee_ng(&mut self) -> BitUpdBfmeeNgW<'_, RegBeamformingInfoNsarpSpec> {
        BitUpdBfmeeNgW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Codebook Information of MIMO control field \\[8\\] for VHT (B10) and (B10-9) when HT mode"]
    #[inline(always)]
    pub fn bit_upd_bfmee_cb(&mut self) -> BitUpdBfmeeCbW<'_, RegBeamformingInfoNsarpSpec> {
        BitUpdBfmeeCbW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel Width of VHT MIMO control field (B7-6) and (B4) when HT mode"]
    #[inline(always)]
    pub fn bit_upd_bfmee_bw(&mut self) -> BitUpdBfmeeBwW<'_, RegBeamformingInfoNsarpSpec> {
        BitUpdBfmeeBwW::new(self, 10)
    }
    #[doc = "Bit 12 - Feedback Type(B11) of VHT MIMO Control field, HT is no used"]
    #[inline(always)]
    pub fn bit_upd_bfmee_fbtp(&mut self) -> BitUpdBfmeeFbtpW<'_, RegBeamformingInfoNsarpSpec> {
        BitUpdBfmeeFbtpW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Select BFMEE User ID"]
    #[inline(always)]
    pub fn bit_upd_bfmee_userid(&mut self) -> BitUpdBfmeeUseridW<'_, RegBeamformingInfoNsarpSpec> {
        BitUpdBfmeeUseridW::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_beamforming_info_nsarp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_beamforming_info_nsarp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBeamformingInfoNsarpSpec;
impl crate::RegisterSpec for RegBeamformingInfoNsarpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_beamforming_info_nsarp::R`](R) reader structure"]
impl crate::Readable for RegBeamformingInfoNsarpSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_beamforming_info_nsarp::W`](W) writer structure"]
impl crate::Writable for RegBeamformingInfoNsarpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BEAMFORMING_INFO_NSARP to value 0"]
impl crate::Resettable for RegBeamformingInfoNsarpSpec {}
