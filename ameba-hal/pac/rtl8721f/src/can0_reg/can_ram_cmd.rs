#[doc = "Register `CAN_RAM_CMD` reader"]
pub type R = crate::R<CanRamCmdSpec>;
#[doc = "Register `CAN_RAM_CMD` writer"]
pub type W = crate::W<CanRamCmdSpec>;
#[doc = "Field `can_ram_acc_num` reader - access buffer number"]
pub type CanRamAccNumR = crate::FieldReader;
#[doc = "Field `can_ram_acc_num` writer - access buffer number"]
pub type CanRamAccNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_acc_mask` reader - access CAN_RAM_MASK field, the field will exchange with RAM space"]
pub type CanRamAccMaskR = crate::BitReader;
#[doc = "Field `can_ram_acc_mask` writer - access CAN_RAM_MASK field, the field will exchange with RAM space"]
pub type CanRamAccMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_cs` reader - access CAN_RAM_CS field, the field will exchange with RAM space"]
pub type CanRamAccCsR = crate::BitReader;
#[doc = "Field `can_ram_acc_cs` writer - access CAN_RAM_CS field, the field will exchange with RAM space"]
pub type CanRamAccCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_arb` reader - access CAN_RAM_ARB field, the field will exchange with RAM space"]
pub type CanRamAccArbR = crate::BitReader;
#[doc = "Field `can_ram_acc_arb` writer - access CAN_RAM_ARB field, the field will exchange with RAM space"]
pub type CanRamAccArbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data0` reader - access CAN_RAM_DATA_0 field, the field will exchange with RAM space"]
pub type CanRamAccData0R = crate::BitReader;
#[doc = "Field `can_ram_acc_data0` writer - access CAN_RAM_DATA_0 field, the field will exchange with RAM space"]
pub type CanRamAccData0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data1` reader - access CAN_RAM_DATA_1 field, the field will exchange with RAM space"]
pub type CanRamAccData1R = crate::BitReader;
#[doc = "Field `can_ram_acc_data1` writer - access CAN_RAM_DATA_1 field, the field will exchange with RAM space"]
pub type CanRamAccData1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data2` reader - access CAN_RAM_DATA_2 field, the field will exchange with RAM space"]
pub type CanRamAccData2R = crate::BitReader;
#[doc = "Field `can_ram_acc_data2` writer - access CAN_RAM_DATA_2 field, the field will exchange with RAM space"]
pub type CanRamAccData2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data3` reader - access CAN_RAM_DATA_3 field, the field will exchange with RAM space"]
pub type CanRamAccData3R = crate::BitReader;
#[doc = "Field `can_ram_acc_data3` writer - access CAN_RAM_DATA_3 field, the field will exchange with RAM space"]
pub type CanRamAccData3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data4` reader - access CAN_RAM_DATA_4 field, the field will exchange with RAM space"]
pub type CanRamAccData4R = crate::BitReader;
#[doc = "Field `can_ram_acc_data4` writer - access CAN_RAM_DATA_4 field, the field will exchange with RAM space"]
pub type CanRamAccData4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data5` reader - access CAN_RAM_DATA_5 field, the field will exchange with RAM space"]
pub type CanRamAccData5R = crate::BitReader;
#[doc = "Field `can_ram_acc_data5` writer - access CAN_RAM_DATA_5 field, the field will exchange with RAM space"]
pub type CanRamAccData5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data6` reader - access CAN_RAM_DATA_6 field, the field will exchange with RAM space"]
pub type CanRamAccData6R = crate::BitReader;
#[doc = "Field `can_ram_acc_data6` writer - access CAN_RAM_DATA_6 field, the field will exchange with RAM space"]
pub type CanRamAccData6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data7` reader - access CAN_RAM_DATA_7 field, the field will exchange with RAM space"]
pub type CanRamAccData7R = crate::BitReader;
#[doc = "Field `can_ram_acc_data7` writer - access CAN_RAM_DATA_7 field, the field will exchange with RAM space"]
pub type CanRamAccData7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data8` reader - access CAN_RAM_DATA_8 field, the field will exchange with RAM space"]
pub type CanRamAccData8R = crate::BitReader;
#[doc = "Field `can_ram_acc_data8` writer - access CAN_RAM_DATA_8 field, the field will exchange with RAM space"]
pub type CanRamAccData8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data9` reader - access CAN_RAM_DATA_9 field, the field will exchange with RAM space"]
pub type CanRamAccData9R = crate::BitReader;
#[doc = "Field `can_ram_acc_data9` writer - access CAN_RAM_DATA_9 field, the field will exchange with RAM space"]
pub type CanRamAccData9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data10` reader - access CAN_RAM_DATA_10 field, the field will exchange with RAM space"]
pub type CanRamAccData10R = crate::BitReader;
#[doc = "Field `can_ram_acc_data10` writer - access CAN_RAM_DATA_10 field, the field will exchange with RAM space"]
pub type CanRamAccData10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data11` reader - access CAN_RAM_DATA_11 field, the field will exchange with RAM space"]
pub type CanRamAccData11R = crate::BitReader;
#[doc = "Field `can_ram_acc_data11` writer - access CAN_RAM_DATA_11 field, the field will exchange with RAM space"]
pub type CanRamAccData11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data12` reader - access CAN_RAM_DATA_12 field, the field will exchange with RAM space"]
pub type CanRamAccData12R = crate::BitReader;
#[doc = "Field `can_ram_acc_data12` writer - access CAN_RAM_DATA_12 field, the field will exchange with RAM space"]
pub type CanRamAccData12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data13` reader - access CAN_RAM_DATA_13 field, the field will exchange with RAM space"]
pub type CanRamAccData13R = crate::BitReader;
#[doc = "Field `can_ram_acc_data13` writer - access CAN_RAM_DATA_13 field, the field will exchange with RAM space"]
pub type CanRamAccData13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data14` reader - access CAN_RAM_DATA_14 field, the field will exchange with RAM space"]
pub type CanRamAccData14R = crate::BitReader;
#[doc = "Field `can_ram_acc_data14` writer - access CAN_RAM_DATA_14 field, the field will exchange with RAM space"]
pub type CanRamAccData14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_acc_data15` reader - access CAN_RAM_DATA_15 field, the field will exchange with RAM space"]
pub type CanRamAccData15R = crate::BitReader;
#[doc = "Field `can_ram_acc_data15` writer - access CAN_RAM_DATA_15 field, the field will exchange with RAM space"]
pub type CanRamAccData15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_buffer_en` reader - enable the can_ram_acc_num specified message buffer for rx/tx"]
pub type CanRamBufferEnR = crate::BitReader;
#[doc = "Field `can_ram_buffer_en` writer - enable the can_ram_acc_num specified message buffer for rx/tx"]
pub type CanRamBufferEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_dir` reader - 0 is for read from RAM to CAN_RAM_* registers, 1 is for write from CAN_RAM_* to RAM"]
pub type CanRamDirR = crate::BitReader;
#[doc = "Field `can_ram_dir` writer - 0 is for read from RAM to CAN_RAM_* registers, 1 is for write from CAN_RAM_* to RAM"]
pub type CanRamDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_start` reader - start to exchange data with RAM, the bit will clear automatically after the data change is finshed."]
pub type CanRamStartR = crate::BitReader;
#[doc = "Field `can_ram_start` writer - start to exchange data with RAM, the bit will clear automatically after the data change is finshed."]
pub type CanRamStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - access buffer number"]
    #[inline(always)]
    pub fn can_ram_acc_num(&self) -> CanRamAccNumR {
        CanRamAccNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - access CAN_RAM_MASK field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_mask(&self) -> CanRamAccMaskR {
        CanRamAccMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - access CAN_RAM_CS field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_cs(&self) -> CanRamAccCsR {
        CanRamAccCsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - access CAN_RAM_ARB field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_arb(&self) -> CanRamAccArbR {
        CanRamAccArbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - access CAN_RAM_DATA_0 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data0(&self) -> CanRamAccData0R {
        CanRamAccData0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - access CAN_RAM_DATA_1 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data1(&self) -> CanRamAccData1R {
        CanRamAccData1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - access CAN_RAM_DATA_2 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data2(&self) -> CanRamAccData2R {
        CanRamAccData2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - access CAN_RAM_DATA_3 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data3(&self) -> CanRamAccData3R {
        CanRamAccData3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - access CAN_RAM_DATA_4 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data4(&self) -> CanRamAccData4R {
        CanRamAccData4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - access CAN_RAM_DATA_5 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data5(&self) -> CanRamAccData5R {
        CanRamAccData5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - access CAN_RAM_DATA_6 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data6(&self) -> CanRamAccData6R {
        CanRamAccData6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - access CAN_RAM_DATA_7 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data7(&self) -> CanRamAccData7R {
        CanRamAccData7R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - access CAN_RAM_DATA_8 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data8(&self) -> CanRamAccData8R {
        CanRamAccData8R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - access CAN_RAM_DATA_9 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data9(&self) -> CanRamAccData9R {
        CanRamAccData9R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - access CAN_RAM_DATA_10 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data10(&self) -> CanRamAccData10R {
        CanRamAccData10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - access CAN_RAM_DATA_11 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data11(&self) -> CanRamAccData11R {
        CanRamAccData11R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - access CAN_RAM_DATA_12 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data12(&self) -> CanRamAccData12R {
        CanRamAccData12R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - access CAN_RAM_DATA_13 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data13(&self) -> CanRamAccData13R {
        CanRamAccData13R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - access CAN_RAM_DATA_14 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data14(&self) -> CanRamAccData14R {
        CanRamAccData14R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - access CAN_RAM_DATA_15 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data15(&self) -> CanRamAccData15R {
        CanRamAccData15R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - enable the can_ram_acc_num specified message buffer for rx/tx"]
    #[inline(always)]
    pub fn can_ram_buffer_en(&self) -> CanRamBufferEnR {
        CanRamBufferEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0 is for read from RAM to CAN_RAM_* registers, 1 is for write from CAN_RAM_* to RAM"]
    #[inline(always)]
    pub fn can_ram_dir(&self) -> CanRamDirR {
        CanRamDirR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - start to exchange data with RAM, the bit will clear automatically after the data change is finshed."]
    #[inline(always)]
    pub fn can_ram_start(&self) -> CanRamStartR {
        CanRamStartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - access buffer number"]
    #[inline(always)]
    pub fn can_ram_acc_num(&mut self) -> CanRamAccNumW<'_, CanRamCmdSpec> {
        CanRamAccNumW::new(self, 0)
    }
    #[doc = "Bit 8 - access CAN_RAM_MASK field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_mask(&mut self) -> CanRamAccMaskW<'_, CanRamCmdSpec> {
        CanRamAccMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - access CAN_RAM_CS field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_cs(&mut self) -> CanRamAccCsW<'_, CanRamCmdSpec> {
        CanRamAccCsW::new(self, 9)
    }
    #[doc = "Bit 10 - access CAN_RAM_ARB field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_arb(&mut self) -> CanRamAccArbW<'_, CanRamCmdSpec> {
        CanRamAccArbW::new(self, 10)
    }
    #[doc = "Bit 11 - access CAN_RAM_DATA_0 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data0(&mut self) -> CanRamAccData0W<'_, CanRamCmdSpec> {
        CanRamAccData0W::new(self, 11)
    }
    #[doc = "Bit 12 - access CAN_RAM_DATA_1 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data1(&mut self) -> CanRamAccData1W<'_, CanRamCmdSpec> {
        CanRamAccData1W::new(self, 12)
    }
    #[doc = "Bit 13 - access CAN_RAM_DATA_2 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data2(&mut self) -> CanRamAccData2W<'_, CanRamCmdSpec> {
        CanRamAccData2W::new(self, 13)
    }
    #[doc = "Bit 14 - access CAN_RAM_DATA_3 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data3(&mut self) -> CanRamAccData3W<'_, CanRamCmdSpec> {
        CanRamAccData3W::new(self, 14)
    }
    #[doc = "Bit 15 - access CAN_RAM_DATA_4 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data4(&mut self) -> CanRamAccData4W<'_, CanRamCmdSpec> {
        CanRamAccData4W::new(self, 15)
    }
    #[doc = "Bit 16 - access CAN_RAM_DATA_5 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data5(&mut self) -> CanRamAccData5W<'_, CanRamCmdSpec> {
        CanRamAccData5W::new(self, 16)
    }
    #[doc = "Bit 17 - access CAN_RAM_DATA_6 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data6(&mut self) -> CanRamAccData6W<'_, CanRamCmdSpec> {
        CanRamAccData6W::new(self, 17)
    }
    #[doc = "Bit 18 - access CAN_RAM_DATA_7 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data7(&mut self) -> CanRamAccData7W<'_, CanRamCmdSpec> {
        CanRamAccData7W::new(self, 18)
    }
    #[doc = "Bit 19 - access CAN_RAM_DATA_8 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data8(&mut self) -> CanRamAccData8W<'_, CanRamCmdSpec> {
        CanRamAccData8W::new(self, 19)
    }
    #[doc = "Bit 20 - access CAN_RAM_DATA_9 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data9(&mut self) -> CanRamAccData9W<'_, CanRamCmdSpec> {
        CanRamAccData9W::new(self, 20)
    }
    #[doc = "Bit 21 - access CAN_RAM_DATA_10 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data10(&mut self) -> CanRamAccData10W<'_, CanRamCmdSpec> {
        CanRamAccData10W::new(self, 21)
    }
    #[doc = "Bit 22 - access CAN_RAM_DATA_11 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data11(&mut self) -> CanRamAccData11W<'_, CanRamCmdSpec> {
        CanRamAccData11W::new(self, 22)
    }
    #[doc = "Bit 23 - access CAN_RAM_DATA_12 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data12(&mut self) -> CanRamAccData12W<'_, CanRamCmdSpec> {
        CanRamAccData12W::new(self, 23)
    }
    #[doc = "Bit 24 - access CAN_RAM_DATA_13 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data13(&mut self) -> CanRamAccData13W<'_, CanRamCmdSpec> {
        CanRamAccData13W::new(self, 24)
    }
    #[doc = "Bit 25 - access CAN_RAM_DATA_14 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data14(&mut self) -> CanRamAccData14W<'_, CanRamCmdSpec> {
        CanRamAccData14W::new(self, 25)
    }
    #[doc = "Bit 26 - access CAN_RAM_DATA_15 field, the field will exchange with RAM space"]
    #[inline(always)]
    pub fn can_ram_acc_data15(&mut self) -> CanRamAccData15W<'_, CanRamCmdSpec> {
        CanRamAccData15W::new(self, 26)
    }
    #[doc = "Bit 29 - enable the can_ram_acc_num specified message buffer for rx/tx"]
    #[inline(always)]
    pub fn can_ram_buffer_en(&mut self) -> CanRamBufferEnW<'_, CanRamCmdSpec> {
        CanRamBufferEnW::new(self, 29)
    }
    #[doc = "Bit 30 - 0 is for read from RAM to CAN_RAM_* registers, 1 is for write from CAN_RAM_* to RAM"]
    #[inline(always)]
    pub fn can_ram_dir(&mut self) -> CanRamDirW<'_, CanRamCmdSpec> {
        CanRamDirW::new(self, 30)
    }
    #[doc = "Bit 31 - start to exchange data with RAM, the bit will clear automatically after the data change is finshed."]
    #[inline(always)]
    pub fn can_ram_start(&mut self) -> CanRamStartW<'_, CanRamCmdSpec> {
        CanRamStartW::new(self, 31)
    }
}
#[doc = "CAN message buffer access control\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamCmdSpec;
impl crate::RegisterSpec for CanRamCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_cmd::R`](R) reader structure"]
impl crate::Readable for CanRamCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_cmd::W`](W) writer structure"]
impl crate::Writable for CanRamCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_CMD to value 0"]
impl crate::Resettable for CanRamCmdSpec {}
