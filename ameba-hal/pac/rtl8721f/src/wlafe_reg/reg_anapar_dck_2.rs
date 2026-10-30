#[doc = "Register `REG_ANAPAR_DCK_2` reader"]
pub type R = crate::R<RegAnaparDck2Spec>;
#[doc = "Register `REG_ANAPAR_DCK_2` writer"]
pub type W = crate::W<RegAnaparDck2Spec>;
#[doc = "Field `DCK_DOS_MANU_Q_CR` reader - DOS_Q\\[5:0\\] by manual"]
pub type DckDosManuQCrR = crate::FieldReader;
#[doc = "Field `DCK_DOS_MANU_Q_CR` writer - DOS_Q\\[5:0\\] by manual"]
pub type DckDosManuQCrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_KOS_EN_MANU_CR` reader - KOS_EN\\[5:0\\] by manual"]
pub type DckKosEnManuCrR = crate::FieldReader;
#[doc = "Field `DCK_KOS_EN_MANU_CR` writer - KOS_EN\\[5:0\\] by manual"]
pub type DckKosEnManuCrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_OFS_POLAR_CR` reader - id\\[8:0\\] & qd\\[8:0\\] offset +/- => 0(+) / 1(-)"]
pub type DckOfsPolarCrR = crate::BitReader;
#[doc = "Field `DCK_OFS_POLAR_CR` writer - id\\[8:0\\] & qd\\[8:0\\] offset +/- => 0(+) / 1(-)"]
pub type DckOfsPolarCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_VOS_ADDR_CR` reader - Fetch the k-th offset average value to monitor"]
pub type DckVosAddrCrR = crate::FieldReader;
#[doc = "Field `DCK_VOS_ADDR_CR` writer - Fetch the k-th offset average value to monitor"]
pub type DckVosAddrCrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCK_KOS_EN_MANU_I_CR` reader - KOS_EN_I\\[5:0\\] by manual"]
pub type DckKosEnManuICrR = crate::FieldReader;
#[doc = "Field `DCK_KOS_EN_MANU_I_CR` writer - KOS_EN_I\\[5:0\\] by manual"]
pub type DckKosEnManuICrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_KOS_EN_MANU_Q_CR` reader - KOS_EN_Q\\[5:0\\] by manual"]
pub type DckKosEnManuQCrR = crate::FieldReader;
#[doc = "Field `DCK_KOS_EN_MANU_Q_CR` writer - KOS_EN_Q\\[5:0\\] by manual"]
pub type DckKosEnManuQCrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_FIFO_CLEAR_CR` reader - Clear all FIFO"]
pub type DckFifoClearCrR = crate::BitReader;
#[doc = "Field `DCK_FIFO_CLEAR_CR` writer - Clear all FIFO"]
pub type DckFifoClearCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_BT_DOS_FORCE_I_CR` reader - Manual write DOS data at BT mode"]
pub type DckBtDosForceICrR = crate::BitReader;
#[doc = "Field `DCK_BT_DOS_FORCE_I_CR` writer - Manual write DOS data at BT mode"]
pub type DckBtDosForceICrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_BT_DOS_FORCE_Q_CR` reader - Manual write DOS data at BT mode"]
pub type DckBtDosForceQCrR = crate::BitReader;
#[doc = "Field `DCK_BT_DOS_FORCE_Q_CR` writer - Manual write DOS data at BT mode"]
pub type DckBtDosForceQCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCK_BT_KOS_EN_FORCE_I_CR` reader - Manual write KOSEN at BT mode"]
pub type DckBtKosEnForceICrR = crate::BitReader;
#[doc = "Field `DCK_BT_KOS_EN_FORCE_I_CR` writer - Manual write KOSEN at BT mode"]
pub type DckBtKosEnForceICrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - DOS_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_q_cr(&self) -> DckDosManuQCrR {
        DckDosManuQCrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - KOS_EN\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_cr(&self) -> DckKosEnManuCrR {
        DckKosEnManuCrR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - id\\[8:0\\] & qd\\[8:0\\] offset +/- => 0(+) / 1(-)"]
    #[inline(always)]
    pub fn dck_ofs_polar_cr(&self) -> DckOfsPolarCrR {
        DckOfsPolarCrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Fetch the k-th offset average value to monitor"]
    #[inline(always)]
    pub fn dck_vos_addr_cr(&self) -> DckVosAddrCrR {
        DckVosAddrCrR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - KOS_EN_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_i_cr(&self) -> DckKosEnManuICrR {
        DckKosEnManuICrR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - KOS_EN_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_q_cr(&self) -> DckKosEnManuQCrR {
        DckKosEnManuQCrR::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - Clear all FIFO"]
    #[inline(always)]
    pub fn dck_fifo_clear_cr(&self) -> DckFifoClearCrR {
        DckFifoClearCrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Manual write DOS data at BT mode"]
    #[inline(always)]
    pub fn dck_bt_dos_force_i_cr(&self) -> DckBtDosForceICrR {
        DckBtDosForceICrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Manual write DOS data at BT mode"]
    #[inline(always)]
    pub fn dck_bt_dos_force_q_cr(&self) -> DckBtDosForceQCrR {
        DckBtDosForceQCrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Manual write KOSEN at BT mode"]
    #[inline(always)]
    pub fn dck_bt_kos_en_force_i_cr(&self) -> DckBtKosEnForceICrR {
        DckBtKosEnForceICrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DOS_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_q_cr(&mut self) -> DckDosManuQCrW<'_, RegAnaparDck2Spec> {
        DckDosManuQCrW::new(self, 0)
    }
    #[doc = "Bits 6:11 - KOS_EN\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_cr(&mut self) -> DckKosEnManuCrW<'_, RegAnaparDck2Spec> {
        DckKosEnManuCrW::new(self, 6)
    }
    #[doc = "Bit 12 - id\\[8:0\\] & qd\\[8:0\\] offset +/- => 0(+) / 1(-)"]
    #[inline(always)]
    pub fn dck_ofs_polar_cr(&mut self) -> DckOfsPolarCrW<'_, RegAnaparDck2Spec> {
        DckOfsPolarCrW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Fetch the k-th offset average value to monitor"]
    #[inline(always)]
    pub fn dck_vos_addr_cr(&mut self) -> DckVosAddrCrW<'_, RegAnaparDck2Spec> {
        DckVosAddrCrW::new(self, 13)
    }
    #[doc = "Bits 16:21 - KOS_EN_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_i_cr(&mut self) -> DckKosEnManuICrW<'_, RegAnaparDck2Spec> {
        DckKosEnManuICrW::new(self, 16)
    }
    #[doc = "Bits 22:27 - KOS_EN_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_q_cr(&mut self) -> DckKosEnManuQCrW<'_, RegAnaparDck2Spec> {
        DckKosEnManuQCrW::new(self, 22)
    }
    #[doc = "Bit 28 - Clear all FIFO"]
    #[inline(always)]
    pub fn dck_fifo_clear_cr(&mut self) -> DckFifoClearCrW<'_, RegAnaparDck2Spec> {
        DckFifoClearCrW::new(self, 28)
    }
    #[doc = "Bit 29 - Manual write DOS data at BT mode"]
    #[inline(always)]
    pub fn dck_bt_dos_force_i_cr(&mut self) -> DckBtDosForceICrW<'_, RegAnaparDck2Spec> {
        DckBtDosForceICrW::new(self, 29)
    }
    #[doc = "Bit 30 - Manual write DOS data at BT mode"]
    #[inline(always)]
    pub fn dck_bt_dos_force_q_cr(&mut self) -> DckBtDosForceQCrW<'_, RegAnaparDck2Spec> {
        DckBtDosForceQCrW::new(self, 30)
    }
    #[doc = "Bit 31 - Manual write KOSEN at BT mode"]
    #[inline(always)]
    pub fn dck_bt_kos_en_force_i_cr(&mut self) -> DckBtKosEnForceICrW<'_, RegAnaparDck2Spec> {
        DckBtKosEnForceICrW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_dck_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_dck_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparDck2Spec;
impl crate::RegisterSpec for RegAnaparDck2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_dck_2::R`](R) reader structure"]
impl crate::Readable for RegAnaparDck2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_dck_2::W`](W) writer structure"]
impl crate::Writable for RegAnaparDck2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_DCK_2 to value 0xc000"]
impl crate::Resettable for RegAnaparDck2Spec {
    const RESET_VALUE: u32 = 0xc000;
}
