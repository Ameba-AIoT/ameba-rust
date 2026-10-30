#[doc = "Register `REG_KEYCAM0_ENTRY13` reader"]
pub type R = crate::R<RegKeycam0Entry13Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY13` writer"]
pub type W = crate::W<RegKeycam0Entry13Spec>;
#[doc = "Field `KEYCAM0_ENTRY13_KEYID_BIT` reader - "]
pub type Keycam0Entry13KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY13_KEYID_BIT` writer - "]
pub type Keycam0Entry13KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY13_TYPE` reader - "]
pub type Keycam0Entry13TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY13_TYPE` writer - "]
pub type Keycam0Entry13TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY13_MICKEY_BIT` reader - "]
pub type Keycam0Entry13MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY13_MICKEY_BIT` writer - "]
pub type Keycam0Entry13MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY13_GRPKEY_BIT` reader - "]
pub type Keycam0Entry13GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY13_GRPKEY_BIT` writer - "]
pub type Keycam0Entry13GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY13_MGNT` reader - "]
pub type Keycam0Entry13MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY13_MGNT` writer - "]
pub type Keycam0Entry13MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY13_REPEATER_MODE` reader - "]
pub type Keycam0Entry13RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY13_REPEATER_MODE` writer - "]
pub type Keycam0Entry13RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY13_EXTTYPE` reader - "]
pub type Keycam0Entry13ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY13_EXTTYPE` writer - "]
pub type Keycam0Entry13ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY13_VALID` reader - "]
pub type Keycam0Entry13ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY13_VALID` writer - "]
pub type Keycam0Entry13ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY13_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry13Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY13_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry13Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry13_keyid_bit(&self) -> Keycam0Entry13KeyidBitR {
        Keycam0Entry13KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry13_type(&self) -> Keycam0Entry13TypeR {
        Keycam0Entry13TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry13_mickey_bit(&self) -> Keycam0Entry13MickeyBitR {
        Keycam0Entry13MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry13_grpkey_bit(&self) -> Keycam0Entry13GrpkeyBitR {
        Keycam0Entry13GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry13_mgnt(&self) -> Keycam0Entry13MgntR {
        Keycam0Entry13MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry13_repeater_mode(&self) -> Keycam0Entry13RepeaterModeR {
        Keycam0Entry13RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry13_exttype(&self) -> Keycam0Entry13ExttypeR {
        Keycam0Entry13ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry13_valid(&self) -> Keycam0Entry13ValidR {
        Keycam0Entry13ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry13_addr0(&self) -> Keycam0Entry13Addr0R {
        Keycam0Entry13Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry13_keyid_bit(
        &mut self,
    ) -> Keycam0Entry13KeyidBitW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry13_type(&mut self) -> Keycam0Entry13TypeW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry13_mickey_bit(
        &mut self,
    ) -> Keycam0Entry13MickeyBitW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry13_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry13GrpkeyBitW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry13_mgnt(&mut self) -> Keycam0Entry13MgntW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry13_repeater_mode(
        &mut self,
    ) -> Keycam0Entry13RepeaterModeW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry13_exttype(&mut self) -> Keycam0Entry13ExttypeW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry13_valid(&mut self) -> Keycam0Entry13ValidW<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry13_addr0(&mut self) -> Keycam0Entry13Addr0W<'_, RegKeycam0Entry13Spec> {
        Keycam0Entry13Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry13Spec;
impl crate::RegisterSpec for RegKeycam0Entry13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry13::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry13::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY13 to value 0"]
impl crate::Resettable for RegKeycam0Entry13Spec {}
