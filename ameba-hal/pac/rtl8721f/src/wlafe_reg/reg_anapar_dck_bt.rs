#[doc = "Register `REG_ANAPAR_DCK_BT` reader"]
pub type R = crate::R<RegAnaparDckBtSpec>;
#[doc = "Register `REG_ANAPAR_DCK_BT` writer"]
pub type W = crate::W<RegAnaparDckBtSpec>;
#[doc = "Field `DCK_KOS_EN_MANU_I_BT` reader - KOS_EN_I\\[5:0\\] by manual"]
pub type DckKosEnManuIBtR = crate::FieldReader;
#[doc = "Field `DCK_KOS_EN_MANU_I_BT` writer - KOS_EN_I\\[5:0\\] by manual"]
pub type DckKosEnManuIBtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_KOS_EN_MANU_Q_BT` reader - KOS_EN_Q\\[5:0\\] by manual"]
pub type DckKosEnManuQBtR = crate::FieldReader;
#[doc = "Field `DCK_KOS_EN_MANU_Q_BT` writer - KOS_EN_Q\\[5:0\\] by manual"]
pub type DckKosEnManuQBtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_DOS_MANU_Q_BT` reader - DOS_Q\\[5:0\\] by manual"]
pub type DckDosManuQBtR = crate::FieldReader;
#[doc = "Field `DCK_DOS_MANU_Q_BT` writer - DOS_Q\\[5:0\\] by manual"]
pub type DckDosManuQBtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DCK_DOS_MANU_I_BT` reader - DOS_I\\[5:0\\] by manual"]
pub type DckDosManuIBtR = crate::FieldReader;
#[doc = "Field `DCK_DOS_MANU_I_BT` writer - DOS_I\\[5:0\\] by manual"]
pub type DckDosManuIBtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_GNT_BT_SEL` reader - 0:select BTG_GNT_BT_RF to select DCK data 1:select reg_manual_mode to select DCK data"]
pub type RegGntBtSelR = crate::BitReader;
#[doc = "Field `reg_GNT_BT_SEL` writer - 0:select BTG_GNT_BT_RF to select DCK data 1:select reg_manual_mode to select DCK data"]
pub type RegGntBtSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_manual_mode` reader - 0:select wifi DCK data 1:select bt DCK data"]
pub type RegManualModeR = crate::BitReader;
#[doc = "Field `reg_manual_mode` writer - 0:select wifi DCK data 1:select bt DCK data"]
pub type RegManualModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - KOS_EN_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_i_bt(&self) -> DckKosEnManuIBtR {
        DckKosEnManuIBtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - KOS_EN_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_q_bt(&self) -> DckKosEnManuQBtR {
        DckKosEnManuQBtR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - DOS_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_q_bt(&self) -> DckDosManuQBtR {
        DckDosManuQBtR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - DOS_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_i_bt(&self) -> DckDosManuIBtR {
        DckDosManuIBtR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - 0:select BTG_GNT_BT_RF to select DCK data 1:select reg_manual_mode to select DCK data"]
    #[inline(always)]
    pub fn reg_gnt_bt_sel(&self) -> RegGntBtSelR {
        RegGntBtSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0:select wifi DCK data 1:select bt DCK data"]
    #[inline(always)]
    pub fn reg_manual_mode(&self) -> RegManualModeR {
        RegManualModeR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - KOS_EN_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_i_bt(&mut self) -> DckKosEnManuIBtW<'_, RegAnaparDckBtSpec> {
        DckKosEnManuIBtW::new(self, 0)
    }
    #[doc = "Bits 6:11 - KOS_EN_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_kos_en_manu_q_bt(&mut self) -> DckKosEnManuQBtW<'_, RegAnaparDckBtSpec> {
        DckKosEnManuQBtW::new(self, 6)
    }
    #[doc = "Bits 12:17 - DOS_Q\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_q_bt(&mut self) -> DckDosManuQBtW<'_, RegAnaparDckBtSpec> {
        DckDosManuQBtW::new(self, 12)
    }
    #[doc = "Bits 18:23 - DOS_I\\[5:0\\] by manual"]
    #[inline(always)]
    pub fn dck_dos_manu_i_bt(&mut self) -> DckDosManuIBtW<'_, RegAnaparDckBtSpec> {
        DckDosManuIBtW::new(self, 18)
    }
    #[doc = "Bit 24 - 0:select BTG_GNT_BT_RF to select DCK data 1:select reg_manual_mode to select DCK data"]
    #[inline(always)]
    pub fn reg_gnt_bt_sel(&mut self) -> RegGntBtSelW<'_, RegAnaparDckBtSpec> {
        RegGntBtSelW::new(self, 24)
    }
    #[doc = "Bit 25 - 0:select wifi DCK data 1:select bt DCK data"]
    #[inline(always)]
    pub fn reg_manual_mode(&mut self) -> RegManualModeW<'_, RegAnaparDckBtSpec> {
        RegManualModeW::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_dck_bt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_dck_bt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparDckBtSpec;
impl crate::RegisterSpec for RegAnaparDckBtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_dck_bt::R`](R) reader structure"]
impl crate::Readable for RegAnaparDckBtSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_dck_bt::W`](W) writer structure"]
impl crate::Writable for RegAnaparDckBtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_DCK_BT to value 0"]
impl crate::Resettable for RegAnaparDckBtSpec {}
