#[doc = "Register `REG_KEYCAM0_ENTRY28` reader"]
pub type R = crate::R<RegKeycam0Entry28Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY28` writer"]
pub type W = crate::W<RegKeycam0Entry28Spec>;
#[doc = "Field `KEYCAM0_ENTRY28_KEYID_BIT` reader - "]
pub type Keycam0Entry28KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY28_KEYID_BIT` writer - "]
pub type Keycam0Entry28KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY28_TYPE` reader - "]
pub type Keycam0Entry28TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY28_TYPE` writer - "]
pub type Keycam0Entry28TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY28_MICKEY_BIT` reader - "]
pub type Keycam0Entry28MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY28_MICKEY_BIT` writer - "]
pub type Keycam0Entry28MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY28_GRPKEY_BIT` reader - "]
pub type Keycam0Entry28GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY28_GRPKEY_BIT` writer - "]
pub type Keycam0Entry28GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY28_MGNT` reader - "]
pub type Keycam0Entry28MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY28_MGNT` writer - "]
pub type Keycam0Entry28MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY28_REPEATER_MODE` reader - "]
pub type Keycam0Entry28RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY28_REPEATER_MODE` writer - "]
pub type Keycam0Entry28RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY28_EXTTYPE` reader - "]
pub type Keycam0Entry28ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY28_EXTTYPE` writer - "]
pub type Keycam0Entry28ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY28_VALID` reader - "]
pub type Keycam0Entry28ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY28_VALID` writer - "]
pub type Keycam0Entry28ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY28_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry28Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY28_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry28Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry28_keyid_bit(&self) -> Keycam0Entry28KeyidBitR {
        Keycam0Entry28KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry28_type(&self) -> Keycam0Entry28TypeR {
        Keycam0Entry28TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry28_mickey_bit(&self) -> Keycam0Entry28MickeyBitR {
        Keycam0Entry28MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry28_grpkey_bit(&self) -> Keycam0Entry28GrpkeyBitR {
        Keycam0Entry28GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry28_mgnt(&self) -> Keycam0Entry28MgntR {
        Keycam0Entry28MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry28_repeater_mode(&self) -> Keycam0Entry28RepeaterModeR {
        Keycam0Entry28RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry28_exttype(&self) -> Keycam0Entry28ExttypeR {
        Keycam0Entry28ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry28_valid(&self) -> Keycam0Entry28ValidR {
        Keycam0Entry28ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry28_addr0(&self) -> Keycam0Entry28Addr0R {
        Keycam0Entry28Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry28_keyid_bit(
        &mut self,
    ) -> Keycam0Entry28KeyidBitW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry28_type(&mut self) -> Keycam0Entry28TypeW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry28_mickey_bit(
        &mut self,
    ) -> Keycam0Entry28MickeyBitW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry28_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry28GrpkeyBitW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry28_mgnt(&mut self) -> Keycam0Entry28MgntW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry28_repeater_mode(
        &mut self,
    ) -> Keycam0Entry28RepeaterModeW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry28_exttype(&mut self) -> Keycam0Entry28ExttypeW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry28_valid(&mut self) -> Keycam0Entry28ValidW<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry28_addr0(&mut self) -> Keycam0Entry28Addr0W<'_, RegKeycam0Entry28Spec> {
        Keycam0Entry28Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry28Spec;
impl crate::RegisterSpec for RegKeycam0Entry28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry28::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry28Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry28::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY28 to value 0"]
impl crate::Resettable for RegKeycam0Entry28Spec {}
