#[doc = "Register `CH5_DATA_INF` reader"]
pub type R = crate::R<Ch5DataInfSpec>;
#[doc = "Register `CH5_DATA_INF` writer"]
pub type W = crate::W<Ch5DataInfSpec>;
#[doc = "Field `ch5_data_ave` reader - Average of channel 5 raw code"]
pub type Ch5DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch5_data_ave` writer - Average of channel 5 raw code"]
pub type Ch5DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch5_diff_data` reader - Difference digital data of channel5 between ch5_data_ave and baseline"]
pub type Ch5DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch5_diff_data` writer - Difference digital data of channel5 between ch5_data_ave and baseline"]
pub type Ch5DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch5_diff_data_polarity` reader - Polarity of ch5_diff_data 0: ch5_data_ave<baseline 1: ch5_data_ave>=baseline"]
pub type Ch5DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch5_diff_data_polarity` writer - Polarity of ch5_diff_data 0: ch5_data_ave<baseline 1: ch5_data_ave>=baseline"]
pub type Ch5DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 5 raw code"]
    #[inline(always)]
    pub fn ch5_data_ave(&self) -> Ch5DataAveR {
        Ch5DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel5 between ch5_data_ave and baseline"]
    #[inline(always)]
    pub fn ch5_diff_data(&self) -> Ch5DiffDataR {
        Ch5DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch5_diff_data 0: ch5_data_ave<baseline 1: ch5_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch5_diff_data_polarity(&self) -> Ch5DiffDataPolarityR {
        Ch5DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 5 raw code"]
    #[inline(always)]
    pub fn ch5_data_ave(&mut self) -> Ch5DataAveW<'_, Ch5DataInfSpec> {
        Ch5DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel5 between ch5_data_ave and baseline"]
    #[inline(always)]
    pub fn ch5_diff_data(&mut self) -> Ch5DiffDataW<'_, Ch5DataInfSpec> {
        Ch5DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch5_diff_data 0: ch5_data_ave<baseline 1: ch5_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch5_diff_data_polarity(&mut self) -> Ch5DiffDataPolarityW<'_, Ch5DataInfSpec> {
        Ch5DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 5 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5DataInfSpec;
impl crate::RegisterSpec for Ch5DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch5DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch5_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch5DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH5_DATA_INF to value 0"]
impl crate::Resettable for Ch5DataInfSpec {}
