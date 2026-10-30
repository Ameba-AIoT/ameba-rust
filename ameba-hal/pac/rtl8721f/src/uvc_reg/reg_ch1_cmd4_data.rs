#[doc = "Register `REG_CH1_CMD4_DATA` reader"]
pub type R = crate::R<RegCh1Cmd4DataSpec>;
#[doc = "Register `REG_CH1_CMD4_DATA` writer"]
pub type W = crate::W<RegCh1Cmd4DataSpec>;
#[doc = "Field `ch1_cmd4_data` reader - "]
pub type Ch1Cmd4DataR = crate::FieldReader<u32>;
#[doc = "Field `ch1_cmd4_data` writer - "]
pub type Ch1Cmd4DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_cmd4_data(&self) -> Ch1Cmd4DataR {
        Ch1Cmd4DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_cmd4_data(&mut self) -> Ch1Cmd4DataW<'_, RegCh1Cmd4DataSpec> {
        Ch1Cmd4DataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_cmd4_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch1_cmd4_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Cmd4DataSpec;
impl crate::RegisterSpec for RegCh1Cmd4DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_cmd4_data::R`](R) reader structure"]
impl crate::Readable for RegCh1Cmd4DataSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch1_cmd4_data::W`](W) writer structure"]
impl crate::Writable for RegCh1Cmd4DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH1_CMD4_DATA to value 0"]
impl crate::Resettable for RegCh1Cmd4DataSpec {}
