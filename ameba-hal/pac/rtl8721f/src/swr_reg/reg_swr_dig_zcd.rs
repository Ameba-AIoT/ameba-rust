#[doc = "Register `REG_SWR_DIG_ZCD` reader"]
pub type R = crate::R<RegSwrDigZcdSpec>;
#[doc = "Register `REG_SWR_DIG_ZCD` writer"]
pub type W = crate::W<RegSwrDigZcdSpec>;
#[doc = "Field `SPS_DIG_ZCD_CODE` reader - SPS_DIG_ZCD_CODE,this is used for SW mode"]
pub type SpsDigZcdCodeR = crate::FieldReader;
#[doc = "Field `SPS_DIG_ZCD_CODE` writer - SPS_DIG_ZCD_CODE,this is used for SW mode"]
pub type SpsDigZcdCodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SPS_DIG_ZCD_AUTO` reader - SPS HW ZCD code auto count function enable"]
pub type SpsDigZcdAutoR = crate::BitReader;
#[doc = "Field `SPS_DIG_ZCD_AUTO` writer - SPS HW ZCD code auto count function enable"]
pub type SpsDigZcdAutoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPS_DIG_ZCD_MODE` reader - SPS ZCD SW mode enable"]
pub type SpsDigZcdModeR = crate::BitReader;
#[doc = "Field `SPS_DIG_ZCD_MODE` writer - SPS ZCD SW mode enable"]
pub type SpsDigZcdModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPS_DIG_ZCD_DBG` reader - The acture zcd code used for SPS core"]
pub type SpsDigZcdDbgR = crate::FieldReader;
#[doc = "Field `SPS_DIG_ZCD_FEN` reader - zcd function enable"]
pub type SpsDigZcdFenR = crate::BitReader;
#[doc = "Field `SPS_DIG_ZCD_FEN` writer - zcd function enable"]
pub type SpsDigZcdFenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - SPS_DIG_ZCD_CODE,this is used for SW mode"]
    #[inline(always)]
    pub fn sps_dig_zcd_code(&self) -> SpsDigZcdCodeR {
        SpsDigZcdCodeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - SPS HW ZCD code auto count function enable"]
    #[inline(always)]
    pub fn sps_dig_zcd_auto(&self) -> SpsDigZcdAutoR {
        SpsDigZcdAutoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPS ZCD SW mode enable"]
    #[inline(always)]
    pub fn sps_dig_zcd_mode(&self) -> SpsDigZcdModeR {
        SpsDigZcdModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - The acture zcd code used for SPS core"]
    #[inline(always)]
    pub fn sps_dig_zcd_dbg(&self) -> SpsDigZcdDbgR {
        SpsDigZcdDbgR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - zcd function enable"]
    #[inline(always)]
    pub fn sps_dig_zcd_fen(&self) -> SpsDigZcdFenR {
        SpsDigZcdFenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - SPS_DIG_ZCD_CODE,this is used for SW mode"]
    #[inline(always)]
    pub fn sps_dig_zcd_code(&mut self) -> SpsDigZcdCodeW<'_, RegSwrDigZcdSpec> {
        SpsDigZcdCodeW::new(self, 0)
    }
    #[doc = "Bit 8 - SPS HW ZCD code auto count function enable"]
    #[inline(always)]
    pub fn sps_dig_zcd_auto(&mut self) -> SpsDigZcdAutoW<'_, RegSwrDigZcdSpec> {
        SpsDigZcdAutoW::new(self, 8)
    }
    #[doc = "Bit 9 - SPS ZCD SW mode enable"]
    #[inline(always)]
    pub fn sps_dig_zcd_mode(&mut self) -> SpsDigZcdModeW<'_, RegSwrDigZcdSpec> {
        SpsDigZcdModeW::new(self, 9)
    }
    #[doc = "Bit 31 - zcd function enable"]
    #[inline(always)]
    pub fn sps_dig_zcd_fen(&mut self) -> SpsDigZcdFenW<'_, RegSwrDigZcdSpec> {
        SpsDigZcdFenW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_swr_dig_zcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_swr_dig_zcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwrDigZcdSpec;
impl crate::RegisterSpec for RegSwrDigZcdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_swr_dig_zcd::R`](R) reader structure"]
impl crate::Readable for RegSwrDigZcdSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_swr_dig_zcd::W`](W) writer structure"]
impl crate::Writable for RegSwrDigZcdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SWR_DIG_ZCD to value 0x0110"]
impl crate::Resettable for RegSwrDigZcdSpec {
    const RESET_VALUE: u32 = 0x0110;
}
