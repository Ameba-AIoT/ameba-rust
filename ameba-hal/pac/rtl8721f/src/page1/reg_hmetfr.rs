#[doc = "Register `REG_HMETFR` reader"]
pub type R = crate::R<RegHmetfrSpec>;
#[doc = "Register `REG_HMETFR` writer"]
pub type W = crate::W<RegHmetfrSpec>;
#[doc = "Field `BIT_H0MSG_VLD` reader - MSG_BOX_0 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH0msgVldR = crate::BitReader;
#[doc = "Field `BIT_H0MSG_VLD` writer - MSG_BOX_0 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH0msgVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_H1MSG_VLD` reader - MSG_BOX_1 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH1msgVldR = crate::BitReader;
#[doc = "Field `BIT_H1MSG_VLD` writer - MSG_BOX_1 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH1msgVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_H2MSG_VLD` reader - MSG_BOX_2 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH2msgVldR = crate::BitReader;
#[doc = "Field `BIT_H2MSG_VLD` writer - MSG_BOX_2 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH2msgVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_H3MSG_VLD` reader - MSG_BOX_3 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH3msgVldR = crate::BitReader;
#[doc = "Field `BIT_H3MSG_VLD` writer - MSG_BOX_3 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
pub type BitH3msgVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `BIT_HRCV_MSG` reader - Host Recover Message, Host Write This Byte will generate interrupt to 8051 (0x0134h\\[5\\])"]
pub type BitHrcvMsgR = crate::FieldReader;
#[doc = "Field `BIT_HRCV_MSG` writer - Host Recover Message, Host Write This Byte will generate interrupt to 8051 (0x0134h\\[5\\])"]
pub type BitHrcvMsgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - MSG_BOX_0 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h0msg_vld(&self) -> BitH0msgVldR {
        BitH0msgVldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MSG_BOX_1 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h1msg_vld(&self) -> BitH1msgVldR {
        BitH1msgVldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSG_BOX_2 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h2msg_vld(&self) -> BitH2msgVldR {
        BitH2msgVldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MSG_BOX_3 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h3msg_vld(&self) -> BitH3msgVldR {
        BitH3msgVldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x000f_ffff)
    }
    #[doc = "Bits 24:31 - Host Recover Message, Host Write This Byte will generate interrupt to 8051 (0x0134h\\[5\\])"]
    #[inline(always)]
    pub fn bit_hrcv_msg(&self) -> BitHrcvMsgR {
        BitHrcvMsgR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MSG_BOX_0 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h0msg_vld(&mut self) -> BitH0msgVldW<'_, RegHmetfrSpec> {
        BitH0msgVldW::new(self, 0)
    }
    #[doc = "Bit 1 - MSG_BOX_1 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h1msg_vld(&mut self) -> BitH1msgVldW<'_, RegHmetfrSpec> {
        BitH1msgVldW::new(self, 1)
    }
    #[doc = "Bit 2 - MSG_BOX_2 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h2msg_vld(&mut self) -> BitH2msgVldW<'_, RegHmetfrSpec> {
        BitH2msgVldW::new(self, 2)
    }
    #[doc = "Bit 3 - MSG_BOX_3 Valid. Enable when the Entry is write, Clear when the Entry is Read by 8051."]
    #[inline(always)]
    pub fn bit_h3msg_vld(&mut self) -> BitH3msgVldW<'_, RegHmetfrSpec> {
        BitH3msgVldW::new(self, 3)
    }
    #[doc = "Bits 4:23 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegHmetfrSpec> {
        RsvdW::new(self, 4)
    }
    #[doc = "Bits 24:31 - Host Recover Message, Host Write This Byte will generate interrupt to 8051 (0x0134h\\[5\\])"]
    #[inline(always)]
    pub fn bit_hrcv_msg(&mut self) -> BitHrcvMsgW<'_, RegHmetfrSpec> {
        BitHrcvMsgW::new(self, 24)
    }
}
#[doc = "Host Message Mail Box Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_hmetfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_hmetfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegHmetfrSpec;
impl crate::RegisterSpec for RegHmetfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_hmetfr::R`](R) reader structure"]
impl crate::Readable for RegHmetfrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_hmetfr::W`](W) writer structure"]
impl crate::Writable for RegHmetfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_HMETFR to value 0"]
impl crate::Resettable for RegHmetfrSpec {}
