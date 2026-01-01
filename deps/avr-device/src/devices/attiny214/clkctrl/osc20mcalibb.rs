#[doc = "Register `OSC20MCALIBB` reader"]
pub struct R(crate::R<OSC20MCALIBB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC20MCALIBB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC20MCALIBB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC20MCALIBB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC20MCALIBB` writer"]
pub struct W(crate::W<OSC20MCALIBB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC20MCALIBB_SPEC>;
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
impl From<crate::W<OSC20MCALIBB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC20MCALIBB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMPCAL20M` reader - Oscillator temperature coefficient"]
pub type TEMPCAL20M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMPCAL20M` writer - Oscillator temperature coefficient"]
pub type TEMPCAL20M_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, OSC20MCALIBB_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u8, OSC20MCALIBB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Oscillator temperature coefficient"]
    #[inline(always)]
    pub fn tempcal20m(&self) -> TEMPCAL20M_R {
        TEMPCAL20M_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oscillator temperature coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn tempcal20m(&mut self) -> TEMPCAL20M_W<0> {
        TEMPCAL20M_W::new(self)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<7> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC20M Calibration B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc20mcalibb](index.html) module"]
pub struct OSC20MCALIBB_SPEC;
impl crate::RegisterSpec for OSC20MCALIBB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc20mcalibb::R](R) reader structure"]
impl crate::Readable for OSC20MCALIBB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc20mcalibb::W](W) writer structure"]
impl crate::Writable for OSC20MCALIBB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC20MCALIBB to value 0"]
impl crate::Resettable for OSC20MCALIBB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
