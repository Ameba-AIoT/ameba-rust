#[doc = "Register `REG_RSV_CTRL` reader"]
pub type R = crate::R<RegRsvCtrlSpec>;
#[doc = "Register `REG_RSV_CTRL` writer"]
pub type W = crate::W<RegRsvCtrlSpec>;
#[doc = "Field `BIT_WLOCK_ALL` reader - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x00FFh)"]
pub type BitWlockAllR = crate::BitReader;
#[doc = "Field `BIT_WLOCK_ALL` writer - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x00FFh)"]
pub type BitWlockAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLOCK_00` reader - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x0003h)"]
pub type BitWlock00R = crate::BitReader;
#[doc = "Field `BIT_WLOCK_00` writer - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x0003h)"]
pub type BitWlock00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLOCK_04` reader - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0004h~0x0007h)"]
pub type BitWlock04R = crate::BitReader;
#[doc = "Field `BIT_WLOCK_04` writer - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0004h~0x0007h)"]
pub type BitWlock04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLOCK_08` reader - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0008h~0x000Bh)"]
pub type BitWlock08R = crate::BitReader;
#[doc = "Field `BIT_WLOCK_08` writer - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0008h~0x000Bh)"]
pub type BitWlock08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLOCK_40` reader - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0040h~0x0043h)"]
pub type BitWlock40R = crate::BitReader;
#[doc = "Field `BIT_WLOCK_40` writer - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0040h~0x0043h)"]
pub type BitWlock40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WLOCK_1C_B6` reader - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[6\\])"]
pub type BitWlock1cB6R = crate::BitReader;
#[doc = "Field `BIT_WLOCK_1C_B6` writer - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[6\\])"]
pub type BitWlock1cB6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LOCK_REGALL_EN` reader - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[0\\])"]
pub type BitLockRegallEnR = crate::BitReader;
#[doc = "Field `BIT_LOCK_REGALL_EN` writer - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[0\\])"]
pub type BitLockRegallEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_HREG_DBG` reader - HREG Access Debug information"]
pub type BitHregDbgR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x00FFh)"]
    #[inline(always)]
    pub fn bit_wlock_all(&self) -> BitWlockAllR {
        BitWlockAllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x0003h)"]
    #[inline(always)]
    pub fn bit_wlock_00(&self) -> BitWlock00R {
        BitWlock00R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0004h~0x0007h)"]
    #[inline(always)]
    pub fn bit_wlock_04(&self) -> BitWlock04R {
        BitWlock04R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0008h~0x000Bh)"]
    #[inline(always)]
    pub fn bit_wlock_08(&self) -> BitWlock08R {
        BitWlock08R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0040h~0x0043h)"]
    #[inline(always)]
    pub fn bit_wlock_40(&self) -> BitWlock40R {
        BitWlock40R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[6\\])"]
    #[inline(always)]
    pub fn bit_wlock_1c_b6(&self) -> BitWlock1cB6R {
        BitWlock1cB6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[0\\])"]
    #[inline(always)]
    pub fn bit_lock_regall_en(&self) -> BitLockRegallEnR {
        BitLockRegallEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:23 - HREG Access Debug information"]
    #[inline(always)]
    pub fn bit_hreg_dbg(&self) -> BitHregDbgR {
        BitHregDbgR::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x00FFh)"]
    #[inline(always)]
    pub fn bit_wlock_all(&mut self) -> BitWlockAllW<'_, RegRsvCtrlSpec> {
        BitWlockAllW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0000h~0x0003h)"]
    #[inline(always)]
    pub fn bit_wlock_00(&mut self) -> BitWlock00W<'_, RegRsvCtrlSpec> {
        BitWlock00W::new(self, 1)
    }
    #[doc = "Bit 2 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0004h~0x0007h)"]
    #[inline(always)]
    pub fn bit_wlock_04(&mut self) -> BitWlock04W<'_, RegRsvCtrlSpec> {
        BitWlock04W::new(self, 2)
    }
    #[doc = "Bit 3 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0008h~0x000Bh)"]
    #[inline(always)]
    pub fn bit_wlock_08(&mut self) -> BitWlock08W<'_, RegRsvCtrlSpec> {
        BitWlock08W::new(self, 3)
    }
    #[doc = "Bit 4 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x0040h~0x0043h)"]
    #[inline(always)]
    pub fn bit_wlock_40(&mut self) -> BitWlock40W<'_, RegRsvCtrlSpec> {
        BitWlock40W::new(self, 4)
    }
    #[doc = "Bit 5 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[6\\])"]
    #[inline(always)]
    pub fn bit_wlock_1c_b6(&mut self) -> BitWlock1cB6W<'_, RegRsvCtrlSpec> {
        BitWlock1cB6W::new(self, 5)
    }
    #[doc = "Bit 7 - 1:Register Write Locked, 0: Register Accessible by SW/FW (offset:0x001C\\[0\\])"]
    #[inline(always)]
    pub fn bit_lock_regall_en(&mut self) -> BitLockRegallEnW<'_, RegRsvCtrlSpec> {
        BitLockRegallEnW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_rsv_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_rsv_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRsvCtrlSpec;
impl crate::RegisterSpec for RegRsvCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rsv_ctrl::R`](R) reader structure"]
impl crate::Readable for RegRsvCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_rsv_ctrl::W`](W) writer structure"]
impl crate::Writable for RegRsvCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_RSV_CTRL to value 0"]
impl crate::Resettable for RegRsvCtrlSpec {}
