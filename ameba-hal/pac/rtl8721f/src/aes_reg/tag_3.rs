#[doc = "Register `tag_3` reader"]
pub type R = crate::R<Tag3Spec>;
#[doc = "Register `tag_3` writer"]
pub type W = crate::W<Tag3Spec>;
#[doc = "Field `tag_3` reader - Tag register bit\\[32*3+31:32*3\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
pub type Tag3R = crate::FieldReader<u32>;
#[doc = "Field `tag_3` writer - Tag register bit\\[32*3+31:32*3\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
pub type Tag3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tag register bit\\[32*3+31:32*3\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
    #[inline(always)]
    pub fn tag_3(&self) -> Tag3R {
        Tag3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tag register bit\\[32*3+31:32*3\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
    #[inline(always)]
    pub fn tag_3(&mut self) -> Tag3W<'_, Tag3Spec> {
        Tag3W::new(self, 0)
    }
}
#[doc = "Tag register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tag3Spec;
impl crate::RegisterSpec for Tag3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag_3::R`](R) reader structure"]
impl crate::Readable for Tag3Spec {}
#[doc = "`write(|w| ..)` method takes [`tag_3::W`](W) writer structure"]
impl crate::Writable for Tag3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tag_3 to value 0"]
impl crate::Resettable for Tag3Spec {}
