#[doc = "Register `REG_KEYCAM0_ENTRY30` reader"]
pub type R = crate::R<RegKeycam0Entry30Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY30` writer"]
pub type W = crate::W<RegKeycam0Entry30Spec>;
#[doc = "Field `KEYCAM0_ENTRY30_KEYID_BIT` reader - "]
pub type Keycam0Entry30KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY30_KEYID_BIT` writer - "]
pub type Keycam0Entry30KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY30_TYPE` reader - "]
pub type Keycam0Entry30TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY30_TYPE` writer - "]
pub type Keycam0Entry30TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY30_MICKEY_BIT` reader - "]
pub type Keycam0Entry30MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY30_MICKEY_BIT` writer - "]
pub type Keycam0Entry30MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY30_GRPKEY_BIT` reader - "]
pub type Keycam0Entry30GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY30_GRPKEY_BIT` writer - "]
pub type Keycam0Entry30GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY30_MGNT` reader - "]
pub type Keycam0Entry30MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY30_MGNT` writer - "]
pub type Keycam0Entry30MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY30_REPEATER_MODE` reader - "]
pub type Keycam0Entry30RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY30_REPEATER_MODE` writer - "]
pub type Keycam0Entry30RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY30_EXTTYPE` reader - "]
pub type Keycam0Entry30ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY30_EXTTYPE` writer - "]
pub type Keycam0Entry30ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY30_VALID` reader - "]
pub type Keycam0Entry30ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY30_VALID` writer - "]
pub type Keycam0Entry30ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY30_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry30Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY30_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry30Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry30_keyid_bit(&self) -> Keycam0Entry30KeyidBitR {
        Keycam0Entry30KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry30_type(&self) -> Keycam0Entry30TypeR {
        Keycam0Entry30TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry30_mickey_bit(&self) -> Keycam0Entry30MickeyBitR {
        Keycam0Entry30MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry30_grpkey_bit(&self) -> Keycam0Entry30GrpkeyBitR {
        Keycam0Entry30GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry30_mgnt(&self) -> Keycam0Entry30MgntR {
        Keycam0Entry30MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry30_repeater_mode(&self) -> Keycam0Entry30RepeaterModeR {
        Keycam0Entry30RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry30_exttype(&self) -> Keycam0Entry30ExttypeR {
        Keycam0Entry30ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry30_valid(&self) -> Keycam0Entry30ValidR {
        Keycam0Entry30ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry30_addr0(&self) -> Keycam0Entry30Addr0R {
        Keycam0Entry30Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry30_keyid_bit(
        &mut self,
    ) -> Keycam0Entry30KeyidBitW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry30_type(&mut self) -> Keycam0Entry30TypeW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry30_mickey_bit(
        &mut self,
    ) -> Keycam0Entry30MickeyBitW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry30_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry30GrpkeyBitW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry30_mgnt(&mut self) -> Keycam0Entry30MgntW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry30_repeater_mode(
        &mut self,
    ) -> Keycam0Entry30RepeaterModeW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry30_exttype(&mut self) -> Keycam0Entry30ExttypeW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry30_valid(&mut self) -> Keycam0Entry30ValidW<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry30_addr0(&mut self) -> Keycam0Entry30Addr0W<'_, RegKeycam0Entry30Spec> {
        Keycam0Entry30Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry30Spec;
impl crate::RegisterSpec for RegKeycam0Entry30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry30::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry30Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry30::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY30 to value 0"]
impl crate::Resettable for RegKeycam0Entry30Spec {}
