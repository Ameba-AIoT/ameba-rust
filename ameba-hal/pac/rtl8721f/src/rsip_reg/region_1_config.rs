#[doc = "Register `Region_1_config` reader"]
pub type R = crate::R<Region1ConfigSpec>;
#[doc = "Register `Region_1_config` writer"]
pub type W = crate::W<Region1ConfigSpec>;
#[doc = "Field `region_1_enable` reader - Enable region x setting"]
pub type Region1EnableR = crate::BitReader;
#[doc = "Field `region_1_enable` writer - Enable region x setting"]
pub type Region1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_1_mode` reader - Mode used in region 1 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region1ModeR = crate::FieldReader;
#[doc = "Field `region_1_mode` writer - Mode used in region 1 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_1_iv_num` reader - IV used in region 1 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region1IvNumR = crate::FieldReader;
#[doc = "Field `region_1_iv_num` writer - IV used in region 1 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region1IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_1_key_num` reader - Key used in region 1 Apply which key reigster to this region Number corresponds to key register"]
pub type Region1KeyNumR = crate::FieldReader;
#[doc = "Field `region_1_key_num` writer - Key used in region 1 Apply which key reigster to this region Number corresponds to key register"]
pub type Region1KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_1_enable(&self) -> Region1EnableR {
        Region1EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 1 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_1_mode(&self) -> Region1ModeR {
        Region1ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 1 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_1_iv_num(&self) -> Region1IvNumR {
        Region1IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 1 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_1_key_num(&self) -> Region1KeyNumR {
        Region1KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_1_enable(&mut self) -> Region1EnableW<'_, Region1ConfigSpec> {
        Region1EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 1 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_1_mode(&mut self) -> Region1ModeW<'_, Region1ConfigSpec> {
        Region1ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 1 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_1_iv_num(&mut self) -> Region1IvNumW<'_, Region1ConfigSpec> {
        Region1IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 1 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_1_key_num(&mut self) -> Region1KeyNumW<'_, Region1ConfigSpec> {
        Region1KeyNumW::new(self, 6)
    }
}
#[doc = "Region 1 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_1_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_1_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region1ConfigSpec;
impl crate::RegisterSpec for Region1ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_1_config::R`](R) reader structure"]
impl crate::Readable for Region1ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_1_config::W`](W) writer structure"]
impl crate::Writable for Region1ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_1_config to value 0"]
impl crate::Resettable for Region1ConfigSpec {}
