#[doc = "Register `REG_KEYCAM0_ENTRY25` reader"]
pub type R = crate::R<RegKeycam0Entry25Spec>;
#[doc = "Register `REG_KEYCAM0_ENTRY25` writer"]
pub type W = crate::W<RegKeycam0Entry25Spec>;
#[doc = "Field `KEYCAM0_ENTRY25_KEYID_BIT` reader - "]
pub type Keycam0Entry25KeyidBitR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY25_KEYID_BIT` writer - "]
pub type Keycam0Entry25KeyidBitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KEYCAM0_ENTRY25_TYPE` reader - "]
pub type Keycam0Entry25TypeR = crate::FieldReader;
#[doc = "Field `KEYCAM0_ENTRY25_TYPE` writer - "]
pub type Keycam0Entry25TypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEYCAM0_ENTRY25_MICKEY_BIT` reader - "]
pub type Keycam0Entry25MickeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY25_MICKEY_BIT` writer - "]
pub type Keycam0Entry25MickeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY25_GRPKEY_BIT` reader - "]
pub type Keycam0Entry25GrpkeyBitR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY25_GRPKEY_BIT` writer - "]
pub type Keycam0Entry25GrpkeyBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY25_MGNT` reader - "]
pub type Keycam0Entry25MgntR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY25_MGNT` writer - "]
pub type Keycam0Entry25MgntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY25_REPEATER_MODE` reader - "]
pub type Keycam0Entry25RepeaterModeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY25_REPEATER_MODE` writer - "]
pub type Keycam0Entry25RepeaterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY25_EXTTYPE` reader - "]
pub type Keycam0Entry25ExttypeR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY25_EXTTYPE` writer - "]
pub type Keycam0Entry25ExttypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY25_VALID` reader - "]
pub type Keycam0Entry25ValidR = crate::BitReader;
#[doc = "Field `KEYCAM0_ENTRY25_VALID` writer - "]
pub type Keycam0Entry25ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYCAM0_ENTRY25_ADDR0` reader - address\\[15:0\\]"]
pub type Keycam0Entry25Addr0R = crate::FieldReader<u16>;
#[doc = "Field `KEYCAM0_ENTRY25_ADDR0` writer - address\\[15:0\\]"]
pub type Keycam0Entry25Addr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry25_keyid_bit(&self) -> Keycam0Entry25KeyidBitR {
        Keycam0Entry25KeyidBitR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry25_type(&self) -> Keycam0Entry25TypeR {
        Keycam0Entry25TypeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry25_mickey_bit(&self) -> Keycam0Entry25MickeyBitR {
        Keycam0Entry25MickeyBitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry25_grpkey_bit(&self) -> Keycam0Entry25GrpkeyBitR {
        Keycam0Entry25GrpkeyBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry25_mgnt(&self) -> Keycam0Entry25MgntR {
        Keycam0Entry25MgntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry25_repeater_mode(&self) -> Keycam0Entry25RepeaterModeR {
        Keycam0Entry25RepeaterModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry25_exttype(&self) -> Keycam0Entry25ExttypeR {
        Keycam0Entry25ExttypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry25_valid(&self) -> Keycam0Entry25ValidR {
        Keycam0Entry25ValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry25_addr0(&self) -> Keycam0Entry25Addr0R {
        Keycam0Entry25Addr0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn keycam0_entry25_keyid_bit(
        &mut self,
    ) -> Keycam0Entry25KeyidBitW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25KeyidBitW::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn keycam0_entry25_type(&mut self) -> Keycam0Entry25TypeW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25TypeW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn keycam0_entry25_mickey_bit(
        &mut self,
    ) -> Keycam0Entry25MickeyBitW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25MickeyBitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn keycam0_entry25_grpkey_bit(
        &mut self,
    ) -> Keycam0Entry25GrpkeyBitW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25GrpkeyBitW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn keycam0_entry25_mgnt(&mut self) -> Keycam0Entry25MgntW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25MgntW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn keycam0_entry25_repeater_mode(
        &mut self,
    ) -> Keycam0Entry25RepeaterModeW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25RepeaterModeW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn keycam0_entry25_exttype(&mut self) -> Keycam0Entry25ExttypeW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25ExttypeW::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn keycam0_entry25_valid(&mut self) -> Keycam0Entry25ValidW<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25ValidW::new(self, 15)
    }
    #[doc = "Bits 16:31 - address\\[15:0\\]"]
    #[inline(always)]
    pub fn keycam0_entry25_addr0(&mut self) -> Keycam0Entry25Addr0W<'_, RegKeycam0Entry25Spec> {
        Keycam0Entry25Addr0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_keycam0_entry25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_keycam0_entry25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegKeycam0Entry25Spec;
impl crate::RegisterSpec for RegKeycam0Entry25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_keycam0_entry25::R`](R) reader structure"]
impl crate::Readable for RegKeycam0Entry25Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_keycam0_entry25::W`](W) writer structure"]
impl crate::Writable for RegKeycam0Entry25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_KEYCAM0_ENTRY25 to value 0"]
impl crate::Resettable for RegKeycam0Entry25Spec {}
