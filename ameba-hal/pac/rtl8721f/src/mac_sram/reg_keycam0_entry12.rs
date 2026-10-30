#[doc = "Register `REG_KEYCAM0_ENTRY12` reader"]
pub type R = crate::R<RegKeycam0Entry12Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY12` writer"]
pub type W = crate::W<RegKeycam0Entry12Spec>;
#[doc = "Field `KEYCAM0_ENTRY12_KEYID_BIT` reader - "]
pub type Keycam0Entry12KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY12_KEYID_BIT` writer - "]
pub type Keycam0Entry12KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY12_TYPE` reader - "]
pub type Keycam0Entry12TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY12_TYPE` writer - "]
pub type Keycam0Entry12TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY12_MICKEY_BIT` reader - "]
pub type Keycam0Entry12MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY12_MICKEY_BIT` writer - "]
pub type Keycam0Entry12MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY12_GRPKEY_BIT` reader - "]
pub type Keycam0Entry12GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY12_GRPKEY_BIT` writer - "]
pub type Keycam0Entry12GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY12_MGNT` reader - "]
pub type Keycam0Entry12MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY12_MGNT` writer - "]
pub type Keycam0Entry12MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY12_REPEATER_MODE` reader - "]
pub type Keycam0Entry12RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY12_REPEATER_MODE` writer - "]
pub type Keycam0Entry12RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY12_EXTTYPE` reader - "]
pub type Keycam0Entry12ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY12_EXTTYPE` writer - "]
pub type Keycam0Entry12ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY12_VALID` reader - "]
pub type Keycam0Entry12ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY12_VALID` writer - "]
pub type Keycam0Entry12ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY12_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry12Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY12_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry12Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry12_keyid_bit(&self) -> Keycam0Entry12KeyidBitR {
        Keycam0Entry12KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry12_type(&self) -> Keycam0Entry12TypeR {
        Keycam0Entry12TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry12_mickey_bit(&self) -> Keycam0Entry12MickeyBitR {
        Keycam0Entry12MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry12_grpkey_bit(&self) -> Keycam0Entry12GrpkeyBitR {
        Keycam0Entry12GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry12_mgnt(&self) -> Keycam0Entry12MgntR {
        Keycam0Entry12MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry12_repeater_mode(&self) -> Keycam0Entry12RepeaterModeR {
        Keycam0Entry12RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry12_exttype(&self) -> Keycam0Entry12ExttypeR {
        Keycam0Entry12ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry12_valid(&self) -> Keycam0Entry12ValidR {
        Keycam0Entry12ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry12_addr0(&self) -> Keycam0Entry12Addr0R {
        Keycam0Entry12Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry12_keyid_bit(
        &mut self,
    ) -> Keycam0Entry12KeyidBitW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry12_type(&mut self) -> Keycam0Entry12TypeW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry12_mickey_bit(
        &mut self,
    ) -> Keycam0Entry12MickeyBitW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry12_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry12GrpkeyBitW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry12_mgnt(&mut self) -> Keycam0Entry12MgntW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry12_repeater_mode(
        &mut self,
    ) -> Keycam0Entry12RepeaterModeW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry12_exttype(&mut self) -> Keycam0Entry12ExttypeW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry12_valid(&mut self) -> Keycam0Entry12ValidW<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry12_addr0(&mut self) -> Keycam0Entry12Addr0W<'_, RegKeycam0Entry12Spec> {
        Keycam0Entry12Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry12Spec;
impl crate::RegisterSpec for RegKeycam0Entry12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry12::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry12Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry12::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY12 to value 0"]
impl crate::Resettable for RegKeycam0Entry12Spec {}
