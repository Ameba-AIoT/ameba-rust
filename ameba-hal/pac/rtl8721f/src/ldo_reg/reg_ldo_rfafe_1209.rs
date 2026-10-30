#[doc = "Register `REG_LDO_RFAFE_1209` reader"]
pub type R = crate::R<RegLdoRfafe1209Spec>;
#[doc = "Register `REG_LDO_RFAFE_1209` writer"]
pub type W = crate::W<RegLdoRfafe1209Spec>;
#[doc = "Field `en_pc_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
pub type EnPcL1209R = crate::BitReader;
#[doc = "Field `en_pc_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
pub type EnPcL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd_regu_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode // 1: power-down Mode (default)"]
pub type PdReguL1209R = crate::BitReader;
#[doc = "Field `pd_regu_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode // 1: power-down Mode (default)"]
pub type PdReguL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `voadj_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO VOSEL // 0000, 0001, 0010, 0011: 0.708, 0.739, 0.772, 0.802 // 0100, 0101, 0110, 0111(default): 0.827, 0.853, 0.877, 0.9 // 1000, 1001, 1010, 1011: 0.924, 0.949, 0.976, 1 // 1100, 1101, 1110, 1111: 1.025, 1.05, 1.075, 1.01"]
pub type VoadjL1209R = crate::FieldReader;
#[doc = "Field `voadj_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO VOSEL // 0000, 0001, 0010, 0011: 0.708, 0.739, 0.772, 0.802 // 0100, 0101, 0110, 0111(default): 0.827, 0.853, 0.877, 0.9 // 1000, 1001, 1010, 1011: 0.924, 0.949, 0.976, 1 // 1100, 1101, 1110, 1111: 1.025, 1.05, 1.075, 1.01"]
pub type VoadjL1209W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ssmode_L_1209` reader - //0:disable ssmode function //1:enable: ssmode function (smaller power mos size)"]
pub type SsmodeL1209R = crate::BitReader;
#[doc = "Field `ssmode_L_1209` writer - //0:disable ssmode function //1:enable: ssmode function (smaller power mos size)"]
pub type SsmodeL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ov_lvl_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV vref level // 0: 1.05V // 1 : 1V"]
pub type RegOvLvlL1209R = crate::BitReader;
#[doc = "Field `reg_ov_lvl_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV vref level // 0: 1.05V // 1 : 1V"]
pub type RegOvLvlL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sleep_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO mode control // 0: normal mode // 1 : sleep mode"]
pub type RegSleepL1209R = crate::BitReader;
#[doc = "Field `reg_sleep_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO mode control // 0: normal mode // 1 : sleep mode"]
pub type RegSleepL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_diode_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 1: enable clamp // 0: disable clamp"]
pub type EnDiode1209R = crate::BitReader;
#[doc = "Field `en_diode_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 1: enable clamp // 0: disable clamp"]
pub type EnDiode1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_light_dmyload_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmy load disable // 1:9k Ω (0.1mA)"]
pub type RegLightDmyloadL1209R = crate::BitReader;
#[doc = "Field `reg_light_dmyload_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmy load disable // 1:9k Ω (0.1mA)"]
pub type RegLightDmyloadL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dmyload_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmy load disable // 1: dmy load enable"]
pub type RegDmyloadL1209R = crate::BitReader;
#[doc = "Field `reg_ov_detect_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV detect function control // 0: disable // 1 : enable"]
pub type RegOvDetectL1209R = crate::BitReader;
#[doc = "Field `reg_ov_detect_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV detect function control // 0: disable // 1 : enable"]
pub type RegOvDetectL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pre_pc_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // 0: LDO-Mode (default) // 1:Pre- PWRCUT-Mode"]
pub type RegPrePcL1209R = crate::BitReader;
#[doc = "Field `reg_pre_pc_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // 0: LDO-Mode (default) // 1:Pre- PWRCUT-Mode"]
pub type RegPrePcL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dmyload_x2_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_2X(10mA)"]
pub type RegDmyloadX2L1209R = crate::BitReader;
#[doc = "Field `reg_dmyload_x2_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_2X(10mA)"]
pub type RegDmyloadX2L1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dmyload_x3_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_3X(15mA)"]
pub type RegDmyloadX3L1209R = crate::BitReader;
#[doc = "Field `reg_dmyload_x3_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_3X(15mA)"]
pub type RegDmyloadX3L1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_fb_sel_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // vo feedback select // 0: vo(digital pad) (default) // 1: vo_18(powermos pad)"]
pub type RegFbSelL1209R = crate::BitReader;
#[doc = "Field `reg_fb_sel_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // vo feedback select // 0: vo(digital pad) (default) // 1: vo_18(powermos pad)"]
pub type RegFbSelL1209W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_vref_sel_L_1209` reader - // \\[REGU\\]\\[LDO\\] REG_CTRL // vref sel // 00: 0.6V // 01: 0.57V // 10: 0.56V // 11: 0.54V"]
pub type RegVrefSelL1209R = crate::FieldReader;
#[doc = "Field `reg_vref_sel_L_1209` writer - // \\[REGU\\]\\[LDO\\] REG_CTRL // vref sel // 00: 0.6V // 01: 0.57V // 10: 0.56V // 11: 0.54V"]
pub type RegVrefSelL1209W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dmy_7` reader - Dummy"]
pub type Dmy7R = crate::FieldReader;
#[doc = "Field `dmy_7` writer - Dummy"]
pub type Dmy7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
    #[inline(always)]
    pub fn en_pc_l_1209(&self) -> EnPcL1209R {
        EnPcL1209R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode // 1: power-down Mode (default)"]
    #[inline(always)]
    pub fn pd_regu_l_1209(&self) -> PdReguL1209R {
        PdReguL1209R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO VOSEL // 0000, 0001, 0010, 0011: 0.708, 0.739, 0.772, 0.802 // 0100, 0101, 0110, 0111(default): 0.827, 0.853, 0.877, 0.9 // 1000, 1001, 1010, 1011: 0.924, 0.949, 0.976, 1 // 1100, 1101, 1110, 1111: 1.025, 1.05, 1.075, 1.01"]
    #[inline(always)]
    pub fn voadj_l_1209(&self) -> VoadjL1209R {
        VoadjL1209R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - //0:disable ssmode function //1:enable: ssmode function (smaller power mos size)"]
    #[inline(always)]
    pub fn ssmode_l_1209(&self) -> SsmodeL1209R {
        SsmodeL1209R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV vref level // 0: 1.05V // 1 : 1V"]
    #[inline(always)]
    pub fn reg_ov_lvl_l_1209(&self) -> RegOvLvlL1209R {
        RegOvLvlL1209R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO mode control // 0: normal mode // 1 : sleep mode"]
    #[inline(always)]
    pub fn reg_sleep_l_1209(&self) -> RegSleepL1209R {
        RegSleepL1209R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 1: enable clamp // 0: disable clamp"]
    #[inline(always)]
    pub fn en_diode_1209(&self) -> EnDiode1209R {
        EnDiode1209R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmy load disable // 1:9k Ω (0.1mA)"]
    #[inline(always)]
    pub fn reg_light_dmyload_l_1209(&self) -> RegLightDmyloadL1209R {
        RegLightDmyloadL1209R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmy load disable // 1: dmy load enable"]
    #[inline(always)]
    pub fn reg_dmyload_l_1209(&self) -> RegDmyloadL1209R {
        RegDmyloadL1209R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV detect function control // 0: disable // 1 : enable"]
    #[inline(always)]
    pub fn reg_ov_detect_l_1209(&self) -> RegOvDetectL1209R {
        RegOvDetectL1209R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - // \\[REGU\\]\\[LDO\\] REG_CTRL // 0: LDO-Mode (default) // 1:Pre- PWRCUT-Mode"]
    #[inline(always)]
    pub fn reg_pre_pc_l_1209(&self) -> RegPrePcL1209R {
        RegPrePcL1209R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_2X(10mA)"]
    #[inline(always)]
    pub fn reg_dmyload_x2_l_1209(&self) -> RegDmyloadX2L1209R {
        RegDmyloadX2L1209R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_3X(15mA)"]
    #[inline(always)]
    pub fn reg_dmyload_x3_l_1209(&self) -> RegDmyloadX3L1209R {
        RegDmyloadX3L1209R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - // \\[REGU\\]\\[LDO\\] REG_CTRL // vo feedback select // 0: vo(digital pad) (default) // 1: vo_18(powermos pad)"]
    #[inline(always)]
    pub fn reg_fb_sel_l_1209(&self) -> RegFbSelL1209R {
        RegFbSelL1209R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - // \\[REGU\\]\\[LDO\\] REG_CTRL // vref sel // 00: 0.6V // 01: 0.57V // 10: 0.56V // 11: 0.54V"]
    #[inline(always)]
    pub fn reg_vref_sel_l_1209(&self) -> RegVrefSelL1209R {
        RegVrefSelL1209R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Dummy"]
    #[inline(always)]
    pub fn dmy_7(&self) -> Dmy7R {
        Dmy7R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PWRCUT enable // 0: LDO-Mode (default) // 1: PWRCUT-Mode"]
    #[inline(always)]
    pub fn en_pc_l_1209(&mut self) -> EnPcL1209W<'_, RegLdoRfafe1209Spec> {
        EnPcL1209W::new(self, 0)
    }
    #[doc = "Bit 1 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PowerDown CTRL // 0: normal-Mode // 1: power-down Mode (default)"]
    #[inline(always)]
    pub fn pd_regu_l_1209(&mut self) -> PdReguL1209W<'_, RegLdoRfafe1209Spec> {
        PdReguL1209W::new(self, 1)
    }
    #[doc = "Bits 2:5 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO VOSEL // 0000, 0001, 0010, 0011: 0.708, 0.739, 0.772, 0.802 // 0100, 0101, 0110, 0111(default): 0.827, 0.853, 0.877, 0.9 // 1000, 1001, 1010, 1011: 0.924, 0.949, 0.976, 1 // 1100, 1101, 1110, 1111: 1.025, 1.05, 1.075, 1.01"]
    #[inline(always)]
    pub fn voadj_l_1209(&mut self) -> VoadjL1209W<'_, RegLdoRfafe1209Spec> {
        VoadjL1209W::new(self, 2)
    }
    #[doc = "Bit 6 - //0:disable ssmode function //1:enable: ssmode function (smaller power mos size)"]
    #[inline(always)]
    pub fn ssmode_l_1209(&mut self) -> SsmodeL1209W<'_, RegLdoRfafe1209Spec> {
        SsmodeL1209W::new(self, 6)
    }
    #[doc = "Bit 7 - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV vref level // 0: 1.05V // 1 : 1V"]
    #[inline(always)]
    pub fn reg_ov_lvl_l_1209(&mut self) -> RegOvLvlL1209W<'_, RegLdoRfafe1209Spec> {
        RegOvLvlL1209W::new(self, 7)
    }
    #[doc = "Bit 8 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO mode control // 0: normal mode // 1 : sleep mode"]
    #[inline(always)]
    pub fn reg_sleep_l_1209(&mut self) -> RegSleepL1209W<'_, RegLdoRfafe1209Spec> {
        RegSleepL1209W::new(self, 8)
    }
    #[doc = "Bit 9 - // \\[REGU\\]\\[LDO\\] REG_CTRL // PGATE diode clamp (this gating to disable by pd_regu_L) // 1: enable clamp // 0: disable clamp"]
    #[inline(always)]
    pub fn en_diode_1209(&mut self) -> EnDiode1209W<'_, RegLdoRfafe1209Spec> {
        EnDiode1209W::new(self, 9)
    }
    #[doc = "Bit 10 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmy load disable // 1:9k Ω (0.1mA)"]
    #[inline(always)]
    pub fn reg_light_dmyload_l_1209(&mut self) -> RegLightDmyloadL1209W<'_, RegLdoRfafe1209Spec> {
        RegLightDmyloadL1209W::new(self, 10)
    }
    #[doc = "Bit 12 - // \\[REGU\\]\\[LDO\\] REG_CTRL // OV detect function control // 0: disable // 1 : enable"]
    #[inline(always)]
    pub fn reg_ov_detect_l_1209(&mut self) -> RegOvDetectL1209W<'_, RegLdoRfafe1209Spec> {
        RegOvDetectL1209W::new(self, 12)
    }
    #[doc = "Bit 13 - // \\[REGU\\]\\[LDO\\] REG_CTRL // 0: LDO-Mode (default) // 1:Pre- PWRCUT-Mode"]
    #[inline(always)]
    pub fn reg_pre_pc_l_1209(&mut self) -> RegPrePcL1209W<'_, RegLdoRfafe1209Spec> {
        RegPrePcL1209W::new(self, 13)
    }
    #[doc = "Bit 14 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_2X(10mA)"]
    #[inline(always)]
    pub fn reg_dmyload_x2_l_1209(&mut self) -> RegDmyloadX2L1209W<'_, RegLdoRfafe1209Spec> {
        RegDmyloadX2L1209W::new(self, 14)
    }
    #[doc = "Bit 15 - // \\[REGU\\]\\[LDO\\] REG_CTRL // LDO dmy load ctrl // 0: dmyload_1X(5mA) // 1: dmyload_3X(15mA)"]
    #[inline(always)]
    pub fn reg_dmyload_x3_l_1209(&mut self) -> RegDmyloadX3L1209W<'_, RegLdoRfafe1209Spec> {
        RegDmyloadX3L1209W::new(self, 15)
    }
    #[doc = "Bit 16 - // \\[REGU\\]\\[LDO\\] REG_CTRL // vo feedback select // 0: vo(digital pad) (default) // 1: vo_18(powermos pad)"]
    #[inline(always)]
    pub fn reg_fb_sel_l_1209(&mut self) -> RegFbSelL1209W<'_, RegLdoRfafe1209Spec> {
        RegFbSelL1209W::new(self, 16)
    }
    #[doc = "Bits 17:18 - // \\[REGU\\]\\[LDO\\] REG_CTRL // vref sel // 00: 0.6V // 01: 0.57V // 10: 0.56V // 11: 0.54V"]
    #[inline(always)]
    pub fn reg_vref_sel_l_1209(&mut self) -> RegVrefSelL1209W<'_, RegLdoRfafe1209Spec> {
        RegVrefSelL1209W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Dummy"]
    #[inline(always)]
    pub fn dmy_7(&mut self) -> Dmy7W<'_, RegLdoRfafe1209Spec> {
        Dmy7W::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ldo_rfafe_1209::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ldo_rfafe_1209::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegLdoRfafe1209Spec;
impl crate::RegisterSpec for RegLdoRfafe1209Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ldo_rfafe_1209::R`](R) reader structure"]
impl crate::Readable for RegLdoRfafe1209Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_ldo_rfafe_1209::W`](W) writer structure"]
impl crate::Writable for RegLdoRfafe1209Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_LDO_RFAFE_1209 to value 0x4a5e"]
impl crate::Resettable for RegLdoRfafe1209Spec {
    const RESET_VALUE: u32 = 0x4a5e;
}
