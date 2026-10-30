#[doc = "Register `Region_0_config` reader"]
pub type R = crate::R<Region0ConfigSpec>;
#[doc = "Register `Region_0_config` writer"]
pub type W = crate::W<Region0ConfigSpec>;
#[doc = "Field `region_0_enable` reader - Enable region x setting"]
pub type Region0EnableR = crate::BitReader;
#[doc = "Field `region_0_enable` writer - Enable region x setting"]
pub type Region0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_0_mode` reader - Mode used in region 0 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region0ModeR = crate::FieldReader;
#[doc = "Field `region_0_mode` writer - Mode used in region 0 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_0_iv_num` reader - IV used in region 0 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region0IvNumR = crate::FieldReader;
#[doc = "Field `region_0_iv_num` writer - IV used in region 0 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region0IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_0_key_num` reader - Key used in region 0 Apply which key reigster to this region Number corresponds to key register"]
pub type Region0KeyNumR = crate::FieldReader;
#[doc = "Field `region_0_key_num` writer - Key used in region 0 Apply which key reigster to this region Number corresponds to key register"]
pub type Region0KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_0_enable(&self) -> Region0EnableR {
        Region0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 0 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_0_mode(&self) -> Region0ModeR {
        Region0ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 0 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_0_iv_num(&self) -> Region0IvNumR {
        Region0IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 0 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_0_key_num(&self) -> Region0KeyNumR {
        Region0KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_0_enable(&mut self) -> Region0EnableW<'_, Region0ConfigSpec> {
        Region0EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 0 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_0_mode(&mut self) -> Region0ModeW<'_, Region0ConfigSpec> {
        Region0ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 0 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_0_iv_num(&mut self) -> Region0IvNumW<'_, Region0ConfigSpec> {
        Region0IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 0 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_0_key_num(&mut self) -> Region0KeyNumW<'_, Region0ConfigSpec> {
        Region0KeyNumW::new(self, 6)
    }
}
#[doc = "Region 0 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_0_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_0_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region0ConfigSpec;
impl crate::RegisterSpec for Region0ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_0_config::R`](R) reader structure"]
impl crate::Readable for Region0ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_0_config::W`](W) writer structure"]
impl crate::Writable for Region0ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_0_config to value 0"]
impl crate::Resettable for Region0ConfigSpec {}
