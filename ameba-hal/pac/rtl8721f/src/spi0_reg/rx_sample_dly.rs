#[doc = "Register `RX_SAMPLE_DLY` reader"]
pub type R = crate::R<RxSampleDlySpec>;
#[doc = "Register `RX_SAMPLE_DLY` writer"]
pub type W = crate::W<RxSampleDlySpec>;
#[doc = "Field `RSD` reader - Receive Data (rxd) Sample Delay. This register is used to delay the sample of the rxd input signal. Each value represents a single ssi_clk delay on the sample of the rxd signal."]
pub type RsdR = crate::FieldReader;
#[doc = "Field `RSD` writer - Receive Data (rxd) Sample Delay. This register is used to delay the sample of the rxd input signal. Each value represents a single ssi_clk delay on the sample of the rxd signal."]
pub type RsdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Data (rxd) Sample Delay. This register is used to delay the sample of the rxd input signal. Each value represents a single ssi_clk delay on the sample of the rxd signal."]
    #[inline(always)]
    pub fn rsd(&self) -> RsdR {
        RsdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Data (rxd) Sample Delay. This register is used to delay the sample of the rxd input signal. Each value represents a single ssi_clk delay on the sample of the rxd signal."]
    #[inline(always)]
    pub fn rsd(&mut self) -> RsdW<'_, RxSampleDlySpec> {
        RsdW::new(self, 0)
    }
}
#[doc = "rxd Sample Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_sample_dly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_sample_dly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxSampleDlySpec;
impl crate::RegisterSpec for RxSampleDlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_sample_dly::R`](R) reader structure"]
impl crate::Readable for RxSampleDlySpec {}
#[doc = "`write(|w| ..)` method takes [`rx_sample_dly::W`](W) writer structure"]
impl crate::Writable for RxSampleDlySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_SAMPLE_DLY to value 0"]
impl crate::Resettable for RxSampleDlySpec {}
