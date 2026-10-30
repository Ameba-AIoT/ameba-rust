#[doc = "Register `CH7_DATA_INF` reader"]
pub type R = crate::R<Ch7DataInfSpec>;
#[doc = "Register `CH7_DATA_INF` writer"]
pub type W = crate::W<Ch7DataInfSpec>;
#[doc = "Field `ch7_data_ave` reader - Average of channel 7 raw code"]
pub type Ch7DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch7_data_ave` writer - Average of channel 7 raw code"]
pub type Ch7DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch7_diff_data` reader - Difference digital data of channel7 between ch7_data_ave and baseline"]
pub type Ch7DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch7_diff_data` writer - Difference digital data of channel7 between ch7_data_ave and baseline"]
pub type Ch7DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch7_diff_data_polarity` reader - Polarity of ch7_diff_data 0: ch7_data_ave<baseline 1: ch7_data_ave>=baseline"]
pub type Ch7DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch7_diff_data_polarity` writer - Polarity of ch7_diff_data 0: ch7_data_ave<baseline 1: ch7_data_ave>=baseline"]
pub type Ch7DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 7 raw code"]
    #[inline(always)]
    pub fn ch7_data_ave(&self) -> Ch7DataAveR {
        Ch7DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel7 between ch7_data_ave and baseline"]
    #[inline(always)]
    pub fn ch7_diff_data(&self) -> Ch7DiffDataR {
        Ch7DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch7_diff_data 0: ch7_data_ave<baseline 1: ch7_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch7_diff_data_polarity(&self) -> Ch7DiffDataPolarityR {
        Ch7DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 7 raw code"]
    #[inline(always)]
    pub fn ch7_data_ave(&mut self) -> Ch7DataAveW<'_, Ch7DataInfSpec> {
        Ch7DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel7 between ch7_data_ave and baseline"]
    #[inline(always)]
    pub fn ch7_diff_data(&mut self) -> Ch7DiffDataW<'_, Ch7DataInfSpec> {
        Ch7DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch7_diff_data 0: ch7_data_ave<baseline 1: ch7_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch7_diff_data_polarity(&mut self) -> Ch7DiffDataPolarityW<'_, Ch7DataInfSpec> {
        Ch7DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 7 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7DataInfSpec;
impl crate::RegisterSpec for Ch7DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch7DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch7DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_DATA_INF to value 0"]
impl crate::Resettable for Ch7DataInfSpec {}
