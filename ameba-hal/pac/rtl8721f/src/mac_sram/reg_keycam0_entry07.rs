#[doc = "Register `REG_KEYCAM0_ENTRY07` reader"]
pub type R = crate::R<RegKeycam0Entry07Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY07` writer"]
pub type W = crate::W<RegKeycam0Entry07Spec>;
#[doc = "Field `KEYCAM0_ENTRY07_KEYID_BIT` reader - "]
pub type Keycam0Entry07KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY07_KEYID_BIT` writer - "]
pub type Keycam0Entry07KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY07_TYPE` reader - "]
pub type Keycam0Entry07TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY07_TYPE` writer - "]
pub type Keycam0Entry07TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY07_MICKEY_BIT` reader - "]
pub type Keycam0Entry07MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY07_MICKEY_BIT` writer - "]
pub type Keycam0Entry07MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY07_GRPKEY_BIT` reader - "]
pub type Keycam0Entry07GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY07_GRPKEY_BIT` writer - "]
pub type Keycam0Entry07GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY07_MGNT` reader - "]
pub type Keycam0Entry07MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY07_MGNT` writer - "]
pub type Keycam0Entry07MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY07_REPEATER_MODE` reader - "]
pub type Keycam0Entry07RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY07_REPEATER_MODE` writer - "]
pub type Keycam0Entry07RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY07_EXTTYPE` reader - "]
pub type Keycam0Entry07ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY07_EXTTYPE` writer - "]
pub type Keycam0Entry07ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY07_VALID` reader - "]
pub type Keycam0Entry07ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY07_VALID` writer - "]
pub type Keycam0Entry07ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY07_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry07Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY07_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry07Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry07_keyid_bit(&self) -> Keycam0Entry07KeyidBitR {
        Keycam0Entry07KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry07_type(&self) -> Keycam0Entry07TypeR {
        Keycam0Entry07TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry07_mickey_bit(&self) -> Keycam0Entry07MickeyBitR {
        Keycam0Entry07MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry07_grpkey_bit(&self) -> Keycam0Entry07GrpkeyBitR {
        Keycam0Entry07GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry07_mgnt(&self) -> Keycam0Entry07MgntR {
        Keycam0Entry07MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry07_repeater_mode(&self) -> Keycam0Entry07RepeaterModeR {
        Keycam0Entry07RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry07_exttype(&self) -> Keycam0Entry07ExttypeR {
        Keycam0Entry07ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry07_valid(&self) -> Keycam0Entry07ValidR {
        Keycam0Entry07ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry07_addr0(&self) -> Keycam0Entry07Addr0R {
        Keycam0Entry07Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry07_keyid_bit(
        &mut self,
    ) -> Keycam0Entry07KeyidBitW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry07_type(&mut self) -> Keycam0Entry07TypeW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry07_mickey_bit(
        &mut self,
    ) -> Keycam0Entry07MickeyBitW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry07_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry07GrpkeyBitW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry07_mgnt(&mut self) -> Keycam0Entry07MgntW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry07_repeater_mode(
        &mut self,
    ) -> Keycam0Entry07RepeaterModeW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry07_exttype(&mut self) -> Keycam0Entry07ExttypeW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry07_valid(&mut self) -> Keycam0Entry07ValidW<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry07_addr0(&mut self) -> Keycam0Entry07Addr0W<'_, RegKeycam0Entry07Spec> {
        Keycam0Entry07Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry07::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry07::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry07Spec;
impl crate::RegisterSpec for RegKeycam0Entry07Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry07::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry07Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry07::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry07Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY07 to value 0"]
impl crate::Resettable for RegKeycam0Entry07Spec {}
