#[doc = "Register `Region_2_config` reader"]
pub type R = crate::R<Region2ConfigSpec>;
#[doc = "Register `Region_2_config` writer"]
pub type W = crate::W<Region2ConfigSpec>;
#[doc = "Field `region_2_enable` reader - Enable region x setting"]
pub type Region2EnableR = crate::BitReader;
#[doc = "Field `region_2_enable` writer - Enable region x setting"]
pub type Region2EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_2_mode` reader - Mode used in region 2 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region2ModeR = crate::FieldReader;
#[doc = "Field `region_2_mode` writer - Mode used in region 2 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_2_iv_num` reader - IV used in region 2 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region2IvNumR = crate::FieldReader;
#[doc = "Field `region_2_iv_num` writer - IV used in region 2 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region2IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_2_key_num` reader - Key used in region 2 Apply which key reigster to this region Number corresponds to key register"]
pub type Region2KeyNumR = crate::FieldReader;
#[doc = "Field `region_2_key_num` writer - Key used in region 2 Apply which key reigster to this region Number corresponds to key register"]
pub type Region2KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_2_enable(&self) -> Region2EnableR {
        Region2EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 2 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_2_mode(&self) -> Region2ModeR {
        Region2ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 2 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_2_iv_num(&self) -> Region2IvNumR {
        Region2IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 2 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_2_key_num(&self) -> Region2KeyNumR {
        Region2KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_2_enable(&mut self) -> Region2EnableW<'_, Region2ConfigSpec> {
        Region2EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 2 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_2_mode(&mut self) -> Region2ModeW<'_, Region2ConfigSpec> {
        Region2ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 2 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_2_iv_num(&mut self) -> Region2IvNumW<'_, Region2ConfigSpec> {
        Region2IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 2 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_2_key_num(&mut self) -> Region2KeyNumW<'_, Region2ConfigSpec> {
        Region2KeyNumW::new(self, 6)
    }
}
#[doc = "Region 2 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_2_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_2_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region2ConfigSpec;
impl crate::RegisterSpec for Region2ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_2_config::R`](R) reader structure"]
impl crate::Readable for Region2ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_2_config::W`](W) writer structure"]
impl crate::Writable for Region2ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_2_config to value 0"]
impl crate::Resettable for Region2ConfigSpec {}
