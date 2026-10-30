#[doc = "Register `REG_KEYCAM0_ENTRY17` reader"]
pub type R = crate::R<RegKeycam0Entry17Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY17` writer"]
pub type W = crate::W<RegKeycam0Entry17Spec>;
#[doc = "Field `KEYCAM0_ENTRY17_KEYID_BIT` reader - "]
pub type Keycam0Entry17KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY17_KEYID_BIT` writer - "]
pub type Keycam0Entry17KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY17_TYPE` reader - "]
pub type Keycam0Entry17TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY17_TYPE` writer - "]
pub type Keycam0Entry17TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY17_MICKEY_BIT` reader - "]
pub type Keycam0Entry17MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY17_MICKEY_BIT` writer - "]
pub type Keycam0Entry17MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY17_GRPKEY_BIT` reader - "]
pub type Keycam0Entry17GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY17_GRPKEY_BIT` writer - "]
pub type Keycam0Entry17GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY17_MGNT` reader - "]
pub type Keycam0Entry17MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY17_MGNT` writer - "]
pub type Keycam0Entry17MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY17_REPEATER_MODE` reader - "]
pub type Keycam0Entry17RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY17_REPEATER_MODE` writer - "]
pub type Keycam0Entry17RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY17_EXTTYPE` reader - "]
pub type Keycam0Entry17ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY17_EXTTYPE` writer - "]
pub type Keycam0Entry17ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY17_VALID` reader - "]
pub type Keycam0Entry17ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY17_VALID` writer - "]
pub type Keycam0Entry17ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY17_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry17Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY17_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry17Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry17_keyid_bit(&self) -> Keycam0Entry17KeyidBitR {
        Keycam0Entry17KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry17_type(&self) -> Keycam0Entry17TypeR {
        Keycam0Entry17TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry17_mickey_bit(&self) -> Keycam0Entry17MickeyBitR {
        Keycam0Entry17MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry17_grpkey_bit(&self) -> Keycam0Entry17GrpkeyBitR {
        Keycam0Entry17GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry17_mgnt(&self) -> Keycam0Entry17MgntR {
        Keycam0Entry17MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry17_repeater_mode(&self) -> Keycam0Entry17RepeaterModeR {
        Keycam0Entry17RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry17_exttype(&self) -> Keycam0Entry17ExttypeR {
        Keycam0Entry17ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry17_valid(&self) -> Keycam0Entry17ValidR {
        Keycam0Entry17ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry17_addr0(&self) -> Keycam0Entry17Addr0R {
        Keycam0Entry17Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry17_keyid_bit(
        &mut self,
    ) -> Keycam0Entry17KeyidBitW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry17_type(&mut self) -> Keycam0Entry17TypeW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry17_mickey_bit(
        &mut self,
    ) -> Keycam0Entry17MickeyBitW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry17_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry17GrpkeyBitW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry17_mgnt(&mut self) -> Keycam0Entry17MgntW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry17_repeater_mode(
        &mut self,
    ) -> Keycam0Entry17RepeaterModeW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry17_exttype(&mut self) -> Keycam0Entry17ExttypeW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry17_valid(&mut self) -> Keycam0Entry17ValidW<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry17_addr0(&mut self) -> Keycam0Entry17Addr0W<'_, RegKeycam0Entry17Spec> {
        Keycam0Entry17Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry17Spec;
impl crate::RegisterSpec for RegKeycam0Entry17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry17::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry17Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry17::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY17 to value 0"]
impl crate::Resettable for RegKeycam0Entry17Spec {}
