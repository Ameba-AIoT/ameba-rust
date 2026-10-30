#[doc = "Register `REG_OTP_POWUP_CFG` reader"]
pub type R = crate::R<RegOtpPowupCfgSpec>;
#[doc = "Register `REG_OTP_POWUP_CFG` writer"]
pub type W = crate::W<RegOtpPowupCfgSpec>;
#[doc = "Field `otp_powup_cfg` reader - 0\\] : penvddr2 \\[1\\]: pldo \\[2\\]: pdstb \\[3\\]: ptrim \\[4\\]: pce"]
pub type OtpPowupCfgR = crate::FieldReader;
#[doc = "Field `otp_powup_cfg` writer - 0\\] : penvddr2 \\[1\\]: pldo \\[2\\]: pdstb \\[3\\]: ptrim \\[4\\]: pce"]
pub type OtpPowupCfgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `otp_powup_sel` reader - select otp pow up flow is inside control or outside control 0: inside 1: outside"]
pub type OtpPowupSelR = crate::BitReader;
#[doc = "Field `otp_powup_sel` writer - select otp pow up flow is inside control or outside control 0: inside 1: outside"]
pub type OtpPowupSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `otp_pclk_period` reader - it is used for otp PCLK period when read operation. it need >=25ns.it is timed by cnt under otp clk. for example . when otp_clk is 4MHz: 0:250ns,meet the require 1:500ns,meet the require"]
pub type OtpPclkPeriodR = crate::FieldReader;
#[doc = "Field `otp_pclk_period` writer - it is used for otp PCLK period when read operation. it need >=25ns.it is timed by cnt under otp clk. for example . when otp_clk is 4MHz: 0:250ns,meet the require 1:500ns,meet the require"]
pub type OtpPclkPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 0\\] : penvddr2 \\[1\\]: pldo \\[2\\]: pdstb \\[3\\]: ptrim \\[4\\]: pce"]
    #[inline(always)]
    pub fn otp_powup_cfg(&self) -> OtpPowupCfgR {
        OtpPowupCfgR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - select otp pow up flow is inside control or outside control 0: inside 1: outside"]
    #[inline(always)]
    pub fn otp_powup_sel(&self) -> OtpPowupSelR {
        OtpPowupSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - it is used for otp PCLK period when read operation. it need >=25ns.it is timed by cnt under otp clk. for example . when otp_clk is 4MHz: 0:250ns,meet the require 1:500ns,meet the require"]
    #[inline(always)]
    pub fn otp_pclk_period(&self) -> OtpPclkPeriodR {
        OtpPclkPeriodR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0\\] : penvddr2 \\[1\\]: pldo \\[2\\]: pdstb \\[3\\]: ptrim \\[4\\]: pce"]
    #[inline(always)]
    pub fn otp_powup_cfg(&mut self) -> OtpPowupCfgW<'_, RegOtpPowupCfgSpec> {
        OtpPowupCfgW::new(self, 0)
    }
    #[doc = "Bit 7 - select otp pow up flow is inside control or outside control 0: inside 1: outside"]
    #[inline(always)]
    pub fn otp_powup_sel(&mut self) -> OtpPowupSelW<'_, RegOtpPowupCfgSpec> {
        OtpPowupSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - it is used for otp PCLK period when read operation. it need >=25ns.it is timed by cnt under otp clk. for example . when otp_clk is 4MHz: 0:250ns,meet the require 1:500ns,meet the require"]
    #[inline(always)]
    pub fn otp_pclk_period(&mut self) -> OtpPclkPeriodW<'_, RegOtpPowupCfgSpec> {
        OtpPclkPeriodW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_otp_powup_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_otp_powup_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegOtpPowupCfgSpec;
impl crate::RegisterSpec for RegOtpPowupCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_otp_powup_cfg::R`](R) reader structure"]
impl crate::Readable for RegOtpPowupCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_otp_powup_cfg::W`](W) writer structure"]
impl crate::Writable for RegOtpPowupCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_OTP_POWUP_CFG to value 0x0100"]
impl crate::Resettable for RegOtpPowupCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
