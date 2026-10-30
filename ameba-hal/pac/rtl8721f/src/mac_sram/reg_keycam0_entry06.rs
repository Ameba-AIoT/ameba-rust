#[doc = "Register `REG_KEYCAM0_ENTRY06` reader"]
pub type R = crate::R<RegKeycam0Entry06Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY06` writer"]
pub type W = crate::W<RegKeycam0Entry06Spec>;
#[doc = "Field `KEYCAM0_ENTRY06_KEYID_BIT` reader - "]
pub type Keycam0Entry06KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY06_KEYID_BIT` writer - "]
pub type Keycam0Entry06KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY06_TYPE` reader - "]
pub type Keycam0Entry06TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY06_TYPE` writer - "]
pub type Keycam0Entry06TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY06_MICKEY_BIT` reader - "]
pub type Keycam0Entry06MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY06_MICKEY_BIT` writer - "]
pub type Keycam0Entry06MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY06_GRPKEY_BIT` reader - "]
pub type Keycam0Entry06GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY06_GRPKEY_BIT` writer - "]
pub type Keycam0Entry06GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY06_MGNT` reader - "]
pub type Keycam0Entry06MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY06_MGNT` writer - "]
pub type Keycam0Entry06MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY06_REPEATER_MODE` reader - "]
pub type Keycam0Entry06RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY06_REPEATER_MODE` writer - "]
pub type Keycam0Entry06RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY06_EXTTYPE` reader - "]
pub type Keycam0Entry06ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY06_EXTTYPE` writer - "]
pub type Keycam0Entry06ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY06_VALID` reader - "]
pub type Keycam0Entry06ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY06_VALID` writer - "]
pub type Keycam0Entry06ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY06_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry06Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY06_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry06Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry06_keyid_bit(&self) -> Keycam0Entry06KeyidBitR {
        Keycam0Entry06KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry06_type(&self) -> Keycam0Entry06TypeR {
        Keycam0Entry06TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry06_mickey_bit(&self) -> Keycam0Entry06MickeyBitR {
        Keycam0Entry06MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry06_grpkey_bit(&self) -> Keycam0Entry06GrpkeyBitR {
        Keycam0Entry06GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry06_mgnt(&self) -> Keycam0Entry06MgntR {
        Keycam0Entry06MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry06_repeater_mode(&self) -> Keycam0Entry06RepeaterModeR {
        Keycam0Entry06RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry06_exttype(&self) -> Keycam0Entry06ExttypeR {
        Keycam0Entry06ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry06_valid(&self) -> Keycam0Entry06ValidR {
        Keycam0Entry06ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry06_addr0(&self) -> Keycam0Entry06Addr0R {
        Keycam0Entry06Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry06_keyid_bit(
        &mut self,
    ) -> Keycam0Entry06KeyidBitW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry06_type(&mut self) -> Keycam0Entry06TypeW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry06_mickey_bit(
        &mut self,
    ) -> Keycam0Entry06MickeyBitW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry06_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry06GrpkeyBitW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry06_mgnt(&mut self) -> Keycam0Entry06MgntW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry06_repeater_mode(
        &mut self,
    ) -> Keycam0Entry06RepeaterModeW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry06_exttype(&mut self) -> Keycam0Entry06ExttypeW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry06_valid(&mut self) -> Keycam0Entry06ValidW<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry06_addr0(&mut self) -> Keycam0Entry06Addr0W<'_, RegKeycam0Entry06Spec> {
        Keycam0Entry06Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry06::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry06::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry06Spec;
impl crate::RegisterSpec for RegKeycam0Entry06Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry06::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry06Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry06::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry06Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY06 to value 0"]
impl crate::Resettable for RegKeycam0Entry06Spec {}
