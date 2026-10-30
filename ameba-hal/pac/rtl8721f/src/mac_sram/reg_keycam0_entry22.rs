#[doc = "Register `REG_KEYCAM0_ENTRY22` reader"]
pub type R = crate::R<RegKeycam0Entry22Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY22` writer"]
pub type W = crate::W<RegKeycam0Entry22Spec>;
#[doc = "Field `KEYCAM0_ENTRY22_KEYID_BIT` reader - "]
pub type Keycam0Entry22KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY22_KEYID_BIT` writer - "]
pub type Keycam0Entry22KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY22_TYPE` reader - "]
pub type Keycam0Entry22TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY22_TYPE` writer - "]
pub type Keycam0Entry22TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY22_MICKEY_BIT` reader - "]
pub type Keycam0Entry22MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY22_MICKEY_BIT` writer - "]
pub type Keycam0Entry22MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY22_GRPKEY_BIT` reader - "]
pub type Keycam0Entry22GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY22_GRPKEY_BIT` writer - "]
pub type Keycam0Entry22GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY22_MGNT` reader - "]
pub type Keycam0Entry22MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY22_MGNT` writer - "]
pub type Keycam0Entry22MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY22_REPEATER_MODE` reader - "]
pub type Keycam0Entry22RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY22_REPEATER_MODE` writer - "]
pub type Keycam0Entry22RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY22_EXTTYPE` reader - "]
pub type Keycam0Entry22ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY22_EXTTYPE` writer - "]
pub type Keycam0Entry22ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY22_VALID` reader - "]
pub type Keycam0Entry22ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY22_VALID` writer - "]
pub type Keycam0Entry22ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY22_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry22Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY22_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry22Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry22_keyid_bit(&self) -> Keycam0Entry22KeyidBitR {
        Keycam0Entry22KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry22_type(&self) -> Keycam0Entry22TypeR {
        Keycam0Entry22TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry22_mickey_bit(&self) -> Keycam0Entry22MickeyBitR {
        Keycam0Entry22MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry22_grpkey_bit(&self) -> Keycam0Entry22GrpkeyBitR {
        Keycam0Entry22GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry22_mgnt(&self) -> Keycam0Entry22MgntR {
        Keycam0Entry22MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry22_repeater_mode(&self) -> Keycam0Entry22RepeaterModeR {
        Keycam0Entry22RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry22_exttype(&self) -> Keycam0Entry22ExttypeR {
        Keycam0Entry22ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry22_valid(&self) -> Keycam0Entry22ValidR {
        Keycam0Entry22ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry22_addr0(&self) -> Keycam0Entry22Addr0R {
        Keycam0Entry22Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry22_keyid_bit(
        &mut self,
    ) -> Keycam0Entry22KeyidBitW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry22_type(&mut self) -> Keycam0Entry22TypeW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry22_mickey_bit(
        &mut self,
    ) -> Keycam0Entry22MickeyBitW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry22_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry22GrpkeyBitW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry22_mgnt(&mut self) -> Keycam0Entry22MgntW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry22_repeater_mode(
        &mut self,
    ) -> Keycam0Entry22RepeaterModeW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry22_exttype(&mut self) -> Keycam0Entry22ExttypeW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry22_valid(&mut self) -> Keycam0Entry22ValidW<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry22_addr0(&mut self) -> Keycam0Entry22Addr0W<'_, RegKeycam0Entry22Spec> {
        Keycam0Entry22Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry22Spec;
impl crate::RegisterSpec for RegKeycam0Entry22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry22::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry22Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry22::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY22 to value 0"]
impl crate::Resettable for RegKeycam0Entry22Spec {}
