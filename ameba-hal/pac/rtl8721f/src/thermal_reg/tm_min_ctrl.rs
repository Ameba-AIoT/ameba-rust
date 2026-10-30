#[doc = "Register `TM_MIN_CTRL` reader"]
pub type R = crate::R<TmMinCtrlSpec>;
#[doc = "Register `TM_MIN_CTRL` writer"]
pub type W = crate::W<TmMinCtrlSpec>;
#[doc = "Field `tm_min` reader - These bits define min temperature."]
pub type TmMinR = crate::FieldReader<u32>;
#[doc = "Field `tm_min_clr` writer - This bit defines clear min temperature * 0: not clear * 1: clear tm_min value"]
pub type TmMinClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - These bits define min temperature."]
    #[inline(always)]
    pub fn tm_min(&self) -> TmMinR {
        TmMinR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 19 - This bit defines clear min temperature * 0: not clear * 1: clear tm_min value"]
    #[inline(always)]
    pub fn tm_min_clr(&mut self) -> TmMinClrW<'_, TmMinCtrlSpec> {
        TmMinClrW::new(self, 19)
    }
}
#[doc = "Thermal Meter Min Temperature Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_min_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_min_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmMinCtrlSpec;
impl crate::RegisterSpec for TmMinCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_min_ctrl::R`](R) reader structure"]
impl crate::Readable for TmMinCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_min_ctrl::W`](W) writer structure"]
impl crate::Writable for TmMinCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_MIN_CTRL to value 0x0003_ffff"]
impl crate::Resettable for TmMinCtrlSpec {
    const RESET_VALUE: u32 = 0x0003_ffff;
}
