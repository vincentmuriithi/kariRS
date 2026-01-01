#[doc = "Register `GIFR` reader"]
pub struct R(crate::R<GIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GIFR` writer"]
pub struct W(crate::W<GIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GIFR_SPEC>;
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
impl From<crate::W<GIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GIFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCIF` reader - Pin Change Interrupt Flags"]
pub type PCIF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCIF` writer - Pin Change Interrupt Flags"]
pub type PCIF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, GIFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `INTF0` reader - External Interrupt Flag 0"]
pub type INTF0_R = crate::BitReader<bool>;
#[doc = "Field `INTF0` writer - External Interrupt Flag 0"]
pub type INTF0_W<'a, const O: u8> = crate::BitWriter<'a, u8, GIFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:5 - Pin Change Interrupt Flags"]
    #[inline(always)]
    pub fn pcif(&self) -> PCIF_R {
        PCIF_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - External Interrupt Flag 0"]
    #[inline(always)]
    pub fn intf0(&self) -> INTF0_R {
        INTF0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Pin Change Interrupt Flags"]
    #[inline(always)]
    #[must_use]
    pub fn pcif(&mut self) -> PCIF_W<4> {
        PCIF_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn intf0(&mut self) -> INTF0_W<6> {
        INTF0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Interrupt Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gifr](index.html) module"]
pub struct GIFR_SPEC;
impl crate::RegisterSpec for GIFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gifr::R](R) reader structure"]
impl crate::Readable for GIFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gifr::W](W) writer structure"]
impl crate::Writable for GIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GIFR to value 0"]
impl crate::Resettable for GIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
