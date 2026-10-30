#[doc = "Register `HMAC_Block_control` reader"]
pub type R = crate::R<HmacBlockControlSpec>;
#[doc = "Register `HMAC_Block_control` writer"]
pub type W = crate::W<HmacBlockControlSpec>;
#[doc = "Field `first_block` reader - Indicates this operation is the first block. If set to SHA2 mode, the digest will be initialized automatically. If set to HMAC mode, the i_pad step will be executed first."]
pub type FirstBlockR = crate::BitReader;
#[doc = "Field `first_block` writer - Indicates this operation is the first block. If set to SHA2 mode, the digest will be initialized automatically. If set to HMAC mode, the i_pad step will be executed first."]
pub type FirstBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `last_block` reader - Indicates this operation is the last block. If set to SHA2 mode, the padding step will be executed at the end. If set to HMAC mode, the o_pad step will be executed at the end."]
pub type LastBlockR = crate::BitReader;
#[doc = "Field `last_block` writer - Indicates this operation is the last block. If set to SHA2 mode, the padding step will be executed at the end. If set to HMAC mode, the o_pad step will be executed at the end."]
pub type LastBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `End_slave_input` reader - Avaliable when using slave mode If all input data has been completed, set this bit to 1 to start padding."]
pub type EndSlaveInputR = crate::BitReader;
#[doc = "Field `End_slave_input` writer - Avaliable when using slave mode If all input data has been completed, set this bit to 1 to start padding."]
pub type EndSlaveInputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates this operation is the first block. If set to SHA2 mode, the digest will be initialized automatically. If set to HMAC mode, the i_pad step will be executed first."]
    #[inline(always)]
    pub fn first_block(&self) -> FirstBlockR {
        FirstBlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates this operation is the last block. If set to SHA2 mode, the padding step will be executed at the end. If set to HMAC mode, the o_pad step will be executed at the end."]
    #[inline(always)]
    pub fn last_block(&self) -> LastBlockR {
        LastBlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Avaliable when using slave mode If all input data has been completed, set this bit to 1 to start padding."]
    #[inline(always)]
    pub fn end_slave_input(&self) -> EndSlaveInputR {
        EndSlaveInputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates this operation is the first block. If set to SHA2 mode, the digest will be initialized automatically. If set to HMAC mode, the i_pad step will be executed first."]
    #[inline(always)]
    pub fn first_block(&mut self) -> FirstBlockW<'_, HmacBlockControlSpec> {
        FirstBlockW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates this operation is the last block. If set to SHA2 mode, the padding step will be executed at the end. If set to HMAC mode, the o_pad step will be executed at the end."]
    #[inline(always)]
    pub fn last_block(&mut self) -> LastBlockW<'_, HmacBlockControlSpec> {
        LastBlockW::new(self, 1)
    }
    #[doc = "Bit 2 - Avaliable when using slave mode If all input data has been completed, set this bit to 1 to start padding."]
    #[inline(always)]
    pub fn end_slave_input(&mut self) -> EndSlaveInputW<'_, HmacBlockControlSpec> {
        EndSlaveInputW::new(self, 2)
    }
}
#[doc = "HMAC engine block control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hmac_block_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hmac_block_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HmacBlockControlSpec;
impl crate::RegisterSpec for HmacBlockControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hmac_block_control::R`](R) reader structure"]
impl crate::Readable for HmacBlockControlSpec {}
#[doc = "`write(|w| ..)` method takes [`hmac_block_control::W`](W) writer structure"]
impl crate::Writable for HmacBlockControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HMAC_Block_control to value 0"]
impl crate::Resettable for HmacBlockControlSpec {}
