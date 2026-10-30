#[doc = "Register `REG_KEYCAM0_ENTRY21` reader"]
pub type R = crate::R<RegKeycam0Entry21Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY21` writer"]
pub type W = crate::W<RegKeycam0Entry21Spec>;
#[doc = "Field `KEYCAM0_ENTRY21_KEYID_BIT` reader - "]
pub type Keycam0Entry21KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY21_KEYID_BIT` writer - "]
pub type Keycam0Entry21KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY21_TYPE` reader - "]
pub type Keycam0Entry21TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY21_TYPE` writer - "]
pub type Keycam0Entry21TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY21_MICKEY_BIT` reader - "]
pub type Keycam0Entry21MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY21_MICKEY_BIT` writer - "]
pub type Keycam0Entry21MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY21_GRPKEY_BIT` reader - "]
pub type Keycam0Entry21GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY21_GRPKEY_BIT` writer - "]
pub type Keycam0Entry21GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY21_MGNT` reader - "]
pub type Keycam0Entry21MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY21_MGNT` writer - "]
pub type Keycam0Entry21MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY21_REPEATER_MODE` reader - "]
pub type Keycam0Entry21RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY21_REPEATER_MODE` writer - "]
pub type Keycam0Entry21RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY21_EXTTYPE` reader - "]
pub type Keycam0Entry21ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY21_EXTTYPE` writer - "]
pub type Keycam0Entry21ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY21_VALID` reader - "]
pub type Keycam0Entry21ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY21_VALID` writer - "]
pub type Keycam0Entry21ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY21_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry21Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY21_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry21Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry21_keyid_bit(&self) -> Keycam0Entry21KeyidBitR {
        Keycam0Entry21KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry21_type(&self) -> Keycam0Entry21TypeR {
        Keycam0Entry21TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry21_mickey_bit(&self) -> Keycam0Entry21MickeyBitR {
        Keycam0Entry21MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry21_grpkey_bit(&self) -> Keycam0Entry21GrpkeyBitR {
        Keycam0Entry21GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry21_mgnt(&self) -> Keycam0Entry21MgntR {
        Keycam0Entry21MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry21_repeater_mode(&self) -> Keycam0Entry21RepeaterModeR {
        Keycam0Entry21RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry21_exttype(&self) -> Keycam0Entry21ExttypeR {
        Keycam0Entry21ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry21_valid(&self) -> Keycam0Entry21ValidR {
        Keycam0Entry21ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry21_addr0(&self) -> Keycam0Entry21Addr0R {
        Keycam0Entry21Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry21_keyid_bit(
        &mut self,
    ) -> Keycam0Entry21KeyidBitW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry21_type(&mut self) -> Keycam0Entry21TypeW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry21_mickey_bit(
        &mut self,
    ) -> Keycam0Entry21MickeyBitW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry21_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry21GrpkeyBitW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry21_mgnt(&mut self) -> Keycam0Entry21MgntW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry21_repeater_mode(
        &mut self,
    ) -> Keycam0Entry21RepeaterModeW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry21_exttype(&mut self) -> Keycam0Entry21ExttypeW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry21_valid(&mut self) -> Keycam0Entry21ValidW<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry21_addr0(&mut self) -> Keycam0Entry21Addr0W<'_, RegKeycam0Entry21Spec> {
        Keycam0Entry21Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry21Spec;
impl crate::RegisterSpec for RegKeycam0Entry21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry21::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry21Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry21::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY21 to value 0"]
impl crate::Resettable for RegKeycam0Entry21Spec {}
