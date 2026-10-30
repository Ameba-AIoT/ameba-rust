#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BaudrSpec>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BaudrSpec>;
#[doc = "Field `SCKDV` reader - SPI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the equation: Fsclk_out =Fssi_clk/SCKDV, where SCKDV is any even value between 2 and 65534."]
pub type SckdvR = crate::FieldReader<u16>;
#[doc = "Field `SCKDV` writer - SPI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the equation: Fsclk_out =Fssi_clk/SCKDV, where SCKDV is any even value between 2 and 65534."]
pub type SckdvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SPI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the equation: Fsclk_out =Fssi_clk/SCKDV, where SCKDV is any even value between 2 and 65534."]
    #[inline(always)]
    pub fn sckdv(&self) -> SckdvR {
        SckdvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI Clock Divider. The LSB for this field is always set to 0 and is unaffected by a write operation, which ensures an even value is held in this register. If the value is 0, the serial output clock (sclk_out) is disabled. The frequency of the sclk_out is derived from the equation: Fsclk_out =Fssi_clk/SCKDV, where SCKDV is any even value between 2 and 65534."]
    #[inline(always)]
    pub fn sckdv(&mut self) -> SckdvW<'_, BaudrSpec> {
        SckdvW::new(self, 0)
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
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BaudrSpec {}
