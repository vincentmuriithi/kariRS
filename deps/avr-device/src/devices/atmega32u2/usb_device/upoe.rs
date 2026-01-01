#[doc = "Register `UPOE` reader"]
pub struct R(crate::R<UPOE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPOE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPOE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPOE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPOE` writer"]
pub struct W(crate::W<UPOE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPOE_SPEC>;
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
impl From<crate::W<UPOE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPOE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMI` reader - D- Input value"]
pub type DMI_R = crate::BitReader<bool>;
#[doc = "Field `DMI` writer - D- Input value"]
pub type DMI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UPOE_SPEC, bool, O>;
#[doc = "Field `DPI` reader - D+ Input value"]
pub type DPI_R = crate::BitReader<bool>;
#[doc = "Field `DPI` writer - D+ Input value"]
pub type DPI_W<'a, const O: u8> = crate::BitWriter<'a, u8, UPOE_SPEC, bool, O>;
#[doc = "Field `UPDRV` reader - USB direct drive values"]
pub type UPDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPDRV` writer - USB direct drive values"]
pub type UPDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UPOE_SPEC, u8, u8, 2, O>;
#[doc = "Field `UPWE` reader - USB Buffers Direct Drive enable configuration"]
pub type UPWE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPWE` writer - USB Buffers Direct Drive enable configuration"]
pub type UPWE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, UPOE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - D- Input value"]
    #[inline(always)]
    pub fn dmi(&self) -> DMI_R {
        DMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D+ Input value"]
    #[inline(always)]
    pub fn dpi(&self) -> DPI_R {
        DPI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USB direct drive values"]
    #[inline(always)]
    pub fn updrv(&self) -> UPDRV_R {
        UPDRV_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - USB Buffers Direct Drive enable configuration"]
    #[inline(always)]
    pub fn upwe(&self) -> UPWE_R {
        UPWE_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - D- Input value"]
    #[inline(always)]
    #[must_use]
    pub fn dmi(&mut self) -> DMI_W<0> {
        DMI_W::new(self)
    }
    #[doc = "Bit 1 - D+ Input value"]
    #[inline(always)]
    #[must_use]
    pub fn dpi(&mut self) -> DPI_W<1> {
        DPI_W::new(self)
    }
    #[doc = "Bits 4:5 - USB direct drive values"]
    #[inline(always)]
    #[must_use]
    pub fn updrv(&mut self) -> UPDRV_W<4> {
        UPDRV_W::new(self)
    }
    #[doc = "Bits 6:7 - USB Buffers Direct Drive enable configuration"]
    #[inline(always)]
    #[must_use]
    pub fn upwe(&mut self) -> UPWE_W<6> {
        UPWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Software Output Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upoe](index.html) module"]
pub struct UPOE_SPEC;
impl crate::RegisterSpec for UPOE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [upoe::R](R) reader structure"]
impl crate::Readable for UPOE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [upoe::W](W) writer structure"]
impl crate::Writable for UPOE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPOE to value 0"]
impl crate::Resettable for UPOE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
