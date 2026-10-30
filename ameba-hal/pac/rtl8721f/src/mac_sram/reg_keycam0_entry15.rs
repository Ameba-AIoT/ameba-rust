#[doc = "Register `REG_KEYCAM0_ENTRY15` reader"]
pub type R = crate::R<RegKeycam0Entry15Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY15` writer"]
pub type W = crate::W<RegKeycam0Entry15Spec>;
#[doc = "Field `KEYCAM0_ENTRY15_KEYID_BIT` reader - "]
pub type Keycam0Entry15KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY15_KEYID_BIT` writer - "]
pub type Keycam0Entry15KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY15_TYPE` reader - "]
pub type Keycam0Entry15TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY15_TYPE` writer - "]
pub type Keycam0Entry15TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY15_MICKEY_BIT` reader - "]
pub type Keycam0Entry15MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY15_MICKEY_BIT` writer - "]
pub type Keycam0Entry15MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY15_GRPKEY_BIT` reader - "]
pub type Keycam0Entry15GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY15_GRPKEY_BIT` writer - "]
pub type Keycam0Entry15GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY15_MGNT` reader - "]
pub type Keycam0Entry15MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY15_MGNT` writer - "]
pub type Keycam0Entry15MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY15_REPEATER_MODE` reader - "]
pub type Keycam0Entry15RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY15_REPEATER_MODE` writer - "]
pub type Keycam0Entry15RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY15_EXTTYPE` reader - "]
pub type Keycam0Entry15ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY15_EXTTYPE` writer - "]
pub type Keycam0Entry15ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY15_VALID` reader - "]
pub type Keycam0Entry15ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY15_VALID` writer - "]
pub type Keycam0Entry15ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY15_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry15Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY15_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry15Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry15_keyid_bit(&self) -> Keycam0Entry15KeyidBitR {
        Keycam0Entry15KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry15_type(&self) -> Keycam0Entry15TypeR {
        Keycam0Entry15TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry15_mickey_bit(&self) -> Keycam0Entry15MickeyBitR {
        Keycam0Entry15MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry15_grpkey_bit(&self) -> Keycam0Entry15GrpkeyBitR {
        Keycam0Entry15GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry15_mgnt(&self) -> Keycam0Entry15MgntR {
        Keycam0Entry15MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry15_repeater_mode(&self) -> Keycam0Entry15RepeaterModeR {
        Keycam0Entry15RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry15_exttype(&self) -> Keycam0Entry15ExttypeR {
        Keycam0Entry15ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry15_valid(&self) -> Keycam0Entry15ValidR {
        Keycam0Entry15ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry15_addr0(&self) -> Keycam0Entry15Addr0R {
        Keycam0Entry15Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry15_keyid_bit(
        &mut self,
    ) -> Keycam0Entry15KeyidBitW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry15_type(&mut self) -> Keycam0Entry15TypeW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry15_mickey_bit(
        &mut self,
    ) -> Keycam0Entry15MickeyBitW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry15_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry15GrpkeyBitW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry15_mgnt(&mut self) -> Keycam0Entry15MgntW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry15_repeater_mode(
        &mut self,
    ) -> Keycam0Entry15RepeaterModeW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry15_exttype(&mut self) -> Keycam0Entry15ExttypeW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry15_valid(&mut self) -> Keycam0Entry15ValidW<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry15_addr0(&mut self) -> Keycam0Entry15Addr0W<'_, RegKeycam0Entry15Spec> {
        Keycam0Entry15Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry15Spec;
impl crate::RegisterSpec for RegKeycam0Entry15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry15::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry15Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry15::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY15 to value 0"]
impl crate::Resettable for RegKeycam0Entry15Spec {}
