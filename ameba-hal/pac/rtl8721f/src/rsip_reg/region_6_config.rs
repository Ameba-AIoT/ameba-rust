#[doc = "Register `Region_6_config` reader"]
pub type R = crate::R<Region6ConfigSpec>;
#[doc = "Register `Region_6_config` writer"]
pub type W = crate::W<Region6ConfigSpec>;
#[doc = "Field `region_6_enable` reader - Enable region x setting"]
pub type Region6EnableR = crate::BitReader;
#[doc = "Field `region_6_enable` writer - Enable region x setting"]
pub type Region6EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_6_mode` reader - Mode used in region 6 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region6ModeR = crate::FieldReader;
#[doc = "Field `region_6_mode` writer - Mode used in region 6 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region6ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_6_iv_num` reader - IV used in region 6 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region6IvNumR = crate::FieldReader;
#[doc = "Field `region_6_iv_num` writer - IV used in region 6 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region6IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_6_key_num` reader - Key used in region 6 Apply which key reigster to this region Number corresponds to key register"]
pub type Region6KeyNumR = crate::FieldReader;
#[doc = "Field `region_6_key_num` writer - Key used in region 6 Apply which key reigster to this region Number corresponds to key register"]
pub type Region6KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_6_enable(&self) -> Region6EnableR {
        Region6EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 6 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_6_mode(&self) -> Region6ModeR {
        Region6ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 6 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_6_iv_num(&self) -> Region6IvNumR {
        Region6IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 6 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_6_key_num(&self) -> Region6KeyNumR {
        Region6KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_6_enable(&mut self) -> Region6EnableW<'_, Region6ConfigSpec> {
        Region6EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 6 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_6_mode(&mut self) -> Region6ModeW<'_, Region6ConfigSpec> {
        Region6ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 6 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_6_iv_num(&mut self) -> Region6IvNumW<'_, Region6ConfigSpec> {
        Region6IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 6 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_6_key_num(&mut self) -> Region6KeyNumW<'_, Region6ConfigSpec> {
        Region6KeyNumW::new(self, 6)
    }
}
#[doc = "Region 6 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_6_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_6_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region6ConfigSpec;
impl crate::RegisterSpec for Region6ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_6_config::R`](R) reader structure"]
impl crate::Readable for Region6ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_6_config::W`](W) writer structure"]
impl crate::Writable for Region6ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_6_config to value 0"]
impl crate::Resettable for Region6ConfigSpec {}
