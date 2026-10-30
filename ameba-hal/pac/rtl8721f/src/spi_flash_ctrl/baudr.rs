#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BaudrSpec>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BaudrSpec>;
#[doc = "Field `SCKDV` reader - Define spi_sclk divider value. The frequency of spi_sclk is derived from: The frequency of spi_sclk = The frequency of bus_clk/(2*SCKDV)."]
pub type SckdvR = crate::FieldReader<u16>;
#[doc = "Field `SCKDV` writer - Define spi_sclk divider value. The frequency of spi_sclk is derived from: The frequency of spi_sclk = The frequency of bus_clk/(2*SCKDV)."]
pub type SckdvW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CKPS` reader - Indicate the shift range of device clk (SCLK) when CKPS_EN is set. (unit: bus_clk) Exists: TAG >= 2.0.8."]
pub type CkpsR = crate::FieldReader;
#[doc = "Field `CKPS` writer - Indicate the shift range of device clk (SCLK) when CKPS_EN is set. (unit: bus_clk) Exists: TAG >= 2.0.8."]
pub type CkpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKPS_EN` reader - CKPS enable bit. Set this bit to use CKPS feature. Exists: TAG >= 2.0.8."]
pub type CkpsEnR = crate::BitReader;
#[doc = "Field `CKPS_EN` writer - CKPS enable bit. Set this bit to use CKPS feature. Exists: TAG >= 2.0.8."]
pub type CkpsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Define spi_sclk divider value. The frequency of spi_sclk is derived from: The frequency of spi_sclk = The frequency of bus_clk/(2*SCKDV)."]
    #[inline(always)]
    pub fn sckdv(&self) -> SckdvR {
        SckdvR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Indicate the shift range of device clk (SCLK) when CKPS_EN is set. (unit: bus_clk) Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn ckps(&self) -> CkpsR {
        CkpsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - CKPS enable bit. Set this bit to use CKPS feature. Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn ckps_en(&self) -> CkpsEnR {
        CkpsEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Define spi_sclk divider value. The frequency of spi_sclk is derived from: The frequency of spi_sclk = The frequency of bus_clk/(2*SCKDV)."]
    #[inline(always)]
    pub fn sckdv(&mut self) -> SckdvW<'_, BaudrSpec> {
        SckdvW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Indicate the shift range of device clk (SCLK) when CKPS_EN is set. (unit: bus_clk) Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn ckps(&mut self) -> CkpsW<'_, BaudrSpec> {
        CkpsW::new(self, 12)
    }
    #[doc = "Bit 15 - CKPS enable bit. Set this bit to use CKPS feature. Exists: TAG >= 2.0.8."]
    #[inline(always)]
    pub fn ckps_en(&mut self) -> CkpsEnW<'_, BaudrSpec> {
        CkpsEnW::new(self, 15)
    }
}
#[doc = "Baud Rate Select\n\nYou can [`read`](crate::Reg::read) this register and get [`baudr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudrSpec;
impl crate::RegisterSpec for BaudrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BaudrSpec {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BaudrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAUDR to value 0x14"]
impl crate::Resettable for BaudrSpec {
    const RESET_VALUE: u32 = 0x14;
}
