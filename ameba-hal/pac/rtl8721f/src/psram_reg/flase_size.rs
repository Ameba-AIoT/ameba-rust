#[doc = "Register `FLASE_SIZE` reader"]
pub type R = crate::R<FlaseSizeSpec>;
#[doc = "Register `FLASE_SIZE` writer"]
pub type W = crate::W<FlaseSizeSpec>;
#[doc = "Field `FLASH_SIZE` reader - Indicate the size of flash size to select the target SPI Flash in auto mode. (flash size unit is byte) If TAG >= 2.0.3: CR_FLASH_SIZE = log2 (flash size) -15. Ex: flash size is 4MB, CR_FLASH_SIZE = log2 (4MB) -15 = 22 - 15 = 7"]
pub type FlashSizeR = crate::FieldReader;
#[doc = "Field `FLASH_SIZE` writer - Indicate the size of flash size to select the target SPI Flash in auto mode. (flash size unit is byte) If TAG >= 2.0.3: CR_FLASH_SIZE = log2 (flash size) -15. Ex: flash size is 4MB, CR_FLASH_SIZE = log2 (4MB) -15 = 22 - 15 = 7"]
pub type FlashSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Indicate the size of flash size to select the target SPI Flash in auto mode. (flash size unit is byte) If TAG >= 2.0.3: CR_FLASH_SIZE = log2 (flash size) -15. Ex: flash size is 4MB, CR_FLASH_SIZE = log2 (4MB) -15 = 22 - 15 = 7"]
    #[inline(always)]
    pub fn flash_size(&self) -> FlashSizeR {
        FlashSizeR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Indicate the size of flash size to select the target SPI Flash in auto mode. (flash size unit is byte) If TAG >= 2.0.3: CR_FLASH_SIZE = log2 (flash size) -15. Ex: flash size is 4MB, CR_FLASH_SIZE = log2 (4MB) -15 = 22 - 15 = 7"]
    #[inline(always)]
    pub fn flash_size(&mut self) -> FlashSizeW<'_, FlaseSizeSpec> {
        FlashSizeW::new(self, 0)
    }
}
#[doc = "Flash Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flase_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flase_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlaseSizeSpec;
impl crate::RegisterSpec for FlaseSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flase_size::R`](R) reader structure"]
impl crate::Readable for FlaseSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`flase_size::W`](W) writer structure"]
impl crate::Writable for FlaseSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASE_SIZE to value 0x0c"]
impl crate::Resettable for FlaseSizeSpec {
    const RESET_VALUE: u32 = 0x0c;
}
