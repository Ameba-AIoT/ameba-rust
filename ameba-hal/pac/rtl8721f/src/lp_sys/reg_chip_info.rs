#[doc = "Register `REG_CHIP_INFO` reader"]
pub type R = crate::R<RegChipInfoSpec>;
#[doc = "Register `REG_CHIP_INFO` writer"]
pub type W = crate::W<RegChipInfoSpec>;
#[doc = "Field `rl_no` reader - CHIP RL NUMBER : 0x6845"]
pub type RlNoR = crate::FieldReader<u16>;
#[doc = "Field `rl_ver` reader - CHIP cut verison : 0: A-cut 1: B-cut ..."]
pub type RlVerR = crate::FieldReader;
#[doc = "Field `chip_info_en` reader - Enable Internal Chip Information display.(Don't move 0x74 address , for code is not auto-gen) 0xA: enable. Others: disable. Note: CHIP_INFO can be read only if CHIP_INFO_EN\\[3:0\\]=0xA, otherwise the read result will return 0x0."]
pub type ChipInfoEnR = crate::FieldReader;
#[doc = "Field `chip_info_en` writer - Enable Internal Chip Information display.(Don't move 0x74 address , for code is not auto-gen) 0xA: enable. Others: disable. Note: CHIP_INFO can be read only if CHIP_INFO_EN\\[3:0\\]=0xA, otherwise the read result will return 0x0."]
pub type ChipInfoEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - CHIP RL NUMBER : 0x6845"]
    #[inline(always)]
    pub fn rl_no(&self) -> RlNoR {
        RlNoR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - CHIP cut verison : 0: A-cut 1: B-cut ..."]
    #[inline(always)]
    pub fn rl_ver(&self) -> RlVerR {
        RlVerR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Enable Internal Chip Information display.(Don't move 0x74 address , for code is not auto-gen) 0xA: enable. Others: disable. Note: CHIP_INFO can be read only if CHIP_INFO_EN\\[3:0\\]=0xA, otherwise the read result will return 0x0."]
    #[inline(always)]
    pub fn chip_info_en(&self) -> ChipInfoEnR {
        ChipInfoEnR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - Enable Internal Chip Information display.(Don't move 0x74 address , for code is not auto-gen) 0xA: enable. Others: disable. Note: CHIP_INFO can be read only if CHIP_INFO_EN\\[3:0\\]=0xA, otherwise the read result will return 0x0."]
    #[inline(always)]
    pub fn chip_info_en(&mut self) -> ChipInfoEnW<'_, RegChipInfoSpec> {
        ChipInfoEnW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_chip_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_chip_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChipInfoSpec;
impl crate::RegisterSpec for RegChipInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_chip_info::R`](R) reader structure"]
impl crate::Readable for RegChipInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_chip_info::W`](W) writer structure"]
impl crate::Writable for RegChipInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHIP_INFO to value 0x6845"]
impl crate::Resettable for RegChipInfoSpec {
    const RESET_VALUE: u32 = 0x6845;
}
