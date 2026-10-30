#[doc = "Register `FBAUDR` reader"]
pub type R = crate::R<FbaudrSpec>;
#[doc = "Register `FBAUDR` writer"]
pub type W = crate::W<FbaudrSpec>;
#[doc = "Field `FSCKDV` reader - Indicate the divider of Fast read command when FAST_RD is setting in user mode or using fast read command in auto mode. The frequency of spi_sclk is derived from: Frequency of spi_sclk = Frequency of bus_clk/(2*FSCKDV)."]
pub type FsckdvR = crate::FieldReader<u16>;
#[doc = "Field `FSCKDV` writer - Indicate the divider of Fast read command when FAST_RD is setting in user mode or using fast read command in auto mode. The frequency of spi_sclk is derived from: Frequency of spi_sclk = Frequency of bus_clk/(2*FSCKDV)."]
pub type FsckdvW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Indicate the divider of Fast read command when FAST_RD is setting in user mode or using fast read command in auto mode. The frequency of spi_sclk is derived from: Frequency of spi_sclk = Frequency of bus_clk/(2*FSCKDV)."]
    #[inline(always)]
    pub fn fsckdv(&self) -> FsckdvR {
        FsckdvR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicate the divider of Fast read command when FAST_RD is setting in user mode or using fast read command in auto mode. The frequency of spi_sclk is derived from: Frequency of spi_sclk = Frequency of bus_clk/(2*FSCKDV)."]
    #[inline(always)]
    pub fn fsckdv(&mut self) -> FsckdvW<'_, FbaudrSpec> {
        FsckdvW::new(self, 0)
    }
}
#[doc = "Fast Baud Rate Select\n\nYou can [`read`](crate::Reg::read) this register and get [`fbaudr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbaudr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbaudrSpec;
impl crate::RegisterSpec for FbaudrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbaudr::R`](R) reader structure"]
impl crate::Readable for FbaudrSpec {}
#[doc = "`write(|w| ..)` method takes [`fbaudr::W`](W) writer structure"]
impl crate::Writable for FbaudrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FBAUDR to value 0x01"]
impl crate::Resettable for FbaudrSpec {
    const RESET_VALUE: u32 = 0x01;
}
