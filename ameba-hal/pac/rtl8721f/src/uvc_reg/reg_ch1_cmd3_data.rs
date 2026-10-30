#[doc = "Register `REG_CH1_CMD3_DATA` reader"]
pub type R = crate::R<RegCh1Cmd3DataSpec>;
#[doc = "Register `REG_CH1_CMD3_DATA` writer"]
pub type W = crate::W<RegCh1Cmd3DataSpec>;
#[doc = "Field `ch1_cmd3_data` reader - "]
pub type Ch1Cmd3DataR = crate::FieldReader<u32>;
#[doc = "Field `ch1_cmd3_data` writer - "]
pub type Ch1Cmd3DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_cmd3_data(&self) -> Ch1Cmd3DataR {
        Ch1Cmd3DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch1_cmd3_data(&mut self) -> Ch1Cmd3DataW<'_, RegCh1Cmd3DataSpec> {
        Ch1Cmd3DataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch1_cmd3_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch1_cmd3_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh1Cmd3DataSpec;
impl crate::RegisterSpec for RegCh1Cmd3DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch1_cmd3_data::R`](R) reader structure"]
impl crate::Readable for RegCh1Cmd3DataSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch1_cmd3_data::W`](W) writer structure"]
impl crate::Writable for RegCh1Cmd3DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH1_CMD3_DATA to value 0"]
impl crate::Resettable for RegCh1Cmd3DataSpec {}
