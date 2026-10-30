#[doc = "Register `MSTICR` reader"]
pub type R = crate::R<MsticrSpec>;
#[doc = "Register `MSTICR` writer"]
pub type W = crate::W<MsticrSpec>;
#[doc = "Field `MSTICR` reader - When reading/writing the register, spi_mstir_r would be cleared. Note: Only response value 0 when read."]
pub type MsticrR = crate::BitReader;
#[doc = "Field `MSTICR` writer - When reading/writing the register, spi_mstir_r would be cleared. Note: Only response value 0 when read."]
pub type MsticrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When reading/writing the register, spi_mstir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn msticr(&self) -> MsticrR {
        MsticrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When reading/writing the register, spi_mstir_r would be cleared. Note: Only response value 0 when read."]
    #[inline(always)]
    pub fn msticr(&mut self) -> MsticrW<'_, MsticrSpec> {
        MsticrW::new(self, 0)
    }
}
#[doc = "Master error Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msticr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msticr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsticrSpec;
impl crate::RegisterSpec for MsticrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msticr::R`](R) reader structure"]
impl crate::Readable for MsticrSpec {}
#[doc = "`write(|w| ..)` method takes [`msticr::W`](W) writer structure"]
impl crate::Writable for MsticrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSTICR to value 0"]
impl crate::Resettable for MsticrSpec {}
