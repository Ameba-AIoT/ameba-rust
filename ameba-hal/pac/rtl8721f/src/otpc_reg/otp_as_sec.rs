#[doc = "Register `OTP_AS_SEC` reader"]
pub type R = crate::R<OtpAsSecSpec>;
#[doc = "Register `OTP_AS_SEC` writer"]
pub type W = crate::W<OtpAsSecSpec>;
#[doc = "Field `EF_DATA` reader - OTP indirect read/write data"]
pub type EfDataR = crate::FieldReader;
#[doc = "Field `EF_DATA` writer - OTP indirect read/write data"]
pub type EfDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EF_ADDR` reader - OTP indirect read/write address"]
pub type EfAddrR = crate::FieldReader<u16>;
#[doc = "Field `EF_ADDR` writer - OTP indirect read/write address"]
pub type EfAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EF_MODE_SEL` reader - OTP mode selection"]
pub type EfModeSelR = crate::FieldReader;
#[doc = "Field `EF_MODE_SEL` writer - OTP mode selection"]
pub type EfModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EF_RD_WR` reader - write triggers indirect read(0)/write(1); toggles when done"]
pub type EfRdWrR = crate::BitReader;
#[doc = "Field `EF_RD_WR` writer - write triggers indirect read(0)/write(1); toggles when done"]
pub type EfRdWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - OTP indirect read/write data"]
    #[inline(always)]
    pub fn ef_data(&self) -> EfDataR {
        EfDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - OTP indirect read/write address"]
    #[inline(always)]
    pub fn ef_addr(&self) -> EfAddrR {
        EfAddrR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 19:21 - OTP mode selection"]
    #[inline(always)]
    pub fn ef_mode_sel(&self) -> EfModeSelR {
        EfModeSelR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 31 - write triggers indirect read(0)/write(1); toggles when done"]
    #[inline(always)]
    pub fn ef_rd_wr(&self) -> EfRdWrR {
        EfRdWrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - OTP indirect read/write data"]
    #[inline(always)]
    pub fn ef_data(&mut self) -> EfDataW<'_, OtpAsSecSpec> {
        EfDataW::new(self, 0)
    }
    #[doc = "Bits 8:18 - OTP indirect read/write address"]
    #[inline(always)]
    pub fn ef_addr(&mut self) -> EfAddrW<'_, OtpAsSecSpec> {
        EfAddrW::new(self, 8)
    }
    #[doc = "Bits 19:21 - OTP mode selection"]
    #[inline(always)]
    pub fn ef_mode_sel(&mut self) -> EfModeSelW<'_, OtpAsSecSpec> {
        EfModeSelW::new(self, 19)
    }
    #[doc = "Bit 31 - write triggers indirect read(0)/write(1); toggles when done"]
    #[inline(always)]
    pub fn ef_rd_wr(&mut self) -> EfRdWrW<'_, OtpAsSecSpec> {
        EfRdWrW::new(self, 31)
    }
}
#[doc = "OTP indirect access register (Secure)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_as_sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_as_sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpAsSecSpec;
impl crate::RegisterSpec for OtpAsSecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otp_as_sec::R`](R) reader structure"]
impl crate::Readable for OtpAsSecSpec {}
#[doc = "`write(|w| ..)` method takes [`otp_as_sec::W`](W) writer structure"]
impl crate::Writable for OtpAsSecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTP_AS_SEC to value 0"]
impl crate::Resettable for OtpAsSecSpec {}
