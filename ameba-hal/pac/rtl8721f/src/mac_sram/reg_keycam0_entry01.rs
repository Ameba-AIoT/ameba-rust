#[doc = "Register `REG_KEYCAM0_ENTRY01` reader"]
pub type R = crate::R<RegKeycam0Entry01Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY01` writer"]
pub type W = crate::W<RegKeycam0Entry01Spec>;
#[doc = "Field `KEYCAM0_ENTRY01_KEYID_BIT` reader - "]
pub type Keycam0Entry01KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY01_KEYID_BIT` writer - "]
pub type Keycam0Entry01KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY01_TYPE` reader - "]
pub type Keycam0Entry01TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY01_TYPE` writer - "]
pub type Keycam0Entry01TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY01_MICKEY_BIT` reader - "]
pub type Keycam0Entry01MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY01_MICKEY_BIT` writer - "]
pub type Keycam0Entry01MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY01_GRPKEY_BIT` reader - "]
pub type Keycam0Entry01GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY01_GRPKEY_BIT` writer - "]
pub type Keycam0Entry01GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY01_MGNT` reader - "]
pub type Keycam0Entry01MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY01_MGNT` writer - "]
pub type Keycam0Entry01MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY01_REPEATER_MODE` reader - "]
pub type Keycam0Entry01RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY01_REPEATER_MODE` writer - "]
pub type Keycam0Entry01RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY01_EXTTYPE` reader - "]
pub type Keycam0Entry01ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY01_EXTTYPE` writer - "]
pub type Keycam0Entry01ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY01_VALID` reader - "]
pub type Keycam0Entry01ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY01_VALID` writer - "]
pub type Keycam0Entry01ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY01_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry01Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY01_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry01Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry01_keyid_bit(&self) -> Keycam0Entry01KeyidBitR {
        Keycam0Entry01KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry01_type(&self) -> Keycam0Entry01TypeR {
        Keycam0Entry01TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry01_mickey_bit(&self) -> Keycam0Entry01MickeyBitR {
        Keycam0Entry01MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry01_grpkey_bit(&self) -> Keycam0Entry01GrpkeyBitR {
        Keycam0Entry01GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry01_mgnt(&self) -> Keycam0Entry01MgntR {
        Keycam0Entry01MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry01_repeater_mode(&self) -> Keycam0Entry01RepeaterModeR {
        Keycam0Entry01RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry01_exttype(&self) -> Keycam0Entry01ExttypeR {
        Keycam0Entry01ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry01_valid(&self) -> Keycam0Entry01ValidR {
        Keycam0Entry01ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry01_addr0(&self) -> Keycam0Entry01Addr0R {
        Keycam0Entry01Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry01_keyid_bit(
        &mut self,
    ) -> Keycam0Entry01KeyidBitW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry01_type(&mut self) -> Keycam0Entry01TypeW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry01_mickey_bit(
        &mut self,
    ) -> Keycam0Entry01MickeyBitW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry01_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry01GrpkeyBitW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry01_mgnt(&mut self) -> Keycam0Entry01MgntW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry01_repeater_mode(
        &mut self,
    ) -> Keycam0Entry01RepeaterModeW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry01_exttype(&mut self) -> Keycam0Entry01ExttypeW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry01_valid(&mut self) -> Keycam0Entry01ValidW<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry01_addr0(&mut self) -> Keycam0Entry01Addr0W<'_, RegKeycam0Entry01Spec> {
        Keycam0Entry01Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry01Spec;
impl crate::RegisterSpec for RegKeycam0Entry01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry01::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry01Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry01::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY01 to value 0"]
impl crate::Resettable for RegKeycam0Entry01Spec {}
