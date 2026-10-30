#[doc = "Register `Region_4_config` reader"]
pub type R = crate::R<Region4ConfigSpec>;
#[doc = "Register `Region_4_config` writer"]
pub type W = crate::W<Region4ConfigSpec>;
#[doc = "Field `region_4_enable` reader - Enable region x setting"]
pub type Region4EnableR = crate::BitReader;
#[doc = "Field `region_4_enable` writer - Enable region x setting"]
pub type Region4EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_4_mode` reader - Mode used in region 4 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region4ModeR = crate::FieldReader;
#[doc = "Field `region_4_mode` writer - Mode used in region 4 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region4ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_4_iv_num` reader - IV used in region 4 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region4IvNumR = crate::FieldReader;
#[doc = "Field `region_4_iv_num` writer - IV used in region 4 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region4IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_4_key_num` reader - Key used in region 4 Apply which key reigster to this region Number corresponds to key register"]
pub type Region4KeyNumR = crate::FieldReader;
#[doc = "Field `region_4_key_num` writer - Key used in region 4 Apply which key reigster to this region Number corresponds to key register"]
pub type Region4KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_4_enable(&self) -> Region4EnableR {
        Region4EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 4 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_4_mode(&self) -> Region4ModeR {
        Region4ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 4 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_4_iv_num(&self) -> Region4IvNumR {
        Region4IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 4 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_4_key_num(&self) -> Region4KeyNumR {
        Region4KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_4_enable(&mut self) -> Region4EnableW<'_, Region4ConfigSpec> {
        Region4EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 4 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_4_mode(&mut self) -> Region4ModeW<'_, Region4ConfigSpec> {
        Region4ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 4 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_4_iv_num(&mut self) -> Region4IvNumW<'_, Region4ConfigSpec> {
        Region4IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 4 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_4_key_num(&mut self) -> Region4KeyNumW<'_, Region4ConfigSpec> {
        Region4KeyNumW::new(self, 6)
    }
}
#[doc = "Region 4 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_4_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_4_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region4ConfigSpec;
impl crate::RegisterSpec for Region4ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_4_config::R`](R) reader structure"]
impl crate::Readable for Region4ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_4_config::W`](W) writer structure"]
impl crate::Writable for Region4ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_4_config to value 0"]
impl crate::Resettable for Region4ConfigSpec {}
