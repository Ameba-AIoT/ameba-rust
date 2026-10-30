#[doc = "Register `REG_CH0_CMD3_DATA` reader"]
pub type R = crate::R<RegCh0Cmd3DataSpec>;
#[doc = "Register `REG_CH0_CMD3_DATA` writer"]
pub type W = crate::W<RegCh0Cmd3DataSpec>;
#[doc = "Field `ch0_cmd3_data` reader - "]
pub type Ch0Cmd3DataR = crate::FieldReader<u32>;
#[doc = "Field `ch0_cmd3_data` writer - "]
pub type Ch0Cmd3DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_cmd3_data(&self) -> Ch0Cmd3DataR {
        Ch0Cmd3DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch0_cmd3_data(&mut self) -> Ch0Cmd3DataW<'_, RegCh0Cmd3DataSpec> {
        Ch0Cmd3DataW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch0_cmd3_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch0_cmd3_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCh0Cmd3DataSpec;
impl crate::RegisterSpec for RegCh0Cmd3DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch0_cmd3_data::R`](R) reader structure"]
impl crate::Readable for RegCh0Cmd3DataSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch0_cmd3_data::W`](W) writer structure"]
impl crate::Writable for RegCh0Cmd3DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH0_CMD3_DATA to value 0"]
impl crate::Resettable for RegCh0Cmd3DataSpec {}
