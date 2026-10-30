#[doc = "Register `REG_PWM_VREF` reader"]
pub type R = crate::R<RegPwmVrefSpec>;
#[doc = "Register `REG_PWM_VREF` writer"]
pub type W = crate::W<RegPwmVrefSpec>;
#[doc = "Field `pwm_vref_sel_norm` reader - PWM default vref"]
pub type PwmVrefSelNormR = crate::FieldReader;
#[doc = "Field `pwm_vref_sel_norm` writer - PWM default vref"]
pub type PwmVrefSelNormW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pwm_vref_sel_snoz` reader - For snooze mode power , it use pwm 0.8v and CLDO pc mode , to select PWM 0.8v , it must change vref"]
pub type PwmVrefSelSnozR = crate::FieldReader;
#[doc = "Field `pwm_vref_sel_snoz` writer - For snooze mode power , it use pwm 0.8v and CLDO pc mode , to select PWM 0.8v , it must change vref"]
pub type PwmVrefSelSnozW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PWM default vref"]
    #[inline(always)]
    pub fn pwm_vref_sel_norm(&self) -> PwmVrefSelNormR {
        PwmVrefSelNormR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - For snooze mode power , it use pwm 0.8v and CLDO pc mode , to select PWM 0.8v , it must change vref"]
    #[inline(always)]
    pub fn pwm_vref_sel_snoz(&self) -> PwmVrefSelSnozR {
        PwmVrefSelSnozR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM default vref"]
    #[inline(always)]
    pub fn pwm_vref_sel_norm(&mut self) -> PwmVrefSelNormW<'_, RegPwmVrefSpec> {
        PwmVrefSelNormW::new(self, 0)
    }
    #[doc = "Bits 2:3 - For snooze mode power , it use pwm 0.8v and CLDO pc mode , to select PWM 0.8v , it must change vref"]
    #[inline(always)]
    pub fn pwm_vref_sel_snoz(&mut self) -> PwmVrefSelSnozW<'_, RegPwmVrefSpec> {
        PwmVrefSelSnozW::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pwm_vref::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pwm_vref::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPwmVrefSpec;
impl crate::RegisterSpec for RegPwmVrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pwm_vref::R`](R) reader structure"]
impl crate::Readable for RegPwmVrefSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pwm_vref::W`](W) writer structure"]
impl crate::Writable for RegPwmVrefSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PWM_VREF to value 0x03"]
impl crate::Resettable for RegPwmVrefSpec {
    const RESET_VALUE: u32 = 0x03;
}
