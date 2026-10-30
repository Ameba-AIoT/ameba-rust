#[doc = "Register `REG_BB_ACCESS_CTRL` reader"]
pub type R = crate::R<RegBbAccessCtrlSpec>;
#[doc = "Register `REG_BB_ACCESS_CTRL` writer"]
pub type W = crate::W<RegBbAccessCtrlSpec>;
#[doc = "Field `BIT_BB_ERRACC` reader - Duplicate Access when previous cycle pending (write one clear)"]
pub type BitBbErraccR = crate::BitReader;
#[doc = "Field `BIT_BB_ERRACC` writer - Duplicate Access when previous cycle pending (write one clear)"]
pub type BitBbErraccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - RSVD"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - RSVD"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BB_ADDR_V1` reader - Baseband Access Write/Read Address (in DW unit)"]
pub type BitBbAddrV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BB_ADDR_V1` writer - Baseband Access Write/Read Address (in DW unit)"]
pub type BitBbAddrV1W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `BIT_BB_WRITE_EN_V1` reader - Byte Write Enable"]
pub type BitBbWriteEnV1R = crate::FieldReader;
#[doc = "Field `BIT_BB_WRITE_EN_V1` writer - Byte Write Enable"]
pub type BitBbWriteEnV1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BIT_BB_WRITE_READ` reader - 00 : No Active Baseband cycle 01 : Baseband Write Access (auto return to 00 when access finished) 1X : Baseband Read Access (auto return to 00 when access finished)"]
pub type BitBbWriteReadR = crate::FieldReader;
#[doc = "Field `BIT_BB_WRITE_READ` writer - 00 : No Active Baseband cycle 01 : Baseband Write Access (auto return to 00 when access finished) 1X : Baseband Read Access (auto return to 00 when access finished)"]
pub type BitBbWriteReadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Duplicate Access when previous cycle pending (write one clear)"]
    #[inline(always)]
    pub fn bit_bb_erracc(&self) -> BitBbErraccR {
        BitBbErraccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - Baseband Access Write/Read Address (in DW unit)"]
    #[inline(always)]
    pub fn bit_bb_addr_v1(&self) -> BitBbAddrV1R {
        BitBbAddrV1R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:19 - Byte Write Enable"]
    #[inline(always)]
    pub fn bit_bb_write_en_v1(&self) -> BitBbWriteEnV1R {
        BitBbWriteEnV1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:29 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - 00 : No Active Baseband cycle 01 : Baseband Write Access (auto return to 00 when access finished) 1X : Baseband Read Access (auto return to 00 when access finished)"]
    #[inline(always)]
    pub fn bit_bb_write_read(&self) -> BitBbWriteReadR {
        BitBbWriteReadR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Duplicate Access when previous cycle pending (write one clear)"]
    #[inline(always)]
    pub fn bit_bb_erracc(&mut self) -> BitBbErraccW<'_, RegBbAccessCtrlSpec> {
        BitBbErraccW::new(self, 0)
    }
    #[doc = "Bit 1 - RSVD"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> Rsvd2W<'_, RegBbAccessCtrlSpec> {
        Rsvd2W::new(self, 1)
    }
    #[doc = "Bits 2:15 - Baseband Access Write/Read Address (in DW unit)"]
    #[inline(always)]
    pub fn bit_bb_addr_v1(&mut self) -> BitBbAddrV1W<'_, RegBbAccessCtrlSpec> {
        BitBbAddrV1W::new(self, 2)
    }
    #[doc = "Bits 16:19 - Byte Write Enable"]
    #[inline(always)]
    pub fn bit_bb_write_en_v1(&mut self) -> BitBbWriteEnV1W<'_, RegBbAccessCtrlSpec> {
        BitBbWriteEnV1W::new(self, 16)
    }
    #[doc = "Bits 20:29 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegBbAccessCtrlSpec> {
        RsvdW::new(self, 20)
    }
    #[doc = "Bits 30:31 - 00 : No Active Baseband cycle 01 : Baseband Write Access (auto return to 00 when access finished) 1X : Baseband Read Access (auto return to 00 when access finished)"]
    #[inline(always)]
    pub fn bit_bb_write_read(&mut self) -> BitBbWriteReadW<'_, RegBbAccessCtrlSpec> {
        BitBbWriteReadW::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bb_access_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bb_access_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBbAccessCtrlSpec;
impl crate::RegisterSpec for RegBbAccessCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bb_access_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBbAccessCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bb_access_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBbAccessCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BB_ACCESS_CTRL to value 0"]
impl crate::Resettable for RegBbAccessCtrlSpec {}
