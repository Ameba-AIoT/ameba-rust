#[doc = "Register `REG_OTP_HWCFG` reader"]
pub type R = crate::R<RegOtpHwcfgSpec>;
#[doc = "Register `REG_OTP_HWCFG` writer"]
pub type W = crate::W<RegOtpHwcfgSpec>;
#[doc = "Field `SW_RSVD6` reader - RSVD for SW usage"]
pub type SwRsvd6R = crate::FieldReader<u32>;
#[doc = "Field `SW_RSVD6` writer - RSVD for SW usage"]
pub type SwRsvd6W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `boot_vol_sel` reader - BOOT Voltage Selection: 0'b0: 0.9V 0'b1: 1.0V"]
pub type BootVolSelR = crate::BitReader;
#[doc = "Field `boot_vol_sel` writer - BOOT Voltage Selection: 0'b0: 0.9V 0'b1: 1.0V"]
pub type BootVolSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd6(&self) -> SwRsvd6R {
        SwRsvd6R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - BOOT Voltage Selection: 0'b0: 0.9V 0'b1: 1.0V"]
    #[inline(always)]
    pub fn boot_vol_sel(&self) -> BootVolSelR {
        BootVolSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - RSVD for SW usage"]
    #[inline(always)]
    pub fn sw_rsvd6(&mut self) -> SwRsvd6W<'_, RegOtpHwcfgSpec> {
        SwRsvd6W::new(self, 0)
    }
    #[doc = "Bit 31 - BOOT Voltage Selection: 0'b0: 0.9V 0'b1: 1.0V"]
    #[inline(always)]
    pub fn boot_vol_sel(&mut self) -> BootVolSelW<'_, RegOtpHwcfgSpec> {
        BootVolSelW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_otp_hwcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_otp_hwcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtpHwcfgSpec;
impl crate::RegisterSpec for RegOtpHwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_otp_hwcfg::R`](R) reader structure"]
impl crate::Readable for RegOtpHwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_otp_hwcfg::W`](W) writer structure"]
impl crate::Writable for RegOtpHwcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OTP_HWCFG to value 0"]
impl crate::Resettable for RegOtpHwcfgSpec {}
