#[doc = "Register `REG_VO_VI_ADMTIME` reader"]
pub type R = crate::R<RegVoViAdmtimeSpec>;
#[doc = "Register `REG_VO_VI_ADMTIME` writer"]
pub type W = crate::W<RegVoViAdmtimeSpec>;
#[doc = "Field `BIT_VO_ADMITTED_TIME` reader - Admitted_time of VO queue. In unit of 32 microsecond. Calculated and set by driver."]
pub type BitVoAdmittedTimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_VO_ADMITTED_TIME` writer - Admitted_time of VO queue. In unit of 32 microsecond. Calculated and set by driver."]
pub type BitVoAdmittedTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_VI_ADMITTED_TIME` reader - Admitted_time of VI queue. In unit of 32 microsecond. Calculated and set by driver."]
pub type BitViAdmittedTimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_VI_ADMITTED_TIME` writer - Admitted_time of VI queue. In unit of 32 microsecond. Calculated and set by driver."]
pub type BitViAdmittedTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Admitted_time of VO queue. In unit of 32 microsecond. Calculated and set by driver."]
    #[inline(always)]
    pub fn bit_vo_admitted_time(&self) -> BitVoAdmittedTimeR {
        BitVoAdmittedTimeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Admitted_time of VI queue. In unit of 32 microsecond. Calculated and set by driver."]
    #[inline(always)]
    pub fn bit_vi_admitted_time(&self) -> BitViAdmittedTimeR {
        BitViAdmittedTimeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Admitted_time of VO queue. In unit of 32 microsecond. Calculated and set by driver."]
    #[inline(always)]
    pub fn bit_vo_admitted_time(&mut self) -> BitVoAdmittedTimeW<'_, RegVoViAdmtimeSpec> {
        BitVoAdmittedTimeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Admitted_time of VI queue. In unit of 32 microsecond. Calculated and set by driver."]
    #[inline(always)]
    pub fn bit_vi_admitted_time(&mut self) -> BitViAdmittedTimeW<'_, RegVoViAdmtimeSpec> {
        BitViAdmittedTimeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_vo_vi_admtime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_vo_vi_admtime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegVoViAdmtimeSpec;
impl crate::RegisterSpec for RegVoViAdmtimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_vo_vi_admtime::R`](R) reader structure"]
impl crate::Readable for RegVoViAdmtimeSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_vo_vi_admtime::W`](W) writer structure"]
impl crate::Writable for RegVoViAdmtimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_VO_VI_ADMTIME to value 0"]
impl crate::Resettable for RegVoViAdmtimeSpec {}
