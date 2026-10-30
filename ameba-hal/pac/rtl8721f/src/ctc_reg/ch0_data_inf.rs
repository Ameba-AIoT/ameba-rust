#[doc = "Register `CH0_DATA_INF` reader"]
pub type R = crate::R<Ch0DataInfSpec>;
#[doc = "Register `CH0_DATA_INF` writer"]
pub type W = crate::W<Ch0DataInfSpec>;
#[doc = "Field `ch0_data_ave` reader - Average of channel 0 raw code"]
pub type Ch0DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch0_data_ave` writer - Average of channel 0 raw code"]
pub type Ch0DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch0_diff_data` reader - Difference digital data of channel0 between ch0_data_ave and baseline"]
pub type Ch0DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch0_diff_data` writer - Difference digital data of channel0 between ch0_data_ave and baseline"]
pub type Ch0DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch0_diff_data_polarity` reader - Polarity of ch0_diff_data 0: ch0_data_ave<baseline 1: ch0_data_ave>=baseline"]
pub type Ch0DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch0_diff_data_polarity` writer - Polarity of ch0_diff_data 0: ch0_data_ave<baseline 1: ch0_data_ave>=baseline"]
pub type Ch0DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 0 raw code"]
    #[inline(always)]
    pub fn ch0_data_ave(&self) -> Ch0DataAveR {
        Ch0DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel0 between ch0_data_ave and baseline"]
    #[inline(always)]
    pub fn ch0_diff_data(&self) -> Ch0DiffDataR {
        Ch0DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch0_diff_data 0: ch0_data_ave<baseline 1: ch0_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch0_diff_data_polarity(&self) -> Ch0DiffDataPolarityR {
        Ch0DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 0 raw code"]
    #[inline(always)]
    pub fn ch0_data_ave(&mut self) -> Ch0DataAveW<'_, Ch0DataInfSpec> {
        Ch0DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel0 between ch0_data_ave and baseline"]
    #[inline(always)]
    pub fn ch0_diff_data(&mut self) -> Ch0DiffDataW<'_, Ch0DataInfSpec> {
        Ch0DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch0_diff_data 0: ch0_data_ave<baseline 1: ch0_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch0_diff_data_polarity(&mut self) -> Ch0DiffDataPolarityW<'_, Ch0DataInfSpec> {
        Ch0DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 0 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0DataInfSpec;
impl crate::RegisterSpec for Ch0DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch0DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch0DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0_DATA_INF to value 0"]
impl crate::Resettable for Ch0DataInfSpec {}
