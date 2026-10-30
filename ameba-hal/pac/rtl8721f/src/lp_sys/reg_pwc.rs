#[doc = "Register `REG_PWC` reader"]
pub type R = crate::R<RegPwcSpec>;
#[doc = "Register `REG_PWC` writer"]
pub type W = crate::W<RegPwcSpec>;
#[doc = "Field `pow_on_soc` reader - 1:SYS PMC request to enable HP SOC power cut and diable ISO 0: SYS PMC request to disable HP SOC power cut and enable ISO"]
pub type PowOnSocR = crate::BitReader;
#[doc = "Field `pow_on_soc` writer - 1:SYS PMC request to enable HP SOC power cut and diable ISO 0: SYS PMC request to disable HP SOC power cut and enable ISO"]
pub type PowOnSocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwc_bt` reader - This is BT function total power cut control 1: Enable BT power cut 0: Disable"]
pub type PwcBtR = crate::BitReader;
#[doc = "Field `pwc_bt` writer - This is BT function total power cut control 1: Enable BT power cut 0: Disable"]
pub type PwcBtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwc_shrm` reader - BT share mem power cut control 1: Enable BT share mem power (mux + mbis + cell) 0: Disable"]
pub type PwcShrmR = crate::BitReader;
#[doc = "Field `pwc_shrm` writer - BT share mem power cut control 1: Enable BT share mem power (mux + mbis + cell) 0: Disable"]
pub type PwcShrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwc_soc_st` reader - pwc soc arb result state"]
pub type PwcSocStR = crate::FieldReader;
#[doc = "Field `pwc_soc_req` reader - pwc soc req arb result"]
pub type PwcSocReqR = crate::BitReader;
#[doc = "Field `psw_out_soc` reader - SOC domain power switch status 1: power on 0: power off Note:there are two group"]
pub type PswOutSocR = crate::FieldReader;
#[doc = "Field `psw_out_bt` reader - BT domain power switch status 1: power on 0: power off Note:there are two group"]
pub type PswOutBtR = crate::FieldReader;
#[doc = "Field `psw_out_shrm` reader - ShareMem domain power switch status 1: power on 0: power off Note:there are two group"]
pub type PswOutShrmR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 1:SYS PMC request to enable HP SOC power cut and diable ISO 0: SYS PMC request to disable HP SOC power cut and enable ISO"]
    #[inline(always)]
    pub fn pow_on_soc(&self) -> PowOnSocR {
        PowOnSocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is BT function total power cut control 1: Enable BT power cut 0: Disable"]
    #[inline(always)]
    pub fn pwc_bt(&self) -> PwcBtR {
        PwcBtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BT share mem power cut control 1: Enable BT share mem power (mux + mbis + cell) 0: Disable"]
    #[inline(always)]
    pub fn pwc_shrm(&self) -> PwcShrmR {
        PwcShrmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - pwc soc arb result state"]
    #[inline(always)]
    pub fn pwc_soc_st(&self) -> PwcSocStR {
        PwcSocStR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - pwc soc req arb result"]
    #[inline(always)]
    pub fn pwc_soc_req(&self) -> PwcSocReqR {
        PwcSocReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SOC domain power switch status 1: power on 0: power off Note:there are two group"]
    #[inline(always)]
    pub fn psw_out_soc(&self) -> PswOutSocR {
        PswOutSocR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BT domain power switch status 1: power on 0: power off Note:there are two group"]
    #[inline(always)]
    pub fn psw_out_bt(&self) -> PswOutBtR {
        PswOutBtR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ShareMem domain power switch status 1: power on 0: power off Note:there are two group"]
    #[inline(always)]
    pub fn psw_out_shrm(&self) -> PswOutShrmR {
        PswOutShrmR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1:SYS PMC request to enable HP SOC power cut and diable ISO 0: SYS PMC request to disable HP SOC power cut and enable ISO"]
    #[inline(always)]
    pub fn pow_on_soc(&mut self) -> PowOnSocW<'_, RegPwcSpec> {
        PowOnSocW::new(self, 0)
    }
    #[doc = "Bit 1 - This is BT function total power cut control 1: Enable BT power cut 0: Disable"]
    #[inline(always)]
    pub fn pwc_bt(&mut self) -> PwcBtW<'_, RegPwcSpec> {
        PwcBtW::new(self, 1)
    }
    #[doc = "Bit 2 - BT share mem power cut control 1: Enable BT share mem power (mux + mbis + cell) 0: Disable"]
    #[inline(always)]
    pub fn pwc_shrm(&mut self) -> PwcShrmW<'_, RegPwcSpec> {
        PwcShrmW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pwc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pwc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPwcSpec;
impl crate::RegisterSpec for RegPwcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_pwc::R`](R) reader structure"]
impl crate::Readable for RegPwcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pwc::W`](W) writer structure"]
impl crate::Writable for RegPwcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PWC to value 0"]
impl crate::Resettable for RegPwcSpec {}
