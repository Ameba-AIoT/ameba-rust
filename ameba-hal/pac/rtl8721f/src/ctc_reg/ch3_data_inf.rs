#[doc = "Register `CH3_DATA_INF` reader"]
pub type R = crate::R<Ch3DataInfSpec>;
#[doc = "Register `CH3_DATA_INF` writer"]
pub type W = crate::W<Ch3DataInfSpec>;
#[doc = "Field `ch3_data_ave` reader - Average of channel 3 raw code"]
pub type Ch3DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch3_data_ave` writer - Average of channel 3 raw code"]
pub type Ch3DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch3_diff_data` reader - Difference digital data of channel3 between ch3_data_ave and baseline"]
pub type Ch3DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch3_diff_data` writer - Difference digital data of channel3 between ch3_data_ave and baseline"]
pub type Ch3DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch3_diff_data_polarity` reader - Polarity of ch3_diff_data 0: ch3_data_ave<baseline 1: ch3_data_ave>=baseline"]
pub type Ch3DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch3_diff_data_polarity` writer - Polarity of ch3_diff_data 0: ch3_data_ave<baseline 1: ch3_data_ave>=baseline"]
pub type Ch3DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 3 raw code"]
    #[inline(always)]
    pub fn ch3_data_ave(&self) -> Ch3DataAveR {
        Ch3DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel3 between ch3_data_ave and baseline"]
    #[inline(always)]
    pub fn ch3_diff_data(&self) -> Ch3DiffDataR {
        Ch3DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch3_diff_data 0: ch3_data_ave<baseline 1: ch3_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch3_diff_data_polarity(&self) -> Ch3DiffDataPolarityR {
        Ch3DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 3 raw code"]
    #[inline(always)]
    pub fn ch3_data_ave(&mut self) -> Ch3DataAveW<'_, Ch3DataInfSpec> {
        Ch3DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel3 between ch3_data_ave and baseline"]
    #[inline(always)]
    pub fn ch3_diff_data(&mut self) -> Ch3DiffDataW<'_, Ch3DataInfSpec> {
        Ch3DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch3_diff_data 0: ch3_data_ave<baseline 1: ch3_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch3_diff_data_polarity(&mut self) -> Ch3DiffDataPolarityW<'_, Ch3DataInfSpec> {
        Ch3DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 3 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3DataInfSpec;
impl crate::RegisterSpec for Ch3DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch3DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch3DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH3_DATA_INF to value 0"]
impl crate::Resettable for Ch3DataInfSpec {}
