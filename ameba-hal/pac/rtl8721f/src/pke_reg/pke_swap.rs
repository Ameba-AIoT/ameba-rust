#[doc = "Register `PKE_Swap` reader"]
pub type R = crate::R<PkeSwapSpec>;
#[doc = "Register `PKE_Swap` writer"]
pub type W = crate::W<PkeSwapSpec>;
#[doc = "Field `SWAP_BASE` reader - Word swap base address in word (32 bits)"]
pub type SwapBaseR = crate::FieldReader<u16>;
#[doc = "Field `SWAP_BASE` writer - Word swap base address in word (32 bits)"]
pub type SwapBaseW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `KEY_PORT_EN` reader - 1: Enable hardware key port 0: Do not enable hardware key port"]
pub type KeyPortEnR = crate::BitReader;
#[doc = "Field `KEY_PORT_EN` writer - 1: Enable hardware key port 0: Do not enable hardware key port"]
pub type KeyPortEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_NUM` reader - Use which key (binary encoding), key0 ~ key7"]
pub type KeyNumR = crate::FieldReader;
#[doc = "Field `KEY_NUM` writer - Use which key (binary encoding), key0 ~ key7"]
pub type KeyNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 2:10 - Word swap base address in word (32 bits)"]
    #[inline(always)]
    pub fn swap_base(&self) -> SwapBaseR {
        SwapBaseR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - 1: Enable hardware key port 0: Do not enable hardware key port"]
    #[inline(always)]
    pub fn key_port_en(&self) -> KeyPortEnR {
        KeyPortEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Use which key (binary encoding), key0 ~ key7"]
    #[inline(always)]
    pub fn key_num(&self) -> KeyNumR {
        KeyNumR::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 2:10 - Word swap base address in word (32 bits)"]
    #[inline(always)]
    pub fn swap_base(&mut self) -> SwapBaseW<'_, PkeSwapSpec> {
        SwapBaseW::new(self, 2)
    }
    #[doc = "Bit 16 - 1: Enable hardware key port 0: Do not enable hardware key port"]
    #[inline(always)]
    pub fn key_port_en(&mut self) -> KeyPortEnW<'_, PkeSwapSpec> {
        KeyPortEnW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Use which key (binary encoding), key0 ~ key7"]
    #[inline(always)]
    pub fn key_num(&mut self) -> KeyNumW<'_, PkeSwapSpec> {
        KeyNumW::new(self, 17)
    }
}
#[doc = "Base addess of word swap Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pke_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pke_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkeSwapSpec;
impl crate::RegisterSpec for PkeSwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pke_swap::R`](R) reader structure"]
impl crate::Readable for PkeSwapSpec {}
#[doc = "`write(|w| ..)` method takes [`pke_swap::W`](W) writer structure"]
impl crate::Writable for PkeSwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKE_Swap to value 0"]
impl crate::Resettable for PkeSwapSpec {}
