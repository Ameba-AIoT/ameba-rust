#[doc = "Register `SER` reader"]
pub type R = crate::R<SerSpec>;
#[doc = "Register `SER` writer"]
pub type W = crate::W<SerSpec>;
#[doc = "Field `SER` reader - Slave Select Enable Flag. When this bit is set (1), the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register that corresponds to the slave device with which the master wants to communicate. 1: Selected 0: Not selected"]
pub type SerR = crate::BitReader;
#[doc = "Field `SER` writer - Slave Select Enable Flag. When this bit is set (1), the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register that corresponds to the slave device with which the master wants to communicate. 1: Selected 0: Not selected"]
pub type SerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Slave Select Enable Flag. When this bit is set (1), the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register that corresponds to the slave device with which the master wants to communicate. 1: Selected 0: Not selected"]
    #[inline(always)]
    pub fn ser(&self) -> SerR {
        SerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Select Enable Flag. When this bit is set (1), the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register that corresponds to the slave device with which the master wants to communicate. 1: Selected 0: Not selected"]
    #[inline(always)]
    pub fn ser(&mut self) -> SerW<'_, SerSpec> {
        SerW::new(self, 0)
    }
}
#[doc = "Slave Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerSpec;
impl crate::RegisterSpec for SerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser::R`](R) reader structure"]
impl crate::Readable for SerSpec {}
#[doc = "`write(|w| ..)` method takes [`ser::W`](W) writer structure"]
impl crate::Writable for SerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SER to value 0"]
impl crate::Resettable for SerSpec {}
