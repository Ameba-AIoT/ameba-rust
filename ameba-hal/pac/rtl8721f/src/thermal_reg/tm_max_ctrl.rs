#[doc = "Register `TM_MAX_CTRL` reader"]
pub type R = crate::R<TmMaxCtrlSpec>;
#[doc = "Register `TM_MAX_CTRL` writer"]
pub type W = crate::W<TmMaxCtrlSpec>;
#[doc = "Field `tm_max` reader - These bits define max temperature."]
pub type TmMaxR = crate::FieldReader<u32>;
#[doc = "Field `tm_max_clr` writer - This bit defines clear max temperature * 0: not clear * 1: clear tm_max value"]
pub type TmMaxClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - These bits define max temperature."]
    #[inline(always)]
    pub fn tm_max(&self) -> TmMaxR {
        TmMaxR::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 19 - This bit defines clear max temperature * 0: not clear * 1: clear tm_max value"]
    #[inline(always)]
    pub fn tm_max_clr(&mut self) -> TmMaxClrW<'_, TmMaxCtrlSpec> {
        TmMaxClrW::new(self, 19)
    }
}
#[doc = "Thermal Meter Max Temperature Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tm_max_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tm_max_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmMaxCtrlSpec;
impl crate::RegisterSpec for TmMaxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm_max_ctrl::R`](R) reader structure"]
impl crate::Readable for TmMaxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tm_max_ctrl::W`](W) writer structure"]
impl crate::Writable for TmMaxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TM_MAX_CTRL to value 0x0004_0000"]
impl crate::Resettable for TmMaxCtrlSpec {
    const RESET_VALUE: u32 = 0x0004_0000;
}
