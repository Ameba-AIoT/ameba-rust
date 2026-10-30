#[doc = "Register `REG_KEYCAM0_ENTRY27` reader"]
pub type R = crate::R<RegKeycam0Entry27Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY27` writer"]
pub type W = crate::W<RegKeycam0Entry27Spec>;
#[doc = "Field `KEYCAM0_ENTRY27_KEYID_BIT` reader - "]
pub type Keycam0Entry27KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY27_KEYID_BIT` writer - "]
pub type Keycam0Entry27KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY27_TYPE` reader - "]
pub type Keycam0Entry27TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY27_TYPE` writer - "]
pub type Keycam0Entry27TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY27_MICKEY_BIT` reader - "]
pub type Keycam0Entry27MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY27_MICKEY_BIT` writer - "]
pub type Keycam0Entry27MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY27_GRPKEY_BIT` reader - "]
pub type Keycam0Entry27GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY27_GRPKEY_BIT` writer - "]
pub type Keycam0Entry27GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY27_MGNT` reader - "]
pub type Keycam0Entry27MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY27_MGNT` writer - "]
pub type Keycam0Entry27MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY27_REPEATER_MODE` reader - "]
pub type Keycam0Entry27RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY27_REPEATER_MODE` writer - "]
pub type Keycam0Entry27RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY27_EXTTYPE` reader - "]
pub type Keycam0Entry27ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY27_EXTTYPE` writer - "]
pub type Keycam0Entry27ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY27_VALID` reader - "]
pub type Keycam0Entry27ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY27_VALID` writer - "]
pub type Keycam0Entry27ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY27_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry27Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY27_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry27Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry27_keyid_bit(&self) -> Keycam0Entry27KeyidBitR {
        Keycam0Entry27KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry27_type(&self) -> Keycam0Entry27TypeR {
        Keycam0Entry27TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry27_mickey_bit(&self) -> Keycam0Entry27MickeyBitR {
        Keycam0Entry27MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry27_grpkey_bit(&self) -> Keycam0Entry27GrpkeyBitR {
        Keycam0Entry27GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry27_mgnt(&self) -> Keycam0Entry27MgntR {
        Keycam0Entry27MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry27_repeater_mode(&self) -> Keycam0Entry27RepeaterModeR {
        Keycam0Entry27RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry27_exttype(&self) -> Keycam0Entry27ExttypeR {
        Keycam0Entry27ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry27_valid(&self) -> Keycam0Entry27ValidR {
        Keycam0Entry27ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry27_addr0(&self) -> Keycam0Entry27Addr0R {
        Keycam0Entry27Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry27_keyid_bit(
        &mut self,
    ) -> Keycam0Entry27KeyidBitW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry27_type(&mut self) -> Keycam0Entry27TypeW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry27_mickey_bit(
        &mut self,
    ) -> Keycam0Entry27MickeyBitW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry27_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry27GrpkeyBitW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry27_mgnt(&mut self) -> Keycam0Entry27MgntW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry27_repeater_mode(
        &mut self,
    ) -> Keycam0Entry27RepeaterModeW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry27_exttype(&mut self) -> Keycam0Entry27ExttypeW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry27_valid(&mut self) -> Keycam0Entry27ValidW<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry27_addr0(&mut self) -> Keycam0Entry27Addr0W<'_, RegKeycam0Entry27Spec> {
        Keycam0Entry27Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry27Spec;
impl crate::RegisterSpec for RegKeycam0Entry27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry27::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry27Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry27::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY27 to value 0"]
impl crate::Resettable for RegKeycam0Entry27Spec {}
