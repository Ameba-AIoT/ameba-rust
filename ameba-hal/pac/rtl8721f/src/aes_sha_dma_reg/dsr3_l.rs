#[doc = "Register `DSR3_L` reader"]
pub type R = crate::R<Dsr3LSpec>;
#[doc = "Register `DSR3_L` writer"]
pub type W = crate::W<Dsr3LSpec>;
#[doc = "Field `DSR3_L_DSI` reader - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr3LDsiR = crate::FieldReader<u32>;
#[doc = "Field `DSR3_L_DSI` writer - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr3LDsiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DSR3_L_DSC` reader - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr3LDscR = crate::FieldReader<u16>;
#[doc = "Field `DSR3_L_DSC` writer - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr3LDscW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr3_l_dsi(&self) -> Dsr3LDsiR {
        Dsr3LDsiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr3_l_dsc(&self) -> Dsr3LDscR {
        Dsr3LDscR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr3_l_dsi(&mut self) -> Dsr3LDsiW<'_, Dsr3LSpec> {
        Dsr3LDsiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr3_l_dsc(&mut self) -> Dsr3LDscW<'_, Dsr3LSpec> {
        Dsr3LDscW::new(self, 20)
    }
}
#[doc = "Channel 3 low-32 bits Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr3_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr3_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr3LSpec;
impl crate::RegisterSpec for Dsr3LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr3_l::R`](R) reader structure"]
impl crate::Readable for Dsr3LSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr3_l::W`](W) writer structure"]
impl crate::Writable for Dsr3LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSR3_L to value 0"]
impl crate::Resettable for Dsr3LSpec {}
