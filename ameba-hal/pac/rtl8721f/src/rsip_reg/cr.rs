#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `tag_size` reader - Tag size. The size of the tag will affect the length of each tag command. 0: 32 bits 1: 64 bits 2: 128bits"]
pub type TagSizeR = crate::FieldReader;
#[doc = "Field `tag_size` writer - Tag size. The size of the tag will affect the length of each tag command. 0: 32 bits 1: 64 bits 2: 128bits"]
pub type TagSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `axi_byte_swap` reader - Determine the endian of ciphertext in slave memory. If set to 1, can swap byte order."]
pub type AxiByteSwapR = crate::BitReader;
#[doc = "Field `axi_byte_swap` writer - Determine the endian of ciphertext in slave memory. If set to 1, can swap byte order."]
pub type AxiByteSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `axi_word_swap` reader - Determine the endian of ciphertext in slave memory. If set to 1, can swap word order."]
pub type AxiWordSwapR = crate::BitReader;
#[doc = "Field `axi_word_swap` writer - Determine the endian of ciphertext in slave memory. If set to 1, can swap word order."]
pub type AxiWordSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `key_size` reader - Determine the size of KEY 0: 128 bits 1: 192 bits 2: 256 bits"]
pub type KeySizeR = crate::FieldReader;
#[doc = "Field `key_size` writer - Determine the size of KEY 0: 128 bits 1: 192 bits 2: 256 bits"]
pub type KeySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `aes_en` reader - Determine whether to enable the AES engine 0: disable decryption engine (expect remapping function) 1: enable decryption engine and line buffer"]
pub type AesEnR = crate::BitReader;
#[doc = "Field `aes_en` writer - Determine whether to enable the AES engine 0: disable decryption engine (expect remapping function) 1: enable decryption engine and line buffer"]
pub type AesEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `icg_ctrl_0` reader - Disable line-buffer auto clock gating"]
pub type IcgCtrl0R = crate::BitReader;
#[doc = "Field `icg_ctrl_0` writer - Disable line-buffer auto clock gating"]
pub type IcgCtrl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `icg_ctrl_1` reader - Disable fetch-unit auto clock gating"]
pub type IcgCtrl1R = crate::BitReader;
#[doc = "Field `icg_ctrl_1` writer - Disable fetch-unit auto clock gating"]
pub type IcgCtrl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `icg_ctrl_2` reader - Disable aes-engine auto clock gating"]
pub type IcgCtrl2R = crate::BitReader;
#[doc = "Field `icg_ctrl_2` writer - Disable aes-engine auto clock gating"]
pub type IcgCtrl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `icg_ctrl_3` reader - Disable apb-ctrl auto clock gating"]
pub type IcgCtrl3R = crate::BitReader;
#[doc = "Field `icg_ctrl_3` writer - Disable apb-ctrl auto clock gating"]
pub type IcgCtrl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `icg_ctrl_4` reader - Disable key-reg auto clock gating"]
pub type IcgCtrl4R = crate::BitReader;
#[doc = "Field `icg_ctrl_4` writer - Disable key-reg auto clock gating"]
pub type IcgCtrl4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Tag size. The size of the tag will affect the length of each tag command. 0: 32 bits 1: 64 bits 2: 128bits"]
    #[inline(always)]
    pub fn tag_size(&self) -> TagSizeR {
        TagSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Determine the endian of ciphertext in slave memory. If set to 1, can swap byte order."]
    #[inline(always)]
    pub fn axi_byte_swap(&self) -> AxiByteSwapR {
        AxiByteSwapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Determine the endian of ciphertext in slave memory. If set to 1, can swap word order."]
    #[inline(always)]
    pub fn axi_word_swap(&self) -> AxiWordSwapR {
        AxiWordSwapR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Determine the size of KEY 0: 128 bits 1: 192 bits 2: 256 bits"]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Determine whether to enable the AES engine 0: disable decryption engine (expect remapping function) 1: enable decryption engine and line buffer"]
    #[inline(always)]
    pub fn aes_en(&self) -> AesEnR {
        AesEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable line-buffer auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_0(&self) -> IcgCtrl0R {
        IcgCtrl0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable fetch-unit auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_1(&self) -> IcgCtrl1R {
        IcgCtrl1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable aes-engine auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_2(&self) -> IcgCtrl2R {
        IcgCtrl2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable apb-ctrl auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_3(&self) -> IcgCtrl3R {
        IcgCtrl3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable key-reg auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_4(&self) -> IcgCtrl4R {
        IcgCtrl4R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tag size. The size of the tag will affect the length of each tag command. 0: 32 bits 1: 64 bits 2: 128bits"]
    #[inline(always)]
    pub fn tag_size(&mut self) -> TagSizeW<'_, CrSpec> {
        TagSizeW::new(self, 0)
    }
    #[doc = "Bit 3 - Determine the endian of ciphertext in slave memory. If set to 1, can swap byte order."]
    #[inline(always)]
    pub fn axi_byte_swap(&mut self) -> AxiByteSwapW<'_, CrSpec> {
        AxiByteSwapW::new(self, 3)
    }
    #[doc = "Bit 4 - Determine the endian of ciphertext in slave memory. If set to 1, can swap word order."]
    #[inline(always)]
    pub fn axi_word_swap(&mut self) -> AxiWordSwapW<'_, CrSpec> {
        AxiWordSwapW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Determine the size of KEY 0: 128 bits 1: 192 bits 2: 256 bits"]
    #[inline(always)]
    pub fn key_size(&mut self) -> KeySizeW<'_, CrSpec> {
        KeySizeW::new(self, 5)
    }
    #[doc = "Bit 8 - Determine whether to enable the AES engine 0: disable decryption engine (expect remapping function) 1: enable decryption engine and line buffer"]
    #[inline(always)]
    pub fn aes_en(&mut self) -> AesEnW<'_, CrSpec> {
        AesEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Disable line-buffer auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_0(&mut self) -> IcgCtrl0W<'_, CrSpec> {
        IcgCtrl0W::new(self, 16)
    }
    #[doc = "Bit 17 - Disable fetch-unit auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_1(&mut self) -> IcgCtrl1W<'_, CrSpec> {
        IcgCtrl1W::new(self, 17)
    }
    #[doc = "Bit 18 - Disable aes-engine auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_2(&mut self) -> IcgCtrl2W<'_, CrSpec> {
        IcgCtrl2W::new(self, 18)
    }
    #[doc = "Bit 19 - Disable apb-ctrl auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_3(&mut self) -> IcgCtrl3W<'_, CrSpec> {
        IcgCtrl3W::new(self, 19)
    }
    #[doc = "Bit 20 - Disable key-reg auto clock gating"]
    #[inline(always)]
    pub fn icg_ctrl_4(&mut self) -> IcgCtrl4W<'_, CrSpec> {
        IcgCtrl4W::new(self, 20)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x40"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x40;
}
