#[doc = "Register `CH2_DATA_INF` reader"]
pub type R = crate::R<Ch2DataInfSpec>;
#[doc = "Register `CH2_DATA_INF` writer"]
pub type W = crate::W<Ch2DataInfSpec>;
#[doc = "Field `ch2_data_ave` reader - Average of channel 2 raw code"]
pub type Ch2DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch2_data_ave` writer - Average of channel 2 raw code"]
pub type Ch2DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch2_diff_data` reader - Difference digital data of channel2 between ch2_data_ave and baseline"]
pub type Ch2DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch2_diff_data` writer - Difference digital data of channel2 between ch2_data_ave and baseline"]
pub type Ch2DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch2_diff_data_polarity` reader - Polarity of ch2_diff_data 0: ch2_data_ave<baseline 1: ch2_data_ave>=baseline"]
pub type Ch2DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch2_diff_data_polarity` writer - Polarity of ch2_diff_data 0: ch2_data_ave<baseline 1: ch2_data_ave>=baseline"]
pub type Ch2DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 2 raw code"]
    #[inline(always)]
    pub fn ch2_data_ave(&self) -> Ch2DataAveR {
        Ch2DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel2 between ch2_data_ave and baseline"]
    #[inline(always)]
    pub fn ch2_diff_data(&self) -> Ch2DiffDataR {
        Ch2DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch2_diff_data 0: ch2_data_ave<baseline 1: ch2_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch2_diff_data_polarity(&self) -> Ch2DiffDataPolarityR {
        Ch2DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 2 raw code"]
    #[inline(always)]
    pub fn ch2_data_ave(&mut self) -> Ch2DataAveW<'_, Ch2DataInfSpec> {
        Ch2DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel2 between ch2_data_ave and baseline"]
    #[inline(always)]
    pub fn ch2_diff_data(&mut self) -> Ch2DiffDataW<'_, Ch2DataInfSpec> {
        Ch2DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch2_diff_data 0: ch2_data_ave<baseline 1: ch2_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch2_diff_data_polarity(&mut self) -> Ch2DiffDataPolarityW<'_, Ch2DataInfSpec> {
        Ch2DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 2 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2DataInfSpec;
impl crate::RegisterSpec for Ch2DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch2DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch2DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2_DATA_INF to value 0"]
impl crate::Resettable for Ch2DataInfSpec {}
