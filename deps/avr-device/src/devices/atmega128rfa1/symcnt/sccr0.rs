#[doc = "Register `SCCR0` reader"]
pub struct R(crate::R<SCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCCR0` writer"]
pub struct W(crate::W<SCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCCR0_SPEC>;
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
impl From<crate::W<SCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCCMP` reader - Symbol Counter Compare Unit 3 Mode select"]
pub type SCCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCCMP` writer - Symbol Counter Compare Unit 3 Mode select"]
pub type SCCMP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCCR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SCTSE` reader - Symbol Counter Automatic Timestamping enable"]
pub type SCTSE_R = crate::BitReader<bool>;
#[doc = "Field `SCTSE` writer - Symbol Counter Automatic Timestamping enable"]
pub type SCTSE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCCR0_SPEC, bool, O>;
#[doc = "Field `SCCKSEL` reader - Symbol Counter Clock Source select"]
pub type SCCKSEL_R = crate::BitReader<bool>;
#[doc = "Field `SCCKSEL` writer - Symbol Counter Clock Source select"]
pub type SCCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCCR0_SPEC, bool, O>;
#[doc = "Field `SCEN` reader - Symbol Counter enable"]
pub type SCEN_R = crate::BitReader<bool>;
#[doc = "Field `SCEN` writer - Symbol Counter enable"]
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCCR0_SPEC, bool, O>;
#[doc = "Field `SCMBTS` reader - Manual Beacon Timestamp"]
pub type SCMBTS_R = crate::BitReader<bool>;
#[doc = "Field `SCMBTS` writer - Manual Beacon Timestamp"]
pub type SCMBTS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCCR0_SPEC, bool, O>;
#[doc = "Field `SCRES` reader - Symbol Counter Synchronization"]
pub type SCRES_R = crate::BitReader<bool>;
#[doc = "Field `SCRES` writer - Symbol Counter Synchronization"]
pub type SCRES_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCCR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Symbol Counter Compare Unit 3 Mode select"]
    #[inline(always)]
    pub fn sccmp(&self) -> SCCMP_R {
        SCCMP_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Symbol Counter Automatic Timestamping enable"]
    #[inline(always)]
    pub fn sctse(&self) -> SCTSE_R {
        SCTSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Symbol Counter Clock Source select"]
    #[inline(always)]
    pub fn sccksel(&self) -> SCCKSEL_R {
        SCCKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Symbol Counter enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Manual Beacon Timestamp"]
    #[inline(always)]
    pub fn scmbts(&self) -> SCMBTS_R {
        SCMBTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Symbol Counter Synchronization"]
    #[inline(always)]
    pub fn scres(&self) -> SCRES_R {
        SCRES_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Symbol Counter Compare Unit 3 Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn sccmp(&mut self) -> SCCMP_W<0> {
        SCCMP_W::new(self)
    }
    #[doc = "Bit 3 - Symbol Counter Automatic Timestamping enable"]
    #[inline(always)]
    #[must_use]
    pub fn sctse(&mut self) -> SCTSE_W<3> {
        SCTSE_W::new(self)
    }
    #[doc = "Bit 4 - Symbol Counter Clock Source select"]
    #[inline(always)]
    #[must_use]
    pub fn sccksel(&mut self) -> SCCKSEL_W<4> {
        SCCKSEL_W::new(self)
    }
    #[doc = "Bit 5 - Symbol Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<5> {
        SCEN_W::new(self)
    }
    #[doc = "Bit 6 - Manual Beacon Timestamp"]
    #[inline(always)]
    #[must_use]
    pub fn scmbts(&mut self) -> SCMBTS_W<6> {
        SCMBTS_W::new(self)
    }
    #[doc = "Bit 7 - Symbol Counter Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn scres(&mut self) -> SCRES_W<7> {
        SCRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Symbol Counter Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sccr0](index.html) module"]
pub struct SCCR0_SPEC;
impl crate::RegisterSpec for SCCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sccr0::R](R) reader structure"]
impl crate::Readable for SCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sccr0::W](W) writer structure"]
impl crate::Writable for SCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCCR0 to value 0"]
impl crate::Resettable for SCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
