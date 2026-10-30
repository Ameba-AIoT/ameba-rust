#[doc = "Register `CH8_DATA_INF` reader"]
pub type R = crate::R<Ch8DataInfSpec>;
#[doc = "Register `CH8_DATA_INF` writer"]
pub type W = crate::W<Ch8DataInfSpec>;
#[doc = "Field `ch8_data_ave` reader - Average of channel 8 raw code"]
pub type Ch8DataAveR = crate::FieldReader<u16>;
#[doc = "Field `ch8_data_ave` writer - Average of channel 8 raw code"]
pub type Ch8DataAveW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch8_diff_data` reader - Difference digital data of channel8 between ch8_data_ave and baseline"]
pub type Ch8DiffDataR = crate::FieldReader<u16>;
#[doc = "Field `ch8_diff_data` writer - Difference digital data of channel8 between ch8_data_ave and baseline"]
pub type Ch8DiffDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ch8_diff_data_polarity` reader - Polarity of ch8_diff_data 0: ch8_data_ave<baseline 1: ch8_data_ave>=baseline"]
pub type Ch8DiffDataPolarityR = crate::BitReader;
#[doc = "Field `ch8_diff_data_polarity` writer - Polarity of ch8_diff_data 0: ch8_data_ave<baseline 1: ch8_data_ave>=baseline"]
pub type Ch8DiffDataPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Average of channel 8 raw code"]
    #[inline(always)]
    pub fn ch8_data_ave(&self) -> Ch8DataAveR {
        Ch8DataAveR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel8 between ch8_data_ave and baseline"]
    #[inline(always)]
    pub fn ch8_diff_data(&self) -> Ch8DiffDataR {
        Ch8DiffDataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - Polarity of ch8_diff_data 0: ch8_data_ave<baseline 1: ch8_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch8_diff_data_polarity(&self) -> Ch8DiffDataPolarityR {
        Ch8DiffDataPolarityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Average of channel 8 raw code"]
    #[inline(always)]
    pub fn ch8_data_ave(&mut self) -> Ch8DataAveW<'_, Ch8DataInfSpec> {
        Ch8DataAveW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Difference digital data of channel8 between ch8_data_ave and baseline"]
    #[inline(always)]
    pub fn ch8_diff_data(&mut self) -> Ch8DiffDataW<'_, Ch8DataInfSpec> {
        Ch8DiffDataW::new(self, 16)
    }
    #[doc = "Bit 28 - Polarity of ch8_diff_data 0: ch8_data_ave<baseline 1: ch8_data_ave>=baseline"]
    #[inline(always)]
    pub fn ch8_diff_data_polarity(&mut self) -> Ch8DiffDataPolarityW<'_, Ch8DataInfSpec> {
        Ch8DiffDataPolarityW::new(self, 28)
    }
}
#[doc = "Channel 8 Dtouch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch8_data_inf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch8_data_inf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch8DataInfSpec;
impl crate::RegisterSpec for Ch8DataInfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch8_data_inf::R`](R) reader structure"]
impl crate::Readable for Ch8DataInfSpec {}
#[doc = "`write(|w| ..)` method takes [`ch8_data_inf::W`](W) writer structure"]
impl crate::Writable for Ch8DataInfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH8_DATA_INF to value 0"]
impl crate::Resettable for Ch8DataInfSpec {}
