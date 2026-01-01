#[doc = "Register `IEEE_ADDR_2` reader"]
pub struct R(crate::R<IEEE_ADDR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEEE_ADDR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEEE_ADDR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEEE_ADDR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEEE_ADDR_2` writer"]
pub struct W(crate::W<IEEE_ADDR_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEEE_ADDR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IEEE_ADDR_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEEE_ADDR_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEEE_ADDR_` reader - MAC IEEE Address"]
pub type IEEE_ADDR__R = crate::FieldReader<u8, u8>;
#[doc = "Field `IEEE_ADDR_` writer - MAC IEEE Address"]
pub type IEEE_ADDR__W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, IEEE_ADDR_2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MAC IEEE Address"]
    #[inline(always)]
    pub fn ieee_addr_(&self) -> IEEE_ADDR__R {
        IEEE_ADDR__R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAC IEEE Address"]
    #[inline(always)]
    #[must_use]
    pub fn ieee_addr_(&mut self) -> IEEE_ADDR__W<0> {
        IEEE_ADDR__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Transceiver MAC IEEE Address Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_addr_2](index.html) module"]
pub struct IEEE_ADDR_2_SPEC;
impl crate::RegisterSpec for IEEE_ADDR_2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ieee_addr_2::R](R) reader structure"]
impl crate::Readable for IEEE_ADDR_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ieee_addr_2::W](W) writer structure"]
impl crate::Writable for IEEE_ADDR_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEEE_ADDR_2 to value 0"]
impl crate::Resettable for IEEE_ADDR_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
