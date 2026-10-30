#[doc = "Register `REG_KEYCAM0_ENTRY04` reader"]
pub type R = crate::R<RegKeycam0Entry04Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY04` writer"]
pub type W = crate::W<RegKeycam0Entry04Spec>;
#[doc = "Field `KEYCAM0_ENTRY04_KEYID_BIT` reader - "]
pub type Keycam0Entry04KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY04_KEYID_BIT` writer - "]
pub type Keycam0Entry04KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY04_TYPE` reader - "]
pub type Keycam0Entry04TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY04_TYPE` writer - "]
pub type Keycam0Entry04TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY04_MICKEY_BIT` reader - "]
pub type Keycam0Entry04MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY04_MICKEY_BIT` writer - "]
pub type Keycam0Entry04MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY04_GRPKEY_BIT` reader - "]
pub type Keycam0Entry04GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY04_GRPKEY_BIT` writer - "]
pub type Keycam0Entry04GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY04_MGNT` reader - "]
pub type Keycam0Entry04MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY04_MGNT` writer - "]
pub type Keycam0Entry04MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY04_REPEATER_MODE` reader - "]
pub type Keycam0Entry04RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY04_REPEATER_MODE` writer - "]
pub type Keycam0Entry04RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY04_EXTTYPE` reader - "]
pub type Keycam0Entry04ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY04_EXTTYPE` writer - "]
pub type Keycam0Entry04ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY04_VALID` reader - "]
pub type Keycam0Entry04ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY04_VALID` writer - "]
pub type Keycam0Entry04ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY04_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry04Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY04_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry04Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry04_keyid_bit(&self) -> Keycam0Entry04KeyidBitR {
        Keycam0Entry04KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry04_type(&self) -> Keycam0Entry04TypeR {
        Keycam0Entry04TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry04_mickey_bit(&self) -> Keycam0Entry04MickeyBitR {
        Keycam0Entry04MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry04_grpkey_bit(&self) -> Keycam0Entry04GrpkeyBitR {
        Keycam0Entry04GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry04_mgnt(&self) -> Keycam0Entry04MgntR {
        Keycam0Entry04MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry04_repeater_mode(&self) -> Keycam0Entry04RepeaterModeR {
        Keycam0Entry04RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry04_exttype(&self) -> Keycam0Entry04ExttypeR {
        Keycam0Entry04ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry04_valid(&self) -> Keycam0Entry04ValidR {
        Keycam0Entry04ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry04_addr0(&self) -> Keycam0Entry04Addr0R {
        Keycam0Entry04Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry04_keyid_bit(
        &mut self,
    ) -> Keycam0Entry04KeyidBitW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry04_type(&mut self) -> Keycam0Entry04TypeW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry04_mickey_bit(
        &mut self,
    ) -> Keycam0Entry04MickeyBitW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry04_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry04GrpkeyBitW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry04_mgnt(&mut self) -> Keycam0Entry04MgntW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry04_repeater_mode(
        &mut self,
    ) -> Keycam0Entry04RepeaterModeW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry04_exttype(&mut self) -> Keycam0Entry04ExttypeW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry04_valid(&mut self) -> Keycam0Entry04ValidW<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry04_addr0(&mut self) -> Keycam0Entry04Addr0W<'_, RegKeycam0Entry04Spec> {
        Keycam0Entry04Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry04Spec;
impl crate::RegisterSpec for RegKeycam0Entry04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry04::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry04Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry04::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY04 to value 0"]
impl crate::Resettable for RegKeycam0Entry04Spec {}
