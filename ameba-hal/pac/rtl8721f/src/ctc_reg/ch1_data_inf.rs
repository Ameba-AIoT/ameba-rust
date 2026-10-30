#[doc = "Register `CH1_DATA_INF` reader"]
pub type R = crate::R<Ch1DataInfSpec>;
#[doc = "Register `CH1_DATA_INF` writer"]
pub type W = crate::W<Ch1DataInfSpec>;
#[doc = "Field `ch1_data_ave` reader - Average of channel 1 raw code"]
pub type Ch1DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch1_data_ave` writer - Average of channel 1 raw code"]
pub type Ch1DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch1_diff_data` reader - Difference digital data of channel1 between ch1_data_ave and baseline"]
pub type Ch1DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch1_diff_data` writer - Difference digital data of channel1 between ch1_data_ave and baseline"]
pub type Ch1DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch1_diff_data_polarity` reader - Polarity of ch1_diff_data 0: ch1_data_ave<baseline 1: ch1_data_ave>=baseline"]
pub type Ch1DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch1_diff_data_polarity` writer - Polarity of ch1_diff_data 0: ch1_data_ave<baseline 1: ch1_data_ave>=baseline"]
pub type Ch1DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 1 raw code"]
    #[inline(always)]
    pub fn ch1_data_ave(&self) -> Ch1DataAveR {
        Ch1DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel1 between ch1_data_ave and baseline"]
    #[inline(always)]
    pub fn ch1_diff_data(&self) -> Ch1DiffDataR {
        Ch1DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch1_diff_data 0: ch1_data_ave<baseline 1: ch1_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch1_diff_data_polarity(&self) -> Ch1DiffDataPolarityR {
        Ch1DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 1 raw code"]
    #[inline(always)]
    pub fn ch1_data_ave(&mut self) -> Ch1DataAveW<'_, Ch1DataInfSpec> {
        Ch1DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel1 between ch1_data_ave and baseline"]
    #[inline(always)]
    pub fn ch1_diff_data(&mut self) -> Ch1DiffDataW<'_, Ch1DataInfSpec> {
        Ch1DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch1_diff_data 0: ch1_data_ave<baseline 1: ch1_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch1_diff_data_polarity(&mut self) -> Ch1DiffDataPolarityW<'_, Ch1DataInfSpec> {
        Ch1DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 1 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1DataInfSpec;
impl crate::RegisterSpec for Ch1DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch1DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch1DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1_DATA_INF to value 0"]
impl crate::Resettable for Ch1DataInfSpec {}
