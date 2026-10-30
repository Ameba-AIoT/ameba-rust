#[doc = "Register `CH6_DATA_INF` reader"]
pub type R = crate::R<Ch6DataInfSpec>;
#[doc = "Register `CH6_DATA_INF` writer"]
pub type W = crate::W<Ch6DataInfSpec>;
#[doc = "Field `ch6_data_ave` reader - Average of channel 6 raw code"]
pub type Ch6DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch6_data_ave` writer - Average of channel 6 raw code"]
pub type Ch6DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch6_diff_data` reader - Difference digital data of channel6 between ch6_data_ave and baseline"]
pub type Ch6DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch6_diff_data` writer - Difference digital data of channel6 between ch6_data_ave and baseline"]
pub type Ch6DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch6_diff_data_polarity` reader - Polarity of ch6_diff_data 0: ch6_data_ave<baseline 1: ch6_data_ave>=baseline"]
pub type Ch6DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch6_diff_data_polarity` writer - Polarity of ch6_diff_data 0: ch6_data_ave<baseline 1: ch6_data_ave>=baseline"]
pub type Ch6DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 6 raw code"]
    #[inline(always)]
    pub fn ch6_data_ave(&self) -> Ch6DataAveR {
        Ch6DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel6 between ch6_data_ave and baseline"]
    #[inline(always)]
    pub fn ch6_diff_data(&self) -> Ch6DiffDataR {
        Ch6DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch6_diff_data 0: ch6_data_ave<baseline 1: ch6_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch6_diff_data_polarity(&self) -> Ch6DiffDataPolarityR {
        Ch6DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 6 raw code"]
    #[inline(always)]
    pub fn ch6_data_ave(&mut self) -> Ch6DataAveW<'_, Ch6DataInfSpec> {
        Ch6DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel6 between ch6_data_ave and baseline"]
    #[inline(always)]
    pub fn ch6_diff_data(&mut self) -> Ch6DiffDataW<'_, Ch6DataInfSpec> {
        Ch6DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch6_diff_data 0: ch6_data_ave<baseline 1: ch6_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch6_diff_data_polarity(&mut self) -> Ch6DiffDataPolarityW<'_, Ch6DataInfSpec> {
        Ch6DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 6 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6DataInfSpec;
impl crate::RegisterSpec for Ch6DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch6DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch6DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6_DATA_INF to value 0"]
impl crate::Resettable for Ch6DataInfSpec {}
