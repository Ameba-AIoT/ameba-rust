#[doc = "Register `PAGE_READ` reader"]
pub type R = crate::R<PageReadSpec>;
#[doc = "Register `PAGE_READ` writer"]
pub type W = crate::W<PageReadSpec>;
#[doc = "Field `PAGE_RD_CMD` reader - Indicates SPI Flash command value of page read command Available only if configuration of NAND_EN is defined."]
pub type PageRdCmdR = crate::FieldReader;
#[doc = "Field `PAGE_RD_CMD` writer - Indicates SPI Flash command value of page read command Available only if configuration of NAND_EN is defined."]
pub type PageRdCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAGE_RD_CH` reader - Indicates channel number of command and address phase in transmitting page read command. 0: single channel 1: dual channels 2: quad channels 3: octal channel Available only if configuration of NAND_EN is defined."]
pub type PageRdChR = crate::FieldReader;
#[doc = "Field `PAGE_RD_CH` writer - Indicates channel number of command and address phase in transmitting page read command. 0: single channel 1: dual channels 2: quad channels 3: octal channel Available only if configuration of NAND_EN is defined."]
pub type PageRdChW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PAGE_RD_ADDR_LEN` reader - Indicates number of bytes in address phase in page read command. Available only if configuration of NAND_EN is defined."]
pub type PageRdAddrLenR = crate::FieldReader;
#[doc = "Field `PAGE_RD_ADDR_LEN` writer - Indicates number of bytes in address phase in page read command. Available only if configuration of NAND_EN is defined."]
pub type PageRdAddrLenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of page read command Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn page_rd_cmd(&self) -> PageRdCmdR {
        PageRdCmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Indicates channel number of command and address phase in transmitting page read command. 0: single channel 1: dual channels 2: quad channels 3: octal channel Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn page_rd_ch(&self) -> PageRdChR {
        PageRdChR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Indicates number of bytes in address phase in page read command. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn page_rd_addr_len(&self) -> PageRdAddrLenR {
        PageRdAddrLenR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of page read command Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn page_rd_cmd(&mut self) -> PageRdCmdW<'_, PageReadSpec> {
        PageRdCmdW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Indicates channel number of command and address phase in transmitting page read command. 0: single channel 1: dual channels 2: quad channels 3: octal channel Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn page_rd_ch(&mut self) -> PageRdChW<'_, PageReadSpec> {
        PageRdChW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Indicates number of bytes in address phase in page read command. Available only if configuration of NAND_EN is defined."]
    #[inline(always)]
    pub fn page_rd_addr_len(&mut self) -> PageRdAddrLenW<'_, PageReadSpec> {
        PageRdAddrLenW::new(self, 18)
    }
}
#[doc = "Page Read Command of NAND Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`page_read::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`page_read::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PageReadSpec;
impl crate::RegisterSpec for PageReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`page_read::R`](R) reader structure"]
impl crate::Readable for PageReadSpec {}
#[doc = "`write(|w| ..)` method takes [`page_read::W`](W) writer structure"]
impl crate::Writable for PageReadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAGE_READ to value 0x000c_000d"]
impl crate::Resettable for PageReadSpec {
    const RESET_VALUE: u32 = 0x000c_000d;
}
