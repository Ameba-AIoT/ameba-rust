#[doc = "Register `REG_KEYCAM0_ENTRY18` reader"]
pub type R = crate::R<RegKeycam0Entry18Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY18` writer"]
pub type W = crate::W<RegKeycam0Entry18Spec>;
#[doc = "Field `KEYCAM0_ENTRY18_KEYID_BIT` reader - "]
pub type Keycam0Entry18KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY18_KEYID_BIT` writer - "]
pub type Keycam0Entry18KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY18_TYPE` reader - "]
pub type Keycam0Entry18TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY18_TYPE` writer - "]
pub type Keycam0Entry18TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY18_MICKEY_BIT` reader - "]
pub type Keycam0Entry18MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY18_MICKEY_BIT` writer - "]
pub type Keycam0Entry18MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY18_GRPKEY_BIT` reader - "]
pub type Keycam0Entry18GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY18_GRPKEY_BIT` writer - "]
pub type Keycam0Entry18GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY18_MGNT` reader - "]
pub type Keycam0Entry18MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY18_MGNT` writer - "]
pub type Keycam0Entry18MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY18_REPEATER_MODE` reader - "]
pub type Keycam0Entry18RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY18_REPEATER_MODE` writer - "]
pub type Keycam0Entry18RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY18_EXTTYPE` reader - "]
pub type Keycam0Entry18ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY18_EXTTYPE` writer - "]
pub type Keycam0Entry18ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY18_VALID` reader - "]
pub type Keycam0Entry18ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY18_VALID` writer - "]
pub type Keycam0Entry18ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY18_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry18Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY18_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry18Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry18_keyid_bit(&self) -> Keycam0Entry18KeyidBitR {
        Keycam0Entry18KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry18_type(&self) -> Keycam0Entry18TypeR {
        Keycam0Entry18TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry18_mickey_bit(&self) -> Keycam0Entry18MickeyBitR {
        Keycam0Entry18MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry18_grpkey_bit(&self) -> Keycam0Entry18GrpkeyBitR {
        Keycam0Entry18GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry18_mgnt(&self) -> Keycam0Entry18MgntR {
        Keycam0Entry18MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry18_repeater_mode(&self) -> Keycam0Entry18RepeaterModeR {
        Keycam0Entry18RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry18_exttype(&self) -> Keycam0Entry18ExttypeR {
        Keycam0Entry18ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry18_valid(&self) -> Keycam0Entry18ValidR {
        Keycam0Entry18ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry18_addr0(&self) -> Keycam0Entry18Addr0R {
        Keycam0Entry18Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry18_keyid_bit(
        &mut self,
    ) -> Keycam0Entry18KeyidBitW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry18_type(&mut self) -> Keycam0Entry18TypeW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry18_mickey_bit(
        &mut self,
    ) -> Keycam0Entry18MickeyBitW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry18_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry18GrpkeyBitW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry18_mgnt(&mut self) -> Keycam0Entry18MgntW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry18_repeater_mode(
        &mut self,
    ) -> Keycam0Entry18RepeaterModeW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry18_exttype(&mut self) -> Keycam0Entry18ExttypeW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry18_valid(&mut self) -> Keycam0Entry18ValidW<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry18_addr0(&mut self) -> Keycam0Entry18Addr0W<'_, RegKeycam0Entry18Spec> {
        Keycam0Entry18Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry18Spec;
impl crate::RegisterSpec for RegKeycam0Entry18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry18::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry18::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY18 to value 0"]
impl crate::Resettable for RegKeycam0Entry18Spec {}
