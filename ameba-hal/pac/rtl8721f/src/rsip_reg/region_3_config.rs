#[doc = "Register `Region_3_config` reader"]
pub type R = crate::R<Region3ConfigSpec>;
#[doc = "Register `Region_3_config` writer"]
pub type W = crate::W<Region3ConfigSpec>;
#[doc = "Field `region_3_enable` reader - Enable region x setting"]
pub type Region3EnableR = crate::BitReader;
#[doc = "Field `region_3_enable` writer - Enable region x setting"]
pub type Region3EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_3_mode` reader - Mode used in region 3 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region3ModeR = crate::FieldReader;
#[doc = "Field `region_3_mode` writer - Mode used in region 3 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region3ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_3_iv_num` reader - IV used in region 3 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region3IvNumR = crate::FieldReader;
#[doc = "Field `region_3_iv_num` writer - IV used in region 3 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region3IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_3_key_num` reader - Key used in region 3 Apply which key reigster to this region Number corresponds to key register"]
pub type Region3KeyNumR = crate::FieldReader;
#[doc = "Field `region_3_key_num` writer - Key used in region 3 Apply which key reigster to this region Number corresponds to key register"]
pub type Region3KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_3_enable(&self) -> Region3EnableR {
        Region3EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 3 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_3_mode(&self) -> Region3ModeR {
        Region3ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 3 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_3_iv_num(&self) -> Region3IvNumR {
        Region3IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 3 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_3_key_num(&self) -> Region3KeyNumR {
        Region3KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_3_enable(&mut self) -> Region3EnableW<'_, Region3ConfigSpec> {
        Region3EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 3 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_3_mode(&mut self) -> Region3ModeW<'_, Region3ConfigSpec> {
        Region3ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 3 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_3_iv_num(&mut self) -> Region3IvNumW<'_, Region3ConfigSpec> {
        Region3IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 3 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_3_key_num(&mut self) -> Region3KeyNumW<'_, Region3ConfigSpec> {
        Region3KeyNumW::new(self, 6)
    }
}
#[doc = "Region 3 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_3_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_3_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region3ConfigSpec;
impl crate::RegisterSpec for Region3ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_3_config::R`](R) reader structure"]
impl crate::Readable for Region3ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_3_config::W`](W) writer structure"]
impl crate::Writable for Region3ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_3_config to value 0"]
impl crate::Resettable for Region3ConfigSpec {}
