#[doc = "Register `Region_7_config` reader"]
pub type R = crate::R<Region7ConfigSpec>;
#[doc = "Register `Region_7_config` writer"]
pub type W = crate::W<Region7ConfigSpec>;
#[doc = "Field `region_7_enable` reader - Enable region x setting"]
pub type Region7EnableR = crate::BitReader;
#[doc = "Field `region_7_enable` writer - Enable region x setting"]
pub type Region7EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `region_7_mode` reader - Mode used in region 7 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region7ModeR = crate::FieldReader;
#[doc = "Field `region_7_mode` writer - Mode used in region 7 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
pub type Region7ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `region_7_iv_num` reader - IV used in region 7 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region7IvNumR = crate::FieldReader;
#[doc = "Field `region_7_iv_num` writer - IV used in region 7 Apply which IV reigster to this region Number corresponds to IV register"]
pub type Region7IvNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `region_7_key_num` reader - Key used in region 7 Apply which key reigster to this region Number corresponds to key register"]
pub type Region7KeyNumR = crate::FieldReader;
#[doc = "Field `region_7_key_num` writer - Key used in region 7 Apply which key reigster to this region Number corresponds to key register"]
pub type Region7KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_7_enable(&self) -> Region7EnableR {
        Region7EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 7 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_7_mode(&self) -> Region7ModeR {
        Region7ModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - IV used in region 7 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_7_iv_num(&self) -> Region7IvNumR {
        Region7IvNumR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Key used in region 7 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_7_key_num(&self) -> Region7KeyNumR {
        Region7KeyNumR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable region x setting"]
    #[inline(always)]
    pub fn region_7_enable(&mut self) -> Region7EnableW<'_, Region7ConfigSpec> {
        Region7EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Mode used in region 7 2'b00: GCM mode 2'b01: CTR mode 2'b10: MIX mode 2'b11: Reserved"]
    #[inline(always)]
    pub fn region_7_mode(&mut self) -> Region7ModeW<'_, Region7ConfigSpec> {
        Region7ModeW::new(self, 1)
    }
    #[doc = "Bits 3:5 - IV used in region 7 Apply which IV reigster to this region Number corresponds to IV register"]
    #[inline(always)]
    pub fn region_7_iv_num(&mut self) -> Region7IvNumW<'_, Region7ConfigSpec> {
        Region7IvNumW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Key used in region 7 Apply which key reigster to this region Number corresponds to key register"]
    #[inline(always)]
    pub fn region_7_key_num(&mut self) -> Region7KeyNumW<'_, Region7ConfigSpec> {
        Region7KeyNumW::new(self, 6)
    }
}
#[doc = "Region 7 setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_7_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_7_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Region7ConfigSpec;
impl crate::RegisterSpec for Region7ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_7_config::R`](R) reader structure"]
impl crate::Readable for Region7ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`region_7_config::W`](W) writer structure"]
impl crate::Writable for Region7ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Region_7_config to value 0"]
impl crate::Resettable for Region7ConfigSpec {}
