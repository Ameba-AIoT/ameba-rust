#[doc = "Register `CH4_DATA_INF` reader"]
pub type R = crate::R<Ch4DataInfSpec>;
#[doc = "Register `CH4_DATA_INF` writer"]
pub type W = crate::W<Ch4DataInfSpec>;
#[doc = "Field `ch4_data_ave` reader - Average of channel 4 raw code"]
pub type Ch4DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch4_data_ave` writer - Average of channel 4 raw code"]
pub type Ch4DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch4_diff_data` reader - Difference digital data of channel4 between ch4_data_ave and baseline"]
pub type Ch4DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch4_diff_data` writer - Difference digital data of channel4 between ch4_data_ave and baseline"]
pub type Ch4DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch4_diff_data_polarity` reader - Polarity of ch4_diff_data 0: ch4_data_ave<baseline 1: ch4_data_ave>=baseline"]
pub type Ch4DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch4_diff_data_polarity` writer - Polarity of ch4_diff_data 0: ch4_data_ave<baseline 1: ch4_data_ave>=baseline"]
pub type Ch4DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 4 raw code"]
    #[inline(always)]
    pub fn ch4_data_ave(&self) -> Ch4DataAveR {
        Ch4DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel4 between ch4_data_ave and baseline"]
    #[inline(always)]
    pub fn ch4_diff_data(&self) -> Ch4DiffDataR {
        Ch4DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch4_diff_data 0: ch4_data_ave<baseline 1: ch4_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch4_diff_data_polarity(&self) -> Ch4DiffDataPolarityR {
        Ch4DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 4 raw code"]
    #[inline(always)]
    pub fn ch4_data_ave(&mut self) -> Ch4DataAveW<'_, Ch4DataInfSpec> {
        Ch4DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel4 between ch4_data_ave and baseline"]
    #[inline(always)]
    pub fn ch4_diff_data(&mut self) -> Ch4DiffDataW<'_, Ch4DataInfSpec> {
        Ch4DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch4_diff_data 0: ch4_data_ave<baseline 1: ch4_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch4_diff_data_polarity(&mut self) -> Ch4DiffDataPolarityW<'_, Ch4DataInfSpec> {
        Ch4DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 4 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4DataInfSpec;
impl crate::RegisterSpec for Ch4DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch4DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch4DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4_DATA_INF to value 0"]
impl crate::Resettable for Ch4DataInfSpec {}
