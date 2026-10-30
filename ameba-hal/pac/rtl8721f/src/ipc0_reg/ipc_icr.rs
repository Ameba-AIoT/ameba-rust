#[doc = "Register `IPC_ICR` reader"]
pub type R = crate::R<IpcIcrSpec>;
#[doc = "Register `IPC_ICR` writer"]
pub type W = crate::W<IpcIcrSpec>;
#[doc = "Field `rsvd0` reader - "]
pub type Rsvd0R = crate::FieldReader<u16>;
#[doc = "Field `rsvd0` writer - "]
pub type Rsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `icr_tx0_data_clear0` reader - 0: Writing 0 has no effect. 1: Clear the Tx data register (tx0_data) channel 0 bit."]
pub type IcrTx0DataClear0R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear1` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear1R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear2` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear2R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear3` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear3R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear4` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear4R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear5` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear5R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear6` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear6R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear7` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear7R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear8` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear8R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear9` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear9R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear10` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear10R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear11` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear11R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear12` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear12R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear13` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear13R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear14` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear14R = crate::BitReader;
#[doc = "Field `icr_tx0_data_clear15` reader - Refer to the description of the icr_tx0_data_clear0."]
pub type IcrTx0DataClear15R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 0: Writing 0 has no effect. 1: Clear the Tx data register (tx0_data) channel 0 bit."]
    #[inline(always)]
    pub fn icr_tx0_data_clear0(&self) -> IcrTx0DataClear0R {
        IcrTx0DataClear0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear1(&self) -> IcrTx0DataClear1R {
        IcrTx0DataClear1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear2(&self) -> IcrTx0DataClear2R {
        IcrTx0DataClear2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear3(&self) -> IcrTx0DataClear3R {
        IcrTx0DataClear3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear4(&self) -> IcrTx0DataClear4R {
        IcrTx0DataClear4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear5(&self) -> IcrTx0DataClear5R {
        IcrTx0DataClear5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear6(&self) -> IcrTx0DataClear6R {
        IcrTx0DataClear6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear7(&self) -> IcrTx0DataClear7R {
        IcrTx0DataClear7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear8(&self) -> IcrTx0DataClear8R {
        IcrTx0DataClear8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear9(&self) -> IcrTx0DataClear9R {
        IcrTx0DataClear9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear10(&self) -> IcrTx0DataClear10R {
        IcrTx0DataClear10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear11(&self) -> IcrTx0DataClear11R {
        IcrTx0DataClear11R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear12(&self) -> IcrTx0DataClear12R {
        IcrTx0DataClear12R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear13(&self) -> IcrTx0DataClear13R {
        IcrTx0DataClear13R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear14(&self) -> IcrTx0DataClear14R {
        IcrTx0DataClear14R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Refer to the description of the icr_tx0_data_clear0."]
    #[inline(always)]
    pub fn icr_tx0_data_clear15(&self) -> IcrTx0DataClear15R {
        IcrTx0DataClear15R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rsvd0(&mut self) -> Rsvd0W<'_, IpcIcrSpec> {
        Rsvd0W::new(self, 0)
    }
}
#[doc = "Clear Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcIcrSpec;
impl crate::RegisterSpec for IpcIcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc_icr::R`](R) reader structure"]
impl crate::Readable for IpcIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc_icr::W`](W) writer structure"]
impl crate::Writable for IpcIcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC_ICR to value 0"]
impl crate::Resettable for IpcIcrSpec {}
