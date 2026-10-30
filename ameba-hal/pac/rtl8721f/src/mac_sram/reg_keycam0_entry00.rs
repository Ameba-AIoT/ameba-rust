#[doc = "Register `REG_KEYCAM0_ENTRY00` reader"]
pub type R = crate::R<RegKeycam0Entry00Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY00` writer"]
pub type W = crate::W<RegKeycam0Entry00Spec>;
#[doc = "Field `KEYCAM0_ENTRY00_KEYID_BIT` reader - "]
pub type Keycam0Entry00KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY00_KEYID_BIT` writer - "]
pub type Keycam0Entry00KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY00_TYPE` reader - "]
pub type Keycam0Entry00TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY00_TYPE` writer - "]
pub type Keycam0Entry00TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY00_MICKEY_BIT` reader - "]
pub type Keycam0Entry00MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY00_MICKEY_BIT` writer - "]
pub type Keycam0Entry00MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY00_GRPKEY_BIT` reader - "]
pub type Keycam0Entry00GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY00_GRPKEY_BIT` writer - "]
pub type Keycam0Entry00GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY00_MGNT` reader - "]
pub type Keycam0Entry00MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY00_MGNT` writer - "]
pub type Keycam0Entry00MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY00_REPEATER_MODE` reader - "]
pub type Keycam0Entry00RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY00_REPEATER_MODE` writer - "]
pub type Keycam0Entry00RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY00_EXTTYPE` reader - "]
pub type Keycam0Entry00ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY00_EXTTYPE` writer - "]
pub type Keycam0Entry00ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY00_VALID` reader - "]
pub type Keycam0Entry00ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY00_VALID` writer - "]
pub type Keycam0Entry00ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY00_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry00Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY00_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry00Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry00_keyid_bit(&self) -> Keycam0Entry00KeyidBitR {
        Keycam0Entry00KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry00_type(&self) -> Keycam0Entry00TypeR {
        Keycam0Entry00TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry00_mickey_bit(&self) -> Keycam0Entry00MickeyBitR {
        Keycam0Entry00MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry00_grpkey_bit(&self) -> Keycam0Entry00GrpkeyBitR {
        Keycam0Entry00GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry00_mgnt(&self) -> Keycam0Entry00MgntR {
        Keycam0Entry00MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry00_repeater_mode(&self) -> Keycam0Entry00RepeaterModeR {
        Keycam0Entry00RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry00_exttype(&self) -> Keycam0Entry00ExttypeR {
        Keycam0Entry00ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry00_valid(&self) -> Keycam0Entry00ValidR {
        Keycam0Entry00ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry00_addr0(&self) -> Keycam0Entry00Addr0R {
        Keycam0Entry00Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry00_keyid_bit(
        &mut self,
    ) -> Keycam0Entry00KeyidBitW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry00_type(&mut self) -> Keycam0Entry00TypeW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry00_mickey_bit(
        &mut self,
    ) -> Keycam0Entry00MickeyBitW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry00_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry00GrpkeyBitW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry00_mgnt(&mut self) -> Keycam0Entry00MgntW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry00_repeater_mode(
        &mut self,
    ) -> Keycam0Entry00RepeaterModeW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry00_exttype(&mut self) -> Keycam0Entry00ExttypeW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry00_valid(&mut self) -> Keycam0Entry00ValidW<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry00_addr0(&mut self) -> Keycam0Entry00Addr0W<'_, RegKeycam0Entry00Spec> {
        Keycam0Entry00Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry00Spec;
impl crate::RegisterSpec for RegKeycam0Entry00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry00::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry00Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry00::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY00 to value 0"]
impl crate::Resettable for RegKeycam0Entry00Spec {}
