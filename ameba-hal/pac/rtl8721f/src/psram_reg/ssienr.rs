#[doc = "Register `SSIENR` reader"]
pub type R = crate::R<SsienrSpec>;
#[doc = "Register `SSIENR` writer"]
pub type W = crate::W<SsienrSpec>;
#[doc = "Field `SPIC_EN` reader - Set to enable SPIC and start user mode transaction. SPIC will reset SPIC_EN to 0 after user mode transaction is finished. SPIC_EN can only be set when BOOT_FIN=1 (boot finished) and USER_MODE=1 (user mode). (User can read SPIC_EN to check whether current user mode transaction is finished.)"]
pub type SpicEnR = crate::BitReader;
#[doc = "Field `SPIC_EN` writer - Set to enable SPIC and start user mode transaction. SPIC will reset SPIC_EN to 0 after user mode transaction is finished. SPIC_EN can only be set when BOOT_FIN=1 (boot finished) and USER_MODE=1 (user mode). (User can read SPIC_EN to check whether current user mode transaction is finished.)"]
pub type SpicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCK_CMD` reader - Set to enable ATCK_CMD implementation. After setting SPIC would not accept any command until checking Flash is not busy or checking time out. (Use in User Mode, especially for Erase/Program to check status automatically.) If CTRLR2.SEQ_SET setting, this function needs to disable."]
pub type AtckCmdR = crate::BitReader;
#[doc = "Field `ATCK_CMD` writer - Set to enable ATCK_CMD implementation. After setting SPIC would not accept any command until checking Flash is not busy or checking time out. (Use in User Mode, especially for Erase/Program to check status automatically.) If CTRLR2.SEQ_SET setting, this function needs to disable."]
pub type AtckCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRQC` reader - Set to enable frequency change function. SPIC will chop current transaction and then set frqc_req = 1 after setting this bit. Frquency change is finished when frqc_req and frqc_ack are both high and then the choped transaction will return and continue. Write: 0: Not to activate FRQC 1: Activate FRQC Read: 0: FRQC is still in progress 1: FRQC is done Exists: TAG >= 1.0.4."]
pub type FrqcR = crate::BitReader;
#[doc = "Field `FRQC` writer - Set to enable frequency change function. SPIC will chop current transaction and then set frqc_req = 1 after setting this bit. Frquency change is finished when frqc_req and frqc_ack are both high and then the choped transaction will return and continue. Write: 0: Not to activate FRQC 1: Activate FRQC Read: 0: FRQC is still in progress 1: FRQC is done Exists: TAG >= 1.0.4."]
pub type FrqcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEX` reader - Set to trigger device enter sleep mode or deep power down mode. After setting PDEX_CTRL0, PDEX_CTRL1, PDEX_CTRL2 and TPR2, then set this bit to trigger PDEX flow. Write: 0 - Not to activate PDEX 1 - Activate PDEX Read: 0 - PDEX is still in progress 1 - PDEX is done Exists: TAG >= 2.0.5."]
pub type PdexR = crate::BitReader;
#[doc = "Field `PDEX` writer - Set to trigger device enter sleep mode or deep power down mode. After setting PDEX_CTRL0, PDEX_CTRL1, PDEX_CTRL2 and TPR2, then set this bit to trigger PDEX flow. Write: 0 - Not to activate PDEX 1 - Activate PDEX Read: 0 - PDEX is still in progress 1 - PDEX is done Exists: TAG >= 2.0.5."]
pub type PdexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GP_NUM` reader - Only used in frequency change function. Set to determine which group register user want to read or write. Base on different operation frequency, some control register setting will change. Therefore we build second group register on USER_LENGTH.USER_RD_DUMMY_LENGTH, AUTO_LENGTH.RD_DUMMY_LENGTH, AUTO_LENGTH.RDSR_DUMMY_LENGTH, AUTO_LENGTH.IN_PHYSICAL_CYC, TPR0.CS_TCEM, TPR0.CS_ACTIVE_HOLD, TPR0.CS_H_WR_DUM_LEN, TPR0.CS_H_RD_DUM_LEN, AUTO_LENGTH2.WR_DUMMY_LENGTH, TPR1.CR_TPWR, TPR1.CR_IDLE_WINDOW, TPR1.CS_ACTICE_SETUP. Exists: TAG >= 1.0.4."]
pub type GpNumR = crate::FieldReader;
#[doc = "Field `GP_NUM` writer - Only used in frequency change function. Set to determine which group register user want to read or write. Base on different operation frequency, some control register setting will change. Therefore we build second group register on USER_LENGTH.USER_RD_DUMMY_LENGTH, AUTO_LENGTH.RD_DUMMY_LENGTH, AUTO_LENGTH.RDSR_DUMMY_LENGTH, AUTO_LENGTH.IN_PHYSICAL_CYC, TPR0.CS_TCEM, TPR0.CS_ACTIVE_HOLD, TPR0.CS_H_WR_DUM_LEN, TPR0.CS_H_RD_DUM_LEN, AUTO_LENGTH2.WR_DUMMY_LENGTH, TPR1.CR_TPWR, TPR1.CR_IDLE_WINDOW, TPR1.CS_ACTICE_SETUP. Exists: TAG >= 1.0.4."]
pub type GpNumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CUR_GP` reader - Only used in frequency change function. Used to identify which group register data is internal used. When frqc_req and frqc_ack are both high, CUR_GP will switch to the group which in GP_NUM. Exists: TAG >= 1.0.4."]
pub type CurGpR = crate::FieldReader;
#[doc = "Field `ECC_RD_SET` reader - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is read."]
pub type EccRdSetR = crate::BitReader;
#[doc = "Field `ECC_RD_SET` writer - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is read."]
pub type EccRdSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_WR_SET` reader - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is write."]
pub type EccWrSetR = crate::BitReader;
#[doc = "Field `ECC_WR_SET` writer - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is write."]
pub type EccWrSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to enable SPIC and start user mode transaction. SPIC will reset SPIC_EN to 0 after user mode transaction is finished. SPIC_EN can only be set when BOOT_FIN=1 (boot finished) and USER_MODE=1 (user mode). (User can read SPIC_EN to check whether current user mode transaction is finished.)"]
    #[inline(always)]
    pub fn spic_en(&self) -> SpicEnR {
        SpicEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to enable ATCK_CMD implementation. After setting SPIC would not accept any command until checking Flash is not busy or checking time out. (Use in User Mode, especially for Erase/Program to check status automatically.) If CTRLR2.SEQ_SET setting, this function needs to disable."]
    #[inline(always)]
    pub fn atck_cmd(&self) -> AtckCmdR {
        AtckCmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to enable frequency change function. SPIC will chop current transaction and then set frqc_req = 1 after setting this bit. Frquency change is finished when frqc_req and frqc_ack are both high and then the choped transaction will return and continue. Write: 0: Not to activate FRQC 1: Activate FRQC Read: 0: FRQC is still in progress 1: FRQC is done Exists: TAG >= 1.0.4."]
    #[inline(always)]
    pub fn frqc(&self) -> FrqcR {
        FrqcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to trigger device enter sleep mode or deep power down mode. After setting PDEX_CTRL0, PDEX_CTRL1, PDEX_CTRL2 and TPR2, then set this bit to trigger PDEX flow. Write: 0 - Not to activate PDEX 1 - Activate PDEX Read: 0 - PDEX is still in progress 1 - PDEX is done Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn pdex(&self) -> PdexR {
        PdexR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Only used in frequency change function. Set to determine which group register user want to read or write. Base on different operation frequency, some control register setting will change. Therefore we build second group register on USER_LENGTH.USER_RD_DUMMY_LENGTH, AUTO_LENGTH.RD_DUMMY_LENGTH, AUTO_LENGTH.RDSR_DUMMY_LENGTH, AUTO_LENGTH.IN_PHYSICAL_CYC, TPR0.CS_TCEM, TPR0.CS_ACTIVE_HOLD, TPR0.CS_H_WR_DUM_LEN, TPR0.CS_H_RD_DUM_LEN, AUTO_LENGTH2.WR_DUMMY_LENGTH, TPR1.CR_TPWR, TPR1.CR_IDLE_WINDOW, TPR1.CS_ACTICE_SETUP. Exists: TAG >= 1.0.4."]
    #[inline(always)]
    pub fn gp_num(&self) -> GpNumR {
        GpNumR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Only used in frequency change function. Used to identify which group register data is internal used. When frqc_req and frqc_ack are both high, CUR_GP will switch to the group which in GP_NUM. Exists: TAG >= 1.0.4."]
    #[inline(always)]
    pub fn cur_gp(&self) -> CurGpR {
        CurGpR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is read."]
    #[inline(always)]
    pub fn ecc_rd_set(&self) -> EccRdSetR {
        EccRdSetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is write."]
    #[inline(always)]
    pub fn ecc_wr_set(&self) -> EccWrSetR {
        EccWrSetR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable SPIC and start user mode transaction. SPIC will reset SPIC_EN to 0 after user mode transaction is finished. SPIC_EN can only be set when BOOT_FIN=1 (boot finished) and USER_MODE=1 (user mode). (User can read SPIC_EN to check whether current user mode transaction is finished.)"]
    #[inline(always)]
    pub fn spic_en(&mut self) -> SpicEnW<'_, SsienrSpec> {
        SpicEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set to enable ATCK_CMD implementation. After setting SPIC would not accept any command until checking Flash is not busy or checking time out. (Use in User Mode, especially for Erase/Program to check status automatically.) If CTRLR2.SEQ_SET setting, this function needs to disable."]
    #[inline(always)]
    pub fn atck_cmd(&mut self) -> AtckCmdW<'_, SsienrSpec> {
        AtckCmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Set to enable frequency change function. SPIC will chop current transaction and then set frqc_req = 1 after setting this bit. Frquency change is finished when frqc_req and frqc_ack are both high and then the choped transaction will return and continue. Write: 0: Not to activate FRQC 1: Activate FRQC Read: 0: FRQC is still in progress 1: FRQC is done Exists: TAG >= 1.0.4."]
    #[inline(always)]
    pub fn frqc(&mut self) -> FrqcW<'_, SsienrSpec> {
        FrqcW::new(self, 2)
    }
    #[doc = "Bit 3 - Set to trigger device enter sleep mode or deep power down mode. After setting PDEX_CTRL0, PDEX_CTRL1, PDEX_CTRL2 and TPR2, then set this bit to trigger PDEX flow. Write: 0 - Not to activate PDEX 1 - Activate PDEX Read: 0 - PDEX is still in progress 1 - PDEX is done Exists: TAG >= 2.0.5."]
    #[inline(always)]
    pub fn pdex(&mut self) -> PdexW<'_, SsienrSpec> {
        PdexW::new(self, 3)
    }
    #[doc = "Bits 8:9 - Only used in frequency change function. Set to determine which group register user want to read or write. Base on different operation frequency, some control register setting will change. Therefore we build second group register on USER_LENGTH.USER_RD_DUMMY_LENGTH, AUTO_LENGTH.RD_DUMMY_LENGTH, AUTO_LENGTH.RDSR_DUMMY_LENGTH, AUTO_LENGTH.IN_PHYSICAL_CYC, TPR0.CS_TCEM, TPR0.CS_ACTIVE_HOLD, TPR0.CS_H_WR_DUM_LEN, TPR0.CS_H_RD_DUM_LEN, AUTO_LENGTH2.WR_DUMMY_LENGTH, TPR1.CR_TPWR, TPR1.CR_IDLE_WINDOW, TPR1.CS_ACTICE_SETUP. Exists: TAG >= 1.0.4."]
    #[inline(always)]
    pub fn gp_num(&mut self) -> GpNumW<'_, SsienrSpec> {
        GpNumW::new(self, 8)
    }
    #[doc = "Bit 12 - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is read."]
    #[inline(always)]
    pub fn ecc_rd_set(&mut self) -> EccRdSetW<'_, SsienrSpec> {
        EccRdSetW::new(self, 12)
    }
    #[doc = "Bit 13 - Only used in NAND ECC function. Set to determaine this nand ECC user mode flow is write."]
    #[inline(always)]
    pub fn ecc_wr_set(&mut self) -> EccWrSetW<'_, SsienrSpec> {
        EccWrSetW::new(self, 13)
    }
}
#[doc = "SPIC Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssienr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssienr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsienrSpec;
impl crate::RegisterSpec for SsienrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssienr::R`](R) reader structure"]
impl crate::Readable for SsienrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssienr::W`](W) writer structure"]
impl crate::Writable for SsienrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SSIENR to value 0"]
impl crate::Resettable for SsienrSpec {}
