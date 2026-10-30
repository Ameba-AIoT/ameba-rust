#[doc = "Register `TIMEOUT_MON_CFG` reader"]
pub type R = crate::R<TimeoutMonCfgSpec>;
#[doc = "Register `TIMEOUT_MON_CFG` writer"]
pub type W = crate::W<TimeoutMonCfgSpec>;
#[doc = "Field `THRESHOLD` reader - Timeout threshold"]
pub type ThresholdR = crate::FieldReader;
#[doc = "Field `THRESHOLD` writer - Timeout threshold"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GRADE` reader - Timeout grade Timeout total cycle = \\[256*(2^(grade-1) +1)*threshold\\] - inaccurate Note: the range of inaccurate = 0 ~ \\[256*(2^(grade-1) +1)-2\\]"]
pub type GradeR = crate::FieldReader;
#[doc = "Field `GRADE` writer - Timeout grade Timeout total cycle = \\[256*(2^(grade-1) +1)*threshold\\] - inaccurate Note: the range of inaccurate = 0 ~ \\[256*(2^(grade-1) +1)-2\\]"]
pub type GradeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Timeout threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Timeout grade Timeout total cycle = \\[256*(2^(grade-1) +1)*threshold\\] - inaccurate Note: the range of inaccurate = 0 ~ \\[256*(2^(grade-1) +1)-2\\]"]
    #[inline(always)]
    pub fn grade(&self) -> GradeR {
        GradeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout threshold"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, TimeoutMonCfgSpec> {
        ThresholdW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timeout grade Timeout total cycle = \\[256*(2^(grade-1) +1)*threshold\\] - inaccurate Note: the range of inaccurate = 0 ~ \\[256*(2^(grade-1) +1)-2\\]"]
    #[inline(always)]
    pub fn grade(&mut self) -> GradeW<'_, TimeoutMonCfgSpec> {
        GradeW::new(self, 16)
    }
}
#[doc = "Timeout monitor configuration register0\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_mon_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_mon_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutMonCfgSpec;
impl crate::RegisterSpec for TimeoutMonCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_mon_cfg::R`](R) reader structure"]
impl crate::Readable for TimeoutMonCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_mon_cfg::W`](W) writer structure"]
impl crate::Writable for TimeoutMonCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEOUT_MON_CFG to value 0x000f_00ff"]
impl crate::Resettable for TimeoutMonCfgSpec {
    const RESET_VALUE: u32 = 0x000f_00ff;
}
