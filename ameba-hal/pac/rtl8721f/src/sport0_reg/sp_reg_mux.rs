#[doc = "Register `SP_REG_MUX` reader"]
pub type R = crate::R<SpRegMuxSpec>;
#[doc = "Register `SP_REG_MUX` writer"]
pub type W = crate::W<SpRegMuxSpec>;
#[doc = "Field `sp_reg_mux` reader - Mux of register write with different base address of the same SPORT. \"sp_reg_mux\" can be set as different value with four different base address in one SPORT, but other registers share the same value with four different base address in one SPORT."]
pub type SpRegMuxR = crate::FieldReader<u32>;
#[doc = "Field `sp_reg_mux` writer - Mux of register write with different base address of the same SPORT. \"sp_reg_mux\" can be set as different value with four different base address in one SPORT, but other registers share the same value with four different base address in one SPORT."]
pub type SpRegMuxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mux of register write with different base address of the same SPORT. \"sp_reg_mux\" can be set as different value with four different base address in one SPORT, but other registers share the same value with four different base address in one SPORT."]
    #[inline(always)]
    pub fn sp_reg_mux(&self) -> SpRegMuxR {
        SpRegMuxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mux of register write with different base address of the same SPORT. \"sp_reg_mux\" can be set as different value with four different base address in one SPORT, but other registers share the same value with four different base address in one SPORT."]
    #[inline(always)]
    pub fn sp_reg_mux(&mut self) -> SpRegMuxW<'_, SpRegMuxSpec> {
        SpRegMuxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_reg_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_reg_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpRegMuxSpec;
impl crate::RegisterSpec for SpRegMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_reg_mux::R`](R) reader structure"]
impl crate::Readable for SpRegMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`sp_reg_mux::W`](W) writer structure"]
impl crate::Writable for SpRegMuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_REG_MUX to value 0xffff_ffff"]
impl crate::Resettable for SpRegMuxSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
