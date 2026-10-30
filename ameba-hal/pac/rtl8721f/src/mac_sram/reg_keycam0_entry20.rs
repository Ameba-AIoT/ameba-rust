#[doc = "Register `REG_KEYCAM0_ENTRY20` reader"]
pub type R = crate::R<RegKeycam0Entry20Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY20` writer"]
pub type W = crate::W<RegKeycam0Entry20Spec>;
#[doc = "Field `KEYCAM0_ENTRY20_KEYID_BIT` reader - "]
pub type Keycam0Entry20KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY20_KEYID_BIT` writer - "]
pub type Keycam0Entry20KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY20_TYPE` reader - "]
pub type Keycam0Entry20TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY20_TYPE` writer - "]
pub type Keycam0Entry20TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY20_MICKEY_BIT` reader - "]
pub type Keycam0Entry20MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY20_MICKEY_BIT` writer - "]
pub type Keycam0Entry20MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY20_GRPKEY_BIT` reader - "]
pub type Keycam0Entry20GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY20_GRPKEY_BIT` writer - "]
pub type Keycam0Entry20GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY20_MGNT` reader - "]
pub type Keycam0Entry20MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY20_MGNT` writer - "]
pub type Keycam0Entry20MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY20_REPEATER_MODE` reader - "]
pub type Keycam0Entry20RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY20_REPEATER_MODE` writer - "]
pub type Keycam0Entry20RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY20_EXTTYPE` reader - "]
pub type Keycam0Entry20ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY20_EXTTYPE` writer - "]
pub type Keycam0Entry20ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY20_VALID` reader - "]
pub type Keycam0Entry20ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY20_VALID` writer - "]
pub type Keycam0Entry20ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY20_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry20Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY20_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry20Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry20_keyid_bit(&self) -> Keycam0Entry20KeyidBitR {
        Keycam0Entry20KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry20_type(&self) -> Keycam0Entry20TypeR {
        Keycam0Entry20TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry20_mickey_bit(&self) -> Keycam0Entry20MickeyBitR {
        Keycam0Entry20MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry20_grpkey_bit(&self) -> Keycam0Entry20GrpkeyBitR {
        Keycam0Entry20GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry20_mgnt(&self) -> Keycam0Entry20MgntR {
        Keycam0Entry20MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry20_repeater_mode(&self) -> Keycam0Entry20RepeaterModeR {
        Keycam0Entry20RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry20_exttype(&self) -> Keycam0Entry20ExttypeR {
        Keycam0Entry20ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry20_valid(&self) -> Keycam0Entry20ValidR {
        Keycam0Entry20ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry20_addr0(&self) -> Keycam0Entry20Addr0R {
        Keycam0Entry20Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry20_keyid_bit(
        &mut self,
    ) -> Keycam0Entry20KeyidBitW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry20_type(&mut self) -> Keycam0Entry20TypeW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry20_mickey_bit(
        &mut self,
    ) -> Keycam0Entry20MickeyBitW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry20_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry20GrpkeyBitW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry20_mgnt(&mut self) -> Keycam0Entry20MgntW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry20_repeater_mode(
        &mut self,
    ) -> Keycam0Entry20RepeaterModeW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry20_exttype(&mut self) -> Keycam0Entry20ExttypeW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry20_valid(&mut self) -> Keycam0Entry20ValidW<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry20_addr0(&mut self) -> Keycam0Entry20Addr0W<'_, RegKeycam0Entry20Spec> {
        Keycam0Entry20Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry20Spec;
impl crate::RegisterSpec for RegKeycam0Entry20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry20::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry20Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry20::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY20 to value 0"]
impl crate::Resettable for RegKeycam0Entry20Spec {}
