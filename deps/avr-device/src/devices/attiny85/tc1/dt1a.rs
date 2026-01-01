#[doc = "Register `DT1A` reader"]
pub struct R(crate::R<DT1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT1A` writer"]
pub struct W(crate::W<DT1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT1A_SPEC>;
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
impl From<crate::W<DT1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTVL` reader - No Description."]
pub type DTVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTVL` writer - No Description."]
pub type DTVL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DT1A_SPEC, u8, u8, 4, O>;
#[doc = "Field `DTVH` reader - No Description."]
pub type DTVH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTVH` writer - No Description."]
pub type DTVH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DT1A_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn dtvl(&self) -> DTVL_R {
        DTVL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    pub fn dtvh(&self) -> DTVH_R {
        DTVH_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dtvl(&mut self) -> DTVL_W<0> {
        DTVL_W::new(self)
    }
    #[doc = "Bits 4:7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn dtvh(&mut self) -> DTVH_W<4> {
        DTVH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dead Time Value Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt1a](index.html) module"]
pub struct DT1A_SPEC;
impl crate::RegisterSpec for DT1A_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dt1a::R](R) reader structure"]
impl crate::Readable for DT1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt1a::W](W) writer structure"]
impl crate::Writable for DT1A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT1A to value 0"]
impl crate::Resettable for DT1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
