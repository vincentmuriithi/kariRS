#[doc = "Register `CLKPR` reader"]
pub struct R(crate::R<CLKPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKPR` writer"]
pub struct W(crate::W<CLKPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPR_SPEC>;
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
impl From<crate::W<CLKPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPS` reader - No Description."]
pub type CLKPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKPS` writer - No Description."]
pub type CLKPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CLKPR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CLKPCE` reader - No Description."]
pub type CLKPCE_R = crate::BitReader<bool>;
#[doc = "Field `CLKPCE` writer - No Description."]
pub type CLKPCE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLKPR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    pub fn clkps(&self) -> CLKPS_R {
        CLKPS_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    pub fn clkpce(&self) -> CLKPCE_R {
        CLKPCE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clkps(&mut self) -> CLKPS_W<0> {
        CLKPS_W::new(self)
    }
    #[doc = "Bit 7 - No Description."]
    #[inline(always)]
    #[must_use]
    pub fn clkpce(&mut self) -> CLKPCE_W<7> {
        CLKPCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpr](index.html) module"]
pub struct CLKPR_SPEC;
impl crate::RegisterSpec for CLKPR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clkpr::R](R) reader structure"]
impl crate::Readable for CLKPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpr::W](W) writer structure"]
impl crate::Writable for CLKPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKPR to value 0"]
impl crate::Resettable for CLKPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
