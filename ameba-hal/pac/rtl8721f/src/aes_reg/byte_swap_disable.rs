#[doc = "Register `byte_swap_disable` reader"]
pub type R = crate::R<ByteSwapDisableSpec>;
#[doc = "Register `byte_swap_disable` writer"]
pub type W = crate::W<ByteSwapDisableSpec>;
#[doc = "Field `payload_swap_disable` reader - 1: disable swap the data from mem"]
pub type PayloadSwapDisableR = crate::BitReader;
#[doc = "Field `payload_swap_disable` writer - 1: disable swap the data from mem"]
pub type PayloadSwapDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `key_swap_disable` reader - 1: disable swap key_x or key_management input key"]
pub type KeySwapDisableR = crate::BitReader;
#[doc = "Field `key_swap_disable` writer - 1: disable swap key_x or key_management input key"]
pub type KeySwapDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_data_swap_disable` reader - 1: disable swap cpu_datain_x and cpu_dataout_x"]
pub type CpuDataSwapDisableR = crate::BitReader;
#[doc = "Field `cpu_data_swap_disable` writer - 1: disable swap cpu_datain_x and cpu_dataout_x"]
pub type CpuDataSwapDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iv_swap_disable` reader - 1: disable swap iv_x"]
pub type IvSwapDisableR = crate::BitReader;
#[doc = "Field `iv_swap_disable` writer - 1: disable swap iv_x"]
pub type IvSwapDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tag_swap_disable` reader - 1: disable swap tag_x"]
pub type TagSwapDisableR = crate::BitReader;
#[doc = "Field `tag_swap_disable` writer - 1: disable swap tag_x"]
pub type TagSwapDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ghash_key_swap_disable` reader - 1: disable swap ghash_key_x"]
pub type GhashKeySwapDisableR = crate::BitReader;
#[doc = "Field `ghash_key_swap_disable` writer - 1: disable swap ghash_key_x"]
pub type GhashKeySwapDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: disable swap the data from mem"]
    #[inline(always)]
    pub fn payload_swap_disable(&self) -> PayloadSwapDisableR {
        PayloadSwapDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: disable swap key_x or key_management input key"]
    #[inline(always)]
    pub fn key_swap_disable(&self) -> KeySwapDisableR {
        KeySwapDisableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: disable swap cpu_datain_x and cpu_dataout_x"]
    #[inline(always)]
    pub fn cpu_data_swap_disable(&self) -> CpuDataSwapDisableR {
        CpuDataSwapDisableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: disable swap iv_x"]
    #[inline(always)]
    pub fn iv_swap_disable(&self) -> IvSwapDisableR {
        IvSwapDisableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: disable swap tag_x"]
    #[inline(always)]
    pub fn tag_swap_disable(&self) -> TagSwapDisableR {
        TagSwapDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: disable swap ghash_key_x"]
    #[inline(always)]
    pub fn ghash_key_swap_disable(&self) -> GhashKeySwapDisableR {
        GhashKeySwapDisableR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: disable swap the data from mem"]
    #[inline(always)]
    pub fn payload_swap_disable(&mut self) -> PayloadSwapDisableW<'_, ByteSwapDisableSpec> {
        PayloadSwapDisableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: disable swap key_x or key_management input key"]
    #[inline(always)]
    pub fn key_swap_disable(&mut self) -> KeySwapDisableW<'_, ByteSwapDisableSpec> {
        KeySwapDisableW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: disable swap cpu_datain_x and cpu_dataout_x"]
    #[inline(always)]
    pub fn cpu_data_swap_disable(&mut self) -> CpuDataSwapDisableW<'_, ByteSwapDisableSpec> {
        CpuDataSwapDisableW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: disable swap iv_x"]
    #[inline(always)]
    pub fn iv_swap_disable(&mut self) -> IvSwapDisableW<'_, ByteSwapDisableSpec> {
        IvSwapDisableW::new(self, 3)
    }
    #[doc = "Bit 4 - 1: disable swap tag_x"]
    #[inline(always)]
    pub fn tag_swap_disable(&mut self) -> TagSwapDisableW<'_, ByteSwapDisableSpec> {
        TagSwapDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - 1: disable swap ghash_key_x"]
    #[inline(always)]
    pub fn ghash_key_swap_disable(&mut self) -> GhashKeySwapDisableW<'_, ByteSwapDisableSpec> {
        GhashKeySwapDisableW::new(self, 5)
    }
}
#[doc = "Byte swap control register\n\nYou can [`read`](crate::Reg::read) this register and get [`byte_swap_disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`byte_swap_disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ByteSwapDisableSpec;
impl crate::RegisterSpec for ByteSwapDisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`byte_swap_disable::R`](R) reader structure"]
impl crate::Readable for ByteSwapDisableSpec {}
#[doc = "`write(|w| ..)` method takes [`byte_swap_disable::W`](W) writer structure"]
impl crate::Writable for ByteSwapDisableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets byte_swap_disable to value 0"]
impl crate::Resettable for ByteSwapDisableSpec {}
