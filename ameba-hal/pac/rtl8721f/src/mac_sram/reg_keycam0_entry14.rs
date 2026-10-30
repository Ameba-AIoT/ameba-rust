#[doc = "Register `REG_KEYCAM0_ENTRY14` reader"]
pub type R = crate::R<RegKeycam0Entry14Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY14` writer"]
pub type W = crate::W<RegKeycam0Entry14Spec>;
#[doc = "Field `KEYCAM0_ENTRY14_KEYID_BIT` reader - "]
pub type Keycam0Entry14KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY14_KEYID_BIT` writer - "]
pub type Keycam0Entry14KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY14_TYPE` reader - "]
pub type Keycam0Entry14TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY14_TYPE` writer - "]
pub type Keycam0Entry14TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY14_MICKEY_BIT` reader - "]
pub type Keycam0Entry14MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY14_MICKEY_BIT` writer - "]
pub type Keycam0Entry14MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY14_GRPKEY_BIT` reader - "]
pub type Keycam0Entry14GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY14_GRPKEY_BIT` writer - "]
pub type Keycam0Entry14GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY14_MGNT` reader - "]
pub type Keycam0Entry14MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY14_MGNT` writer - "]
pub type Keycam0Entry14MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY14_REPEATER_MODE` reader - "]
pub type Keycam0Entry14RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY14_REPEATER_MODE` writer - "]
pub type Keycam0Entry14RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY14_EXTTYPE` reader - "]
pub type Keycam0Entry14ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY14_EXTTYPE` writer - "]
pub type Keycam0Entry14ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY14_VALID` reader - "]
pub type Keycam0Entry14ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY14_VALID` writer - "]
pub type Keycam0Entry14ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY14_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry14Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY14_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry14Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry14_keyid_bit(&self) -> Keycam0Entry14KeyidBitR {
        Keycam0Entry14KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry14_type(&self) -> Keycam0Entry14TypeR {
        Keycam0Entry14TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry14_mickey_bit(&self) -> Keycam0Entry14MickeyBitR {
        Keycam0Entry14MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry14_grpkey_bit(&self) -> Keycam0Entry14GrpkeyBitR {
        Keycam0Entry14GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry14_mgnt(&self) -> Keycam0Entry14MgntR {
        Keycam0Entry14MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry14_repeater_mode(&self) -> Keycam0Entry14RepeaterModeR {
        Keycam0Entry14RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry14_exttype(&self) -> Keycam0Entry14ExttypeR {
        Keycam0Entry14ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry14_valid(&self) -> Keycam0Entry14ValidR {
        Keycam0Entry14ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry14_addr0(&self) -> Keycam0Entry14Addr0R {
        Keycam0Entry14Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry14_keyid_bit(
        &mut self,
    ) -> Keycam0Entry14KeyidBitW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry14_type(&mut self) -> Keycam0Entry14TypeW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry14_mickey_bit(
        &mut self,
    ) -> Keycam0Entry14MickeyBitW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry14_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry14GrpkeyBitW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry14_mgnt(&mut self) -> Keycam0Entry14MgntW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry14_repeater_mode(
        &mut self,
    ) -> Keycam0Entry14RepeaterModeW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry14_exttype(&mut self) -> Keycam0Entry14ExttypeW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry14_valid(&mut self) -> Keycam0Entry14ValidW<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry14_addr0(&mut self) -> Keycam0Entry14Addr0W<'_, RegKeycam0Entry14Spec> {
        Keycam0Entry14Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry14Spec;
impl crate::RegisterSpec for RegKeycam0Entry14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry14::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry14::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY14 to value 0"]
impl crate::Resettable for RegKeycam0Entry14Spec {}
