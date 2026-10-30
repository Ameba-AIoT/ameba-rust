#[doc = "Register `Region_5_config` reader"]
pub type R = crate::R<Region5ConfigSpec>;
#[doc = "Register `Region_5_config` writer"]
pub type W = crate::W<Region5ConfigSpec>;
#[doc = "Field `region_5_enable` reader - Enable region x setting"]
pub type Region5EnableR = crate::BitReader;
#[doc = "Field `region_5_enable` writer - Enable region x setting"]
pub type Region5EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_5_mode` reader - Mode used in region 5 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region5ModeR = crate::FieldReader;
#[doc = "Field `region_5_mode` writer - Mode used in region 5 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region5ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_5_iv_num` reader - IV used in region 5 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region5IvNumR = crate::FieldReader;
#[doc = "Field `region_5_iv_num` writer - IV used in region 5 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region5IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_5_key_num` reader - Key used in region 5 Apply which key reigster to this region Number corresponds to key register"]
pub type Region5KeyNumR = crate::FieldReader;
#[doc = "Field `region_5_key_num` writer - Key used in region 5 Apply which key reigster to this region Number corresponds to key register"]
pub type Region5KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_5_enable(&self) -> Region5EnableR {
        Region5EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 5 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_5_mode(&self) -> Region5ModeR {
        Region5ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 5 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_5_iv_num(&self) -> Region5IvNumR {
        Region5IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 5 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_5_key_num(&self) -> Region5KeyNumR {
        Region5KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_5_enable(&mut self) -> Region5EnableW<'_, Region5ConfigSpec> {
        Region5EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 5 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_5_mode(&mut self) -> Region5ModeW<'_, Region5ConfigSpec> {
        Region5ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 5 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_5_iv_num(&mut self) -> Region5IvNumW<'_, Region5ConfigSpec> {
        Region5IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 5 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_5_key_num(&mut self) -> Region5KeyNumW<'_, Region5ConfigSpec> {
        Region5KeyNumW::new(self, 6)
    }
}
#[doc = "Region 5 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_5_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_5_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region5ConfigSpec;
impl crate::RegisterSpec for Region5ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_5_config::R`](R) reader structure"]
impl crate::Readable for Region5ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_5_config::W`](W) writer structure"]
impl crate::Writable for Region5ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_5_config to value 0"]
impl crate::Resettable for Region5ConfigSpec {}
