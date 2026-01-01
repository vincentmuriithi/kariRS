#[doc = "Register `OSC20MCALIBA` reader"]
pub struct R(crate::R<OSC20MCALIBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC20MCALIBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC20MCALIBA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC20MCALIBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC20MCALIBA` writer"]
pub struct W(crate::W<OSC20MCALIBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC20MCALIBA_SPEC>;
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
impl From<crate::W<OSC20MCALIBA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC20MCALIBA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL20M` reader - Calibration"]
pub type CAL20M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL20M` writer - Calibration"]
pub type CAL20M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, OSC20MCALIBA_SPEC, u8, u8, 6, O>;
#[doc = "Field `CALSEL20M` reader - Calibration freq select"]
pub type CALSEL20M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALSEL20M` writer - Calibration freq select"]
pub type CALSEL20M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, OSC20MCALIBA_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration"]
    #[inline(always)]
    pub fn cal20m(&self) -> CAL20M_R {
        CAL20M_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Calibration freq select"]
    #[inline(always)]
    pub fn calsel20m(&self) -> CALSEL20M_R {
        CALSEL20M_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal20m(&mut self) -> CAL20M_W<0> {
        CAL20M_W::new(self)
    }
    #[doc = "Bits 6:7 - Calibration freq select"]
    #[inline(always)]
    #[must_use]
    pub fn calsel20m(&mut self) -> CALSEL20M_W<6> {
        CALSEL20M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC20M Calibration A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc20mcaliba](index.html) module"]
pub struct OSC20MCALIBA_SPEC;
impl crate::RegisterSpec for OSC20MCALIBA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc20mcaliba::R](R) reader structure"]
impl crate::Readable for OSC20MCALIBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc20mcaliba::W](W) writer structure"]
impl crate::Writable for OSC20MCALIBA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC20MCALIBA to value 0"]
impl crate::Resettable for OSC20MCALIBA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
